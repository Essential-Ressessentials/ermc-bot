use std::{sync::Arc, time::Duration};

use poise::serenity_prelude::{ActivityData, OnlineStatus};
use serde::Deserialize;
use tokio::time;

use crate::{context::TaskCtx, error::BotResult};

#[derive(Deserialize)]
struct ServerStats {
	players: Players,
}

#[derive(Deserialize)]
struct Players {
	online: u32,
	#[serde(rename = "max")]
	_max: u32,
}

pub async fn start_status(ctx: Arc<TaskCtx>) -> BotResult {
	let client = &ctx.data.http_client;
	let config = &ctx.data.config;

	let ip = &config.minecraft.ip;
	let port = config.minecraft.port;
	let url = format!("https://api.mcquery.net/java?target={ip}&port={port}");

	let mut interval = time::interval(Duration::from_secs(30));

	loop {
		interval.tick().await;

		let Ok(res) = client
			.get(&url)
			.header("User-Agent", format!("ermc-bot/{}", env!("CARGO_PKG_VERSION")))
			.send()
			.await
		else {
			eprintln!("Failed to retrieve server stats");
			continue;
		};

		let Ok(stats) = res
			.json::<ServerStats>()
			.await
		else {
			eprintln!("Failed to deserialize JSON");
			continue;
		};

		let online = stats.players.online;

		let status = format!(
			"{} Player{} Online",
			online,
			if online == 1 {
				""
			} else {
				"s"
			}
		);
		ctx.shard
			.set_presence(Some(ActivityData::custom(status)), OnlineStatus::Online);
	}
}
