use std::sync::Arc;

use anyhow::Context;
use poise::serenity_prelude::{Context as Ctx, Message};
use serde::Serialize;

use crate::{data::Data, error::BotResult, services::redis};

#[derive(Serialize)]
struct Msg {
	name: String,
	msg: String,
}

pub async fn on_message(_ctx: &Ctx, data: &Arc<Data>, message: &Message) -> BotResult {
	if message.channel_id != data.config.discord.mc_chat_id {
		return Ok(());
	}

	let author = &message.author;

	if author.bot {
		return Ok(());
	}

	let name = author.name.clone();
	let contents = message.content.clone();

	let msg = Msg {
		name,
		msg: contents,
	};

	redis::redis_pub(
		data.clone(),
		"ermc:discord",
		&serde_json::to_string(&msg).context("Failed to serialize message")?,
	)
	.await?;

	Ok(())
}
