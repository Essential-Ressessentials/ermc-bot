#[cfg(unix)]
pub async fn sigterm_signal() {
	use tokio::signal::unix::{SignalKind, signal};

	let mut sigterm = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
	sigterm.recv().await;

	println!("Received SIGTERM");
}
