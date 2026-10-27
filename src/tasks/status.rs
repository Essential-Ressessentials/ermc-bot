use std::{sync::Arc, time::Duration};

use anyhow::Context;
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

	let server_ip = &config.minecraft.server_ip;
	let url = format!("https://api.mcsrvstat.us/3/{server_ip}");

	let mut interval = time::interval(Duration::from_secs(10));

	loop {
		interval.tick().await;

		let res = client
			.get(&url)
			.header("User-Agent", format!("ermc-bot/{}", env!("CARGO_PKG_VERSION")))
			.send()
			.await
			.context("Failed to retrieve server stats")?;

		let stats = res
			.json::<ServerStats>()
			.await
			.context("Failed to deserialize JSON")?;
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
