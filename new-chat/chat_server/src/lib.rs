mod agent;
mod config;
mod error;
mod handlers;
mod middlewares;
mod models;
mod openapi;
mod redis;

use crate::{
    config::SigninRateLimit,
    handlers::*,
    middlewares::{rate_limit_signin, verify_chat},
    openapi::OpenApiRouter,
    redis::RedisPool,
};
use anyhow::Context;
use axum::{
    Router,
    http::Method,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use chat_core::{
    DecodingKey, EncodingKey, User,
    middlewares::{TokenVerify, set_layers, verify_token},
};
use sqlx::PgPool;
use std::{fmt, ops::Deref, sync::Arc};
use tokio::fs;
use tower_http::cors::{Any, CorsLayer};

pub use config::AppConfig;
pub use error::AppError;

#[derive(Clone, Debug)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

/// Rate limit state for signin endpoint
#[derive(Clone)]
pub struct RateLimitState {
    pub config: SigninRateLimit,
    pub redis: RedisPool,
}

impl RateLimitState {
    pub fn new(config: SigninRateLimit, redis: RedisPool) -> Self {
        Self { config, redis }
    }
}

pub struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
    pub(crate) pool: PgPool,
    #[allow(dead_code)]
    pub(crate) redis: Option<RedisPool>,
    pub(crate) rate_limit_state: Option<RateLimitState>,
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let chat = Router::new()
        .route(
            "/{id}",
            get(get_chat_handler)
                .patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route(
            "/{id}/agents",
            get(list_agent_handler)
                .post(create_agent_handler)
                .patch(update_agent_handler),
        )
        .route(
            "/{id}/agents/{agent_id}",
            axum::routing::delete(delete_agent_handler),
        )
        .route("/{id}/messages", get(list_message_handler))
        .route(
            "/{id}/messages/{message_id}",
            axum::routing::delete(delete_message_handler),
        )
        .route("/{id}/members", post(add_members_handler))
        .route(
            "/{id}/members/{member_id}",
            axum::routing::delete(remove_member_handler),
        )
        .layer(from_fn_with_state(state.clone(), verify_chat))
        .route("/", get(list_chat_handler).post(create_chat_handler));

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    // Build the signin route with optional rate limiting
    let signin_route = post(signin_handler);
    let signin_route = if state.rate_limit_state.is_some() {
        signin_route.layer(from_fn_with_state(state.inner.clone(), rate_limit_signin))
    } else {
        signin_route
    };

    let api = Router::new()
        .route("/users", get(list_chat_users_handler))
        .nest("/chats", chat)
        .route("/upload", post(upload_handler))
        .route("/files/{ws_id}/{*path}", get(file_handler))
        .route("/change-password", post(change_password_handler))
        .route(
            "/workspaces/invitations",
            get(list_invitations_handler).post(create_invitation_handler),
        )
        .route(
            "/workspaces/invitations/{id}",
            axum::routing::delete(deactivate_invitation_handler),
        )
        .route("/workspaces/join", post(join_workspace_handler))
        .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
        // routes doesn't need token verification
        .route("/signin", signin_route)
        .route("/signup", post(signup_handler))
        .route("/refresh", post(refresh_handler))
        .layer(cors);

    let app = Router::new().openapi().nest("/api", api).with_state(state);

    Ok(set_layers(app))
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &AppStateInner {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir)
            .await
            .context("create base_dir failed")?;
        let pool = PgPool::connect(&config.server.db_url).await?;
        let ek = EncodingKey::load(&config.auth.sk).context("load pk failed")?;
        let dk = DecodingKey::load(&config.auth.pk).context("load sk failed")?;

        // Initialize Redis if configured
        let redis = if let Some(ref redis_config) = config.redis {
            Some(RedisPool::new(redis_config).await?)
        } else {
            tracing::warn!("Redis not configured, rate limiting disabled");
            None
        };

        // Initialize rate limit state if Redis and config are available
        let rate_limit_state = match (&redis, &config.rate_limit) {
            (Some(redis), Some(rate_limit_config)) => Some(RateLimitState::new(
                rate_limit_config.signin.clone(),
                redis.clone(),
            )),
            _ => None,
        };

        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
                redis,
                rate_limit_state,
            }),
        })
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.dk.verify_access(token)?)
    }
}

#[cfg(feature = "test-util")]
mod test_util {
    use super::*;
    use sqlx::Executor;
    use sqlx_db_tester::TestPg;
    use std::path::Path;

    impl AppState {
        pub async fn new_for_test() -> Result<(sqlx_db_tester::TestPg, Self), AppError> {
            let config = AppConfig::load()?;
            let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
            let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
            let post = config.server.db_url.rfind('/').expect("invalid db_url");
            let server_url = &config.server.db_url[..post];

            let (tdb, pool) = get_test_pool(Some(server_url)).await;

            // For tests, Redis is optional - skip if not configured
            let redis = if let Some(ref redis_config) = config.redis {
                Some(RedisPool::new(redis_config).await?)
            } else {
                None
            };

            let rate_limit_state = match (&redis, &config.rate_limit) {
                (Some(redis), Some(rate_limit_config)) => Some(RateLimitState::new(
                    rate_limit_config.signin.clone(),
                    redis.clone(),
                )),
                _ => None,
            };

            let state = Self {
                inner: Arc::new(AppStateInner {
                    config,
                    ek,
                    dk,
                    pool,
                    redis,
                    rate_limit_state,
                }),
            };
            Ok((tdb, state))
        }
    }

    pub async fn get_test_pool(url: Option<&str>) -> (TestPg, PgPool) {
        let url = match url {
            Some(url) => url.to_string(),
            None => "postgres://postgres:postgres@localhost:5433".to_string(),
        };
        let tdb = TestPg::new(url, Path::new("../migrations"));
        let pool = tdb.get_pool().await;

        // run prepared sql to insert test data
        let sql = include_str!("../fixtures/test.sql").split(';');
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");

        (tdb, pool)
    }
}
