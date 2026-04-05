use crate::{AppError, config::RedisConfig};

use deadpool_redis::{Config, Pool, Runtime};

#[derive(Clone)]
pub struct RedisPool(pub Pool);

impl RedisPool {
    pub async fn new(config: &RedisConfig) -> Result<Self, AppError> {
        let cfg = Config::from_url(&config.url);
        let pool = cfg
            .builder()
            .map_err(|e| {
                AppError::RedisBuildError(format!("failed to create redis pool builder: {}", e))
            })?
            .max_size(config.pool_size)
            .runtime(Runtime::Tokio1)
            .build()
            .map_err(|e| AppError::RedisBuildError(format!("failed to build redis pool: {}", e)))?;
        Ok(Self(pool))
    }
}
