use crate::{AppStateInner, error::AppError, models::SigninUser};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use deadpool_redis::Pool;
use std::sync::Arc;

/// Extract client IP from request, checking X-Forwarded-For header first
fn extract_client_ip(request: &Request<Body>) -> String {
    // Try X-Forwarded-For header first (for reverse proxy setups)
    if let Some(header) = request.headers().get("x-forwarded-for")
        && let Ok(value) = header.to_str()
        && let Some(ip) = value.split(',').next()
    {
        return ip.trim().to_string();
    }

    // Fall back to X-Real-IP header
    if let Some(header) = request.headers().get("x-real-ip")
        && let Ok(value) = header.to_str()
    {
        return value.trim().to_string();
    }

    // Fall back to Host header
    if let Some(header) = request.headers().get("host")
        && let Ok(value) = header.to_str()
    {
        return value.trim().to_string();
    }

    // Default fallback
    "unknown".to_string()
}

/// Lua script for atomic sliding window rate limiting
/// Returns: (is_allowed, current_count, ttl_seconds)
const RATE_LIMIT_LUA_SCRIPT: &str = r#"
local key = KEYS[1]
local now = tonumber(ARGV[1])
local window = tonumber(ARGV[2])
local max_requests = tonumber(ARGV[3])

-- Remove expired entries (outside the window)
local cutoff = now - window
redis.call('ZREMRANGEBYSCORE', key, '-inf', cutoff)

-- Count current requests in window
local current = redis.call('ZCARD', key)

if current < max_requests then
    -- Add new request with timestamp as score
    redis.call('ZADD', key, now, now .. ':' .. math.random())
    -- Set expiry on the key
    redis.call('EXPIRE', key, window)
    current = current + 1
end

-- Get TTL for the key
local ttl = redis.call('TTL', key)
if ttl < 0 then
    ttl = window
end

return {1, current, ttl}
"#;

#[derive(Debug, Clone)]
struct RateLimitResult {
    allowed: bool,
    #[allow(dead_code)]
    current: usize,
    limit: usize,
    remaining: usize,
    reset_in: u64,
}

/// Check rate limit for a specific key
async fn check_rate_limit(
    pool: &Pool,
    key: &str,
    max_requests: usize,
    window_secs: u64,
) -> Result<RateLimitResult, AppError> {
    let mut conn = pool.get().await?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let script = redis::Script::new(RATE_LIMIT_LUA_SCRIPT);
    let result: Vec<u64> = script
        .key(key)
        .arg(now)
        .arg(window_secs)
        .arg(max_requests)
        .invoke_async(&mut conn)
        .await
        .map_err(|e| anyhow::anyhow!("rate limit script failed: {}", e))?;

    let is_allowed = result[0] == 1;
    let current = result[1] as usize;
    let ttl = result[2];

    let remaining = max_requests.saturating_sub(current);

    Ok(RateLimitResult {
        allowed: is_allowed,
        current,
        limit: max_requests,
        remaining,
        reset_in: ttl,
    })
}

/// Rate limit middleware for signin endpoint
pub async fn rate_limit_signin(
    State(state): State<Arc<AppStateInner>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Get rate limit config from state
    let rate_limit_state = state
        .rate_limit_state
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("rate limit not configured"))?;

    let config = &rate_limit_state.config;
    let redis = &rate_limit_state.redis;

    // Extract client IP
    let client_ip = extract_client_ip(&request);

    // Extract headers and extensions before consuming the request
    let headers = request.headers().clone();
    let extensions = request.extensions().clone();

    // We need to read and restore the body to extract email
    let (parts, body) = request.into_parts();
    let body_bytes = axum::body::to_bytes(body, 10 * 1024)
        .await
        .map_err(|e| anyhow::anyhow!("failed to read body: {}", e))?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    // Try to parse email from request body
    let email = if let Ok(signin) = serde_json::from_str::<SigninUser>(&body_str) {
        Some(signin.email)
    } else {
        None
    };

    // Reconstruct the request with the same headers, extensions and new body
    let request_body = Body::from(body_bytes.to_vec());
    let mut new_parts = parts;
    new_parts.headers = headers;
    new_parts.extensions = extensions;
    let request = Request::from_parts(new_parts, request_body);

    // Check rate limits in order of specificity
    // 1. IP + Email combination (most specific)
    // 2. Email alone
    // 3. IP alone (least specific)

    // Check IP + Email rate limit
    if let Some(ref email) = email {
        let key = format!("ratelimit:signin:ip_email:{}:{}", client_ip, email);
        let result = check_rate_limit(
            &redis.0,
            &key,
            config.max_attempts_ip_email,
            config.window_secs,
        )
        .await?;

        if !result.allowed {
            return Err(AppError::RateLimitExceeded(format!(
                "too many attempts for this email from this IP, retry after {} seconds",
                result.reset_in
            )));
        }
    }

    // Check Email rate limit
    if let Some(ref email) = email {
        let key = format!("ratelimit:signin:email:{}", email);
        let result = check_rate_limit(
            &redis.0,
            &key,
            config.max_attempts_email,
            config.window_secs,
        )
        .await?;

        if !result.allowed {
            return Err(AppError::RateLimitExceeded(format!(
                "too many login attempts for this email, retry after {} seconds",
                result.reset_in
            )));
        }
    }

    // Check IP rate limit
    let key = format!("ratelimit:signin:ip:{}", client_ip);
    let result =
        check_rate_limit(&redis.0, &key, config.max_attempts_ip, config.window_secs).await?;

    if !result.allowed {
        return Err(AppError::RateLimitExceeded(format!(
            "too many login attempts from this IP, retry after {} seconds",
            result.reset_in
        )));
    }

    // Continue to the next middleware/handler
    let response = next.run(request).await;

    tracing::debug!(
        "Rate limit check passed: {}/{} remaining, resets in {}s",
        result.remaining,
        result.limit,
        result.reset_in
    );

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;

    #[test]
    fn test_extract_client_ip_from_x_forwarded_for() {
        let request = HttpRequest::builder()
            .header("x-forwarded-for", "192.168.1.1, 10.0.0.1")
            .body(Body::empty())
            .unwrap();

        let ip = extract_client_ip(&request);
        assert_eq!(ip, "192.168.1.1");
    }

    #[test]
    fn test_extract_client_ip_from_x_real_ip() {
        let request = HttpRequest::builder()
            .header("x-real-ip", "192.168.1.100")
            .body(Body::empty())
            .unwrap();

        let ip = extract_client_ip(&request);
        assert_eq!(ip, "192.168.1.100");
    }

    #[test]
    fn test_extract_client_ip_from_host() {
        let request = HttpRequest::builder()
            .header("host", "example.com")
            .body(Body::empty())
            .unwrap();

        let ip = extract_client_ip(&request);
        assert_eq!(ip, "example.com");
    }

    #[test]
    fn test_extract_client_ip_default() {
        let request = HttpRequest::builder().body(Body::empty()).unwrap();

        let ip = extract_client_ip(&request);
        assert_eq!(ip, "unknown");
    }
}
