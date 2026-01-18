use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{env, fs::File};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from ./bot.yaml, /etc/config/bot.yaml, or from env BOT_CONFIG
        let ret = match (
            File::open("bot.yaml"),
            File::open("/etc/config/bot.yaml"),
            env::var("BOT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Bot config file not found"),
        };
        Ok(ret?)
    }
}
