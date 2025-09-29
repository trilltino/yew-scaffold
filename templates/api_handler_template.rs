/// Template for new API handlers in backend
///
/// To create a new API endpoint:
/// 1. Copy this template to backend/src/handlers.rs
/// 2. Replace FUNCTION_NAME and ENDPOINT_NAME
/// 3. Define request/response types
/// 4. Add route to backend/src/lib.rs
/// 5. Test with curl

use axum::{
    extract::{Query, State},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::{info, error};

use crate::{AppState, AppError, Result};

/// Request parameters for the new endpoint
#[derive(Debug, Deserialize)]
pub struct NewEndpointRequest {
    /// Example: Account address
    pub account: String,

    /// Example: Optional parameter
    pub param: Option<String>,

    /// Example: Amount (use String for large numbers)
    pub amount: Option<String>,
}

/// Response for the new endpoint
#[derive(Debug, Serialize)]
pub struct NewEndpointResponse {
    /// Success status
    pub success: bool,

    /// Response data
    pub data: String,

    /// Human-readable message
    pub message: String,

    /// Optional additional metadata
    pub metadata: Option<serde_json::Value>,
}

/// Handler for new endpoint
pub async fn new_endpoint_handler(
    Query(params): Query<NewEndpointRequest>,
    State(state): State<AppState>,
) -> Result<Json<NewEndpointResponse>> {
    info!("New endpoint request received for account: {}", params.account);

    // Validate input parameters
    if params.account.is_empty() {
        error!("Empty account parameter");
        return Err(AppError::BadRequest("Account parameter is required".to_string()));
    }

    // Validate Stellar address format
    if !params.account.starts_with('G') || params.account.len() != 56 {
        error!("Invalid Stellar address format: {}", params.account);
        return Err(AppError::BadRequest("Invalid Stellar address format".to_string()));
    }

    // Example: Call Stellar service
    let result = match call_stellar_service(&params, &state).await {
        Ok(data) => data,
        Err(e) => {
            error!("Stellar service call failed: {:?}", e);
            return Err(AppError::StellarError(format!("Service call failed: {}", e)));
        }
    };

    info!("New endpoint completed successfully for account: {}", params.account);

    Ok(Json(NewEndpointResponse {
        success: true,
        data: result,
        message: "Operation completed successfully".to_string(),
        metadata: Some(serde_json::json!({
            "account": params.account,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
    }))
}

/// Example Stellar service call
async fn call_stellar_service(
    params: &NewEndpointRequest,
    state: &AppState,
) -> Result<String> {
    // Example: Using soroban client
    /*
    let client = &state.soroban_client;
    let response = client
        .some_method(&params.account)
        .await
        .map_err(|e| AppError::StellarError(format!("Client error: {}", e)))?;
    */

    // Example: HTTP request to Stellar RPC
    /*
    let rpc_url = "https://soroban-testnet.stellar.org";
    let response = reqwest::get(&format!("{}/some-endpoint", rpc_url))
        .await
        .map_err(|e| AppError::NetworkError(format!("RPC request failed: {}", e)))?
        .text()
        .await
        .map_err(|e| AppError::NetworkError(format!("Response parsing failed: {}", e)))?;
    */

    // Placeholder implementation
    Ok(format!("Processed account: {}", params.account))
}

// Add to backend/src/lib.rs in create_app():
/*
let app = Router::new()
    .route("/generate-xdr", get(generate_xdr_handler))
    .route("/submit-transaction", post(submit_transaction_handler))
    .route("/health", get(health_handler))
    .route("/new-endpoint", get(new_endpoint_handler))  // Add this line
    .with_state(state)
    .layer(create_cors_layer(config.allowed_origins.clone()));
*/

// Test with curl:
/*
curl "http://127.0.0.1:3001/new-endpoint?account=GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54&param=test"
*/

// Frontend usage example:
/*
async fn call_new_endpoint(account: &str, param: Option<&str>) -> Result<String, String> {
    let mut url = format!("http://127.0.0.1:3001/new-endpoint?account={}", account);
    if let Some(p) = param {
        url.push_str(&format!("&param={}", p));
    }

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Backend error: HTTP {}", response.status()));
    }

    let endpoint_response: NewEndpointResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    if endpoint_response.success {
        Ok(endpoint_response.data)
    } else {
        Err(endpoint_response.message)
    }
}
*/