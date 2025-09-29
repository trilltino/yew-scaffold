use tracing::info;

use stellar_xdr_service::{AppConfig, run_server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    info!("Starting Stellar XDR Service");

    let config = AppConfig::from_env();

    if let Err(e) = run_server(config).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}