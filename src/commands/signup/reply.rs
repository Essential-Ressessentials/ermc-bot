use anyhow::Context;
use poise::{
	CreateReply,
	serenity_prelude::{
		ComponentInteraction, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponse,
		CreateInteractionResponseMessage, CreateMessage, Mentionable, Message, UserId,
		colours::branding::WHITE,
	},
};

use crate::{commands::signup::profile::Profile, context::Ctx, error::BotResult, utils::response};

const SKIN_API: &str = "https://nmsr.nickac.dev/fullbodyiso";

pub async fn send_confirmation(ctx: Ctx<'_>, profile: &Profile) -> BotResult<Message> {
	let name = &profile.name;
	let uuid = &profile.uuid;

	let skin_url = format!("{}/{}", SKIN_API, uuid);

	let mut author = CreateEmbedAuthor::new(&ctx.author().name);
	if let Some(url) = ctx.author().avatar_url() {
		author = author.icon_url(url);
	}

	let embed = CreateEmbed::default()
		.author(author)
		.title("Is this you?")
		.description(format!("**IGN:** {}\n**Discord:** {}", name, ctx.author().mention()))
		.thumbnail(skin_url)
		.color(WHITE);
	let row = response::confirmation_action_row();

	let reply = CreateReply::default()
		.embed(embed)
		.components(vec![row]);

	let handle = ctx
		.send(reply)
		.await
		.context("Failed to reply to signup")?;
	let message = handle
		.into_message()
		.await
		.context("Failed to get message from handle")?;

	Ok(message)
}

pub async fn user_confirmed(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let embed = CreateEmbed::default()
		.title("Request sent")
		.description("Your request has been sent")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx, interaction_response)
		.await
		.context("Failed to respond to confirmation")?;

	Ok(())
}

pub async fn user_denied(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let embed = CreateEmbed::default()
		.title("Request cancelled")
		.description("Your request has been cancelled")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx, interaction_response)
		.await
		.context("Failed to respond to denial")?;

	Ok(())
}

pub async fn send_request(ctx: Ctx<'_>, profile: &Profile) -> BotResult<Message> {
	let config = &ctx.data().config;

	let owner_id = UserId::new(config.discord.owner_id);
	let owner = owner_id
		.to_user(ctx)
		.await
		.context("Failed to get user from owner ID")?;

	let name = &profile.name;
	let uuid = &profile.uuid;

	let skin_url = format!("{}/{}", SKIN_API, uuid);

	let mut author = CreateEmbedAuthor::new(&ctx.author().name);
	if let Some(url) = ctx.author().avatar_url() {
		author = author.icon_url(url);
	}

	let embed = CreateEmbed::default()
		.author(author)
		.title("Whitelist Request")
		.description(format!("**IGN:** {}\n**Discord:** {}", name, ctx.author().mention()))
		.thumbnail(skin_url)
		.color(WHITE);
	let row = response::confirmation_action_row();

	let message = CreateMessage::default()
		.embed(embed)
		.components(vec![row]);

	let sent = owner
		.dm(ctx, message)
		.await
		.context("Failed to send request to owner")?;

	Ok(sent)
}

pub async fn owner_confirmed(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let mut author = CreateEmbedAuthor::new(&ctx.author().name);
	if let Some(url) = ctx.author().avatar_url() {
		author = author.icon_url(url);
	}

	let embed = CreateEmbed::default()
		.author(author)
		.title("Request confirmed")
		.description("This request has been confirmed")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx, interaction_response)
		.await
		.context("Failed to respond to confirmation")?;

	Ok(())
}

pub async fn owner_denied(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let mut author = CreateEmbedAuthor::new(&ctx.author().name);
	if let Some(url) = ctx.author().avatar_url() {
		author = author.icon_url(url);
	}

	let embed = CreateEmbed::default()
		.author(author)
		.title("Request denied")
		.description("This request has been denied")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx, interaction_response)
		.await
		.context("Failed to respond to denial")?;

	Ok(())
}

pub async fn send_confirmed(ctx: Ctx<'_>, discord_id: u64) -> BotResult {
	let user_id = UserId::new(discord_id);
	let user = user_id
		.to_user(ctx)
		.await
		.context("Failed to get user from user ID")?;

	let embed = CreateEmbed::default()
		.title("Request accepted")
		.description("Your whitelist request has been accepted")
		.color(WHITE);

	let message = CreateMessage::default().embed(embed);

	user.dm(ctx, message)
		.await
		.context("Failed to send message to user")?;

	Ok(())
}

pub async fn send_denied(ctx: Ctx<'_>, discord_id: u64) -> BotResult {
	let user_id = UserId::new(discord_id);
	let user = user_id
		.to_user(ctx)
		.await
		.context("Failed to get user from user ID")?;

	let embed = CreateEmbed::default()
		.title("Request denied")
		.description("Your whitelist request has been denied")
		.color(WHITE);

	let message = CreateMessage::default().embed(embed);

	user.dm(ctx, message)
		.await
		.context("Failed to send message to user")?;

	Ok(())
}
