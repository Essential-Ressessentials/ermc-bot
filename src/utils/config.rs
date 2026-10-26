use std::fs;

use anyhow::Context;
use serde::Deserialize;

use crate::error::BotResult;

const CONFIG_PATH: &str = "ermc/config.toml";

#[derive(Deserialize)]
pub struct Config {
	pub discord: DiscordConfig,
	pub redis: RedisConfig,
	pub rest: RESTConfig,
}

#[derive(Deserialize)]
pub struct DiscordConfig {
	pub token: String,
	pub webhook_url: String,
	pub owner_id: u64,
	pub essress_id: u64,
	pub mc_chat_id: u64,
}

#[derive(Deserialize)]
pub struct RedisConfig {
	pub ip: String,
	pub port: u32,
}

#[derive(Deserialize)]
pub struct RESTConfig {
	pub ip: String,
	pub port: u32,
}

impl Config {
	pub fn load() -> BotResult<Self> {
		let config_str =
			fs::read_to_string(CONFIG_PATH).context(format!("Failed to read {}", CONFIG_PATH))?;
		let config: Config =
			toml::from_str(&config_str).context(format!("Failed to parse {}", CONFIG_PATH))?;

		Ok(config)
	}
}
