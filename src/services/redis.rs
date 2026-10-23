use std::sync::Arc;

use anyhow::Context;
use redis::{AsyncTypedCommands, aio::PubSub};

use crate::{data::Data, error::BotResult};

pub async fn _redis_sub(data: Arc<Data>, channel: &str) -> BotResult<PubSub> {
	let client = &data.redis_client;

	let mut pubsub = client
		.get_async_pubsub()
		.await
		.context("Failed to get async pubsub")?;

	pubsub
		.subscribe(channel)
		.await
		.context("Failed to subscribe to channel")?;

	Ok(pubsub)
}

pub async fn redis_psub(data: Arc<Data>, pattern: &str) -> BotResult<PubSub> {
	let client = &data.redis_client;

	let mut pubsub = client
		.get_async_pubsub()
		.await
		.context("Failed to get async pubsub")?;

	pubsub
		.psubscribe(pattern)
		.await
		.context("Failed to subscribe to channel")?;

	Ok(pubsub)
}

pub async fn redis_pub(data: Arc<Data>, channel: &str, message: &str) -> BotResult<usize> {
	let mut manager = data.redis_manager.clone();

	let subscribers = manager
		.publish(channel, message)
		.await
		.context("Failed to publish message")?;

	Ok(subscribers)
}

#[cfg(test)]
mod tests {
	use crate::utils::config::Config;

	use super::*;

	#[tokio::test]
	async fn test() {
		let config = Config::load().unwrap();
		let data = Arc::new(
			Data::new(config)
				.await
				.unwrap(),
		);

		let payload = "{
			\"ign\": \"THMIGU\",
			\"uuid\": \"fd7f416a-04f9-4849-851b-c4c4cfbe1ac1\",
			\"name\": \"nothing\"
		}";

		let subscribers = redis_pub(data, "ermc:webhook:adv", payload)
			.await
			.unwrap();
		println!("{}", subscribers);
	}
}
