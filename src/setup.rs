use std::sync::Arc;

use anyhow::Context;
use poise::{
	Framework,
	serenity_prelude::{Context as SerenityContext, Ready},
};

use crate::{
	context::TaskCtx,
	data::Data,
	error::{BotError, BotResult},
	tasks,
	utils::{config::Config, database},
};

pub async fn setup(
	ctx: &SerenityContext,
	ready: &Ready,
	framework: &Framework<Arc<Data>, BotError>,
) -> BotResult<Arc<Data>> {
	let config: Config = Config::load().context("Failed to load config")?;

	poise::builtins::register_globally(ctx, &framework.options().commands)
		.await
		.context("Failed to register commands")?;
	println!("Logged in as {}", ready.user.tag());

	database::init_db().context("Failed to initialize database")?;

	let data = Arc::new(
		Data::new(config)
			.await
			.context("Failed to initialize data")?,
	);

	let task_ctx = Arc::new(TaskCtx {
		data: data.clone(),
		http: ctx.http.clone(),
	});

	tokio::spawn(tasks::webhook::receive_webhook(task_ctx.clone()));
	tokio::spawn(tasks::rest::start_rest(task_ctx.clone()));

	Ok(data)
}
