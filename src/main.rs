mod commands;
mod context;
mod data;
mod error;
mod events;
mod services;
mod setup;
mod tasks;
mod utils;

use anyhow::Context;
use poise::serenity_prelude as serenity;

use crate::{
	commands::commands,
	error::{BotResult, on_error},
	events::event_handler,
	utils::config::Config,
};

#[tokio::main]
async fn main() -> BotResult {
	let config: Config = Config::load().context("Failed to load config")?;
	let token = config.discord.token;

	let intents = serenity::GatewayIntents::all();

	let framework_options = poise::FrameworkOptions {
		commands: commands(),
		event_handler: |ctx, event, _framework, data| Box::pin(event_handler(ctx, event, data)),
		on_error: |err| Box::pin(on_error(err)),
		prefix_options: poise::PrefixFrameworkOptions {
			mention_as_prefix: false,
			..Default::default()
		},
		..Default::default()
	};

	let framework = poise::Framework::builder()
		.options(framework_options)
		.setup(|ctx, ready, framework| Box::pin(setup::setup(ctx, ready, framework)))
		.build();

	let mut client = serenity::ClientBuilder::new(&token, intents)
		.framework(framework)
		.await
		.context("Failed to initialize client")?;

	#[cfg(unix)]
	{
		use crate::utils::sigterm;

		let shard_manager = client.shard_manager.clone();

		tokio::select! {
			result = client.start() => {
				result.context("Failed to start client")?;
			}
			_ = sigterm::sigterm_signal() => {
				shard_manager.shutdown_all().await;
			}
		}
	}

	#[cfg(not(unix))]
	client
		.start()
		.await
		.context("Failed to start client")?;

	println!("Shutting down");

	Ok(())
}
