mod advancement;
mod chat;
mod death;
mod jl;
mod send;
mod sleep;

use std::sync::Arc;

use anyhow::Context;
use futures_util::StreamExt;

use crate::{context::TaskCtx, error::BotResult, services::redis};

pub async fn receive_webhook(ctx: Arc<TaskCtx>) -> BotResult {
	let mut sub = redis::redis_psub(ctx.data.clone(), "ermc:webhook:*").await?;

	let mut stream = sub.on_message();

	while let Some(msg) = stream.next().await {
		let payload: String = msg
			.get_payload()
			.context("Failed to get payload")?;

		let channel = msg.get_channel_name();
		let prefix = "ermc:webhook:";

		let subchannel = channel
			.strip_prefix(prefix)
			.unwrap();

		match subchannel {
			"join" => jl::join_webhook(ctx.clone(), &payload).await?,
			"leave" => jl::leave_webhook(ctx.clone(), &payload).await?,
			"chat" => chat::chat_webhook(ctx.clone(), &payload).await?,
			"sleep" => sleep::sleep_webhook(ctx.clone(), &payload).await?,
			"death" => death::death_webhook(ctx.clone(), &payload).await?,
			"adv" => advancement::adv_webhook(ctx.clone(), &payload).await?,
			_ => continue,
		};
	}

	Ok(())
}
