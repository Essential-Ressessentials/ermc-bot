pub mod rest;
pub mod status;
pub mod webhook;

use std::sync::Arc;

use crate::{context::TaskCtx, error::BotResult};

pub fn run_tasks(task_ctx: Arc<TaskCtx>) {
	tokio::spawn(run_task("Webhook", webhook::receive_webhook(task_ctx.clone())));
	tokio::spawn(run_task("REST", rest::start_rest(task_ctx.clone())));
	tokio::spawn(run_task("Status", status::start_status(task_ctx.clone())));
}

async fn run_task<F>(name: &'static str, task: F)
where
	F: Future<Output = BotResult>,
{
	if let Err(e) = task.await {
		eprintln!("{name} task failed: {e:#}")
	}
}
