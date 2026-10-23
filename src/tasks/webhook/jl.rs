use std::sync::Arc;

use poise::serenity_prelude::{CreateEmbed, ExecuteWebhook, colours::branding::WHITE};
use serde::Deserialize;

use crate::{context::TaskCtx, error::BotResult, tasks::webhook::send};

#[derive(Deserialize)]
struct JL {
	ign: String,
	uuid: String,
}

pub async fn join_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(join) = serde_json::from_str::<JL>(payload) else {
		return Ok(());
	};

	let avatar_url = format!("https://visage.surgeplay.com/head/512/{}", join.uuid);
	let title = format!("🟢 {} has joined the game", join.ign);

	let embed = CreateEmbed::new()
		.title(title)
		.color(WHITE);

	let builder = ExecuteWebhook::new()
		.username(join.ign)
		.avatar_url(avatar_url)
		.embed(embed);

	send::send_webhook(ctx, builder).await?;

	Ok(())
}

pub async fn leave_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(leave) = serde_json::from_str::<JL>(payload) else {
		return Ok(());
	};

	let avatar_url = format!("https://visage.surgeplay.com/head/512/{}", leave.uuid);
	let title = format!("🔴 {} has left the game", leave.ign);

	let embed = CreateEmbed::new()
		.title(title)
		.color(WHITE);

	let builder = ExecuteWebhook::new()
		.username(leave.ign)
		.avatar_url(avatar_url)
		.embed(embed);

	send::send_webhook(ctx, builder).await?;

	Ok(())
}
