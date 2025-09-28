/// Simplified, secure XDR generation service for Stellar Soroban contracts
///
/// This service provides a single, focused endpoint for generating XDR for the hello_yew
/// contract function. It's designed with security, simplicity, and reliability in mind.

use axum::{
    http::Method,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use tracing::{info, warn};

// XDR generation module
mod xdr_generator;
use xdr_generator::{generate_hello_yew_xdr, submit_signed_transaction, XdrConfig};


/// Application state
#[derive(Clone)]
struct AppState {
    #[allow(dead_code)]
    config: AppConfig,
}

/// Simplified configuration
#[derive(Clone, Debug)]
struct AppConfig {
    pub port: u16,
    pub allowed_origins: Vec<String>,
    pub contract_id: String,
    #[allow(dead_code)]
    pub network_passphrase: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 3001,
            allowed_origins: vec![
                "http://localhost:8080".to_string(),
                "http://127.0.0.1:8080".to_string(),
                "http://localhost:8081".to_string(),
                "http://127.0.0.1:8081".to_string(),
            ],
            contract_id: "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF".to_string(),
            network_passphrase: "Test SDF Network ; September 2015".to_string(),
        }
    }
}

/// XDR generation request
#[derive(Debug, Deserialize)]
struct XdrRequest {
    source_account: String,
}

/// XDR generation response
#[derive(Debug, Serialize)]
struct XdrResponse {
    success: bool,
    xdr: String,
    message: String,
}

/// Transaction submission request
#[derive(Debug, Deserialize)]
struct SubmitRequest {
    signed_xdr: String,
}

/// Transaction submission response
#[derive(Debug, Serialize)]
struct SubmitResponse {
    success: bool,
    result: String,
    transaction_hash: String,
    message: String,
}

/// Health check response
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

/// Generate XDR for hello_yew contract function
async fn generate_xdr(axum::extract::Query(params): axum::extract::Query<XdrRequest>) -> Json<XdrResponse> {
    info!("XDR generation request received for account: {}", params.source_account);

    let source_account = params.source_account;

    // Test config - you can wire this back to state later
    let test_config = XdrConfig::default();

    info!("Generating XDR for test account: {}...{}",
          &source_account[..6], &source_account[source_account.len()-6..]);

    // Use spawn_blocking to handle the non-Send soroban-client types
    let result = tokio::task::spawn_blocking(move || {
        tokio::runtime::Handle::current().block_on(async move {
            generate_hello_yew_xdr(&test_config, &source_account).await
        })
    }).await;

    match result {
        Ok(Ok(xdr)) => {
            info!("✅ XDR generated successfully for Freighter wallet");
            Json(XdrResponse {
                success: true,
                xdr,
                message: "XDR generated successfully for Freighter wallet".to_string(),
            })
        }
        Ok(Err(error)) => {
            warn!("❌ XDR generation failed: {}", error);
            Json(XdrResponse {
                success: false,
                xdr: String::new(),
                message: format!("XDR generation failed: {}", error),
            })
        }
        Err(join_error) => {
            warn!("❌ Task join failed: {}", join_error);
            Json(XdrResponse {
                success: false,
                xdr: String::new(),
                message: format!("Task execution failed: {}", join_error),
            })
        }
    }
}

/// Submit signed transaction and get contract result
async fn submit_transaction(Json(payload): Json<SubmitRequest>) -> Json<SubmitResponse> {
    info!("Transaction submission request received");
    info!("Signed XDR length: {} characters", payload.signed_xdr.len());

    // Use spawn_blocking to handle the non-Send soroban-client types
    let result = tokio::task::spawn_blocking(move || {
        tokio::runtime::Handle::current().block_on(async move {
            submit_signed_transaction(&payload.signed_xdr).await
        })
    }).await;

    match result {
        Ok(Ok((hash, contract_result))) => {
            info!("✅ Transaction submitted successfully: {}", hash);
            Json(SubmitResponse {
                success: true,
                result: contract_result,
                transaction_hash: hash,
                message: "Contract executed successfully".to_string(),
            })
        }
        Ok(Err(error)) => {
            warn!("❌ Transaction submission failed: {}", error);
            Json(SubmitResponse {
                success: false,
                result: String::new(),
                transaction_hash: String::new(),
                message: format!("Transaction submission failed: {}", error),
            })
        }
        Err(join_error) => {
            warn!("❌ Task join failed: {}", join_error);
            Json(SubmitResponse {
                success: false,
                result: String::new(),
                transaction_hash: String::new(),
                message: format!("Task execution failed: {}", join_error),
            })
        }
    }
}

/// Health check endpoint
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "stellar-xdr-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}


/// Validate Stellar address format (G followed by 55 characters)


/// Create CORS layer with secure configuration
fn create_cors_layer(allowed_origins: Vec<String>) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // If specific origins are configured, use them; otherwise allow any for development
    if allowed_origins.is_empty() || allowed_origins.contains(&"*".to_string()) {
        cors = cors.allow_origin(Any);
    } else {
        for origin in allowed_origins {
            if let Ok(origin_header) = origin.parse::<axum::http::HeaderValue>() {
                cors = cors.allow_origin(origin_header);
            }
        }
    }

    cors
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    info!("Starting Stellar XDR Service");

    // Load configuration
    let config = AppConfig::default();
    info!("Service configuration loaded");
    info!("  Port: {}", config.port);
    info!("  Contract ID: {}", config.contract_id);
    info!("  Allowed origins: {:?}", config.allowed_origins);

    // Create application state
    let app_state = AppState { config: config.clone() };

    // Build router with simplified middleware
    let app = Router::new()
        .route("/generate-xdr", get(generate_xdr))
        .route("/submit-transaction", post(submit_transaction))
        .route("/health", get(health))
        .with_state(app_state)
        .layer(create_cors_layer(config.allowed_origins.clone()));

    // Start server
    let bind_address = format!("127.0.0.1:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    info!("🚀 Stellar XDR Service running on http://{}", bind_address);
    info!("📋 Health check: http://{}/health", bind_address);
    info!("🔧 Generate XDR: http://{}/generate-xdr?source_account=GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54", bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}