pub mod config;
pub mod error;
pub mod handlers;
pub mod services;
pub mod types;
pub mod utils;

use axum::{routing::{get, post}, Router};
use tracing::info;

pub use config::{AppConfig, AppState};
pub use error::{AppError, Result};
pub use handlers::{generate_xdr_handler, submit_transaction_handler, health_handler};
pub use types::{XdrRequest, XdrResponse, SubmitRequest, SubmitResponse, HealthResponse};
pub use utils::create_cors_layer;

pub async fn create_app(config: AppConfig) -> Result<Router> {
    let state = AppState::new(config.clone())?;

    let app = Router::new()
        .route("/generate-xdr", get(generate_xdr_handler))
        .route("/submit-transaction", post(submit_transaction_handler))
        .route("/health", get(health_handler))
        .with_state(state)
        .layer(create_cors_layer(config.allowed_origins.clone()));

    Ok(app)
}

pub async fn run_server(config: AppConfig) -> Result<()> {
    info!("Service configuration loaded");
    info!("  Port: {}", config.port);
    info!("  Contract ID: {}", config.contract_id);
    info!("  Allowed origins: {:?}", config.allowed_origins);

    let app = create_app(config.clone()).await?;

    let bind_address = format!("127.0.0.1:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .map_err(|e| AppError::Config(format!("Failed to bind to address {}: {}", bind_address, e)))?;

    info!("🚀 Stellar XDR Service running on http://{}", bind_address);
    info!("📋 Health check: http://{}/health", bind_address);
    info!("🔧 Generate XDR: http://{}/generate-xdr?source_account=GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54", bind_address);

    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::Internal(format!("Server error: {}", e)))?;

    Ok(())
}