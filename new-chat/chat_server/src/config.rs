use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
    #[serde(default)]
    pub redis: Option<RedisConfig>,
    #[serde(default)]
    pub rate_limit: Option<RateLimitConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub sk: String,
    pub pk: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
    pub base_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    #[serde(default = "default_pool_size")]
    pub pool_size: usize,
}

fn default_pool_size() -> usize {
    16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub signin: SigninRateLimit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SigninRateLimit {
    #[serde(default = "default_max_attempts_ip")]
    pub max_attempts_ip: usize,
    #[serde(default = "default_max_attempts_email")]
    pub max_attempts_email: usize,
    #[serde(default = "default_max_attempts_ip_email")]
    pub max_attempts_ip_email: usize,
    #[serde(default = "default_window_secs")]
    pub window_secs: u64,
}

fn default_max_attempts_ip() -> usize {
    10
}

fn default_max_attempts_email() -> usize {
    5
}

fn default_max_attempts_ip_email() -> usize {
    3
}

fn default_window_secs() -> u64 {
    60
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from ./app.yaml, /etc/config/app.yaml, or from env CHAT_CONFIG
        let ret = match (
            File::open("chat.yaml"),
            File::open("/etc/config/chat.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Chat config file not found"),
        };
        Ok(ret?)
    }
}
