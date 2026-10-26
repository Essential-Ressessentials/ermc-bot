use std::sync::Arc;

use anyhow::Context;
use axum::{Router, extract::Path, routing::get};

use crate::{context::TaskCtx, error::BotResult, utils::database};

async fn check_whitelist(Path(uuid): Path<String>) -> &'static str {
	let Ok(whitelisted) = database::check_uuid(&uuid) else {
		return "0";
	};

	if whitelisted {
		"1"
	} else {
		"0"
	}
}

pub async fn start_rest(ctx: Arc<TaskCtx>) -> BotResult {
	let config = &ctx.data.config;

	let rest_ip = &config.rest.ip;
	let rest_port = config.rest.port;

	let app = Router::new().route("/api/whitelist/{uuid}", get(check_whitelist));

	let listener = tokio::net::TcpListener::bind(format!("{rest_ip}:{rest_port}"))
		.await
		.context("Failed to bind to REST address")?;

	axum::serve(listener, app).await?;

	Ok(())
}
