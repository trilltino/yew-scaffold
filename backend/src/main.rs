use tracing::info;

use stellar_xdr_service::{AppConfig, run_server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    match dotenvy::dotenv() {
        Ok(path) => eprintln!("Loaded .env from: {:?}", path),
        Err(e) => eprintln!("WARNING: Failed to load .env: {}", e),
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    info!("Starting Stellar XDR Service");

    // Debug: Check if DATABASE_URL is set
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        info!("DATABASE_URL detected: {}...", &db_url[..30]);
    } else {
        info!("DATABASE_URL not found in environment");
    }

    let config = AppConfig::from_env();

    if let Err(e) = run_server(config).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}