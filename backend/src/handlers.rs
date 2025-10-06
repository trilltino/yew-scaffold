pub mod auth;
pub mod soroban;

use axum::{extract::Query, response::Json, extract::State};
use tracing::info;

use crate::config::AppState;
use crate::error::Result;
use crate::types::{XdrRequest, XdrResponse, SubmitRequest, SubmitResponse, HealthResponse};
use crate::services::stellar::{generate_hello_yew_xdr, submit_signed_transaction};
use crate::utils::truncate_address;

pub async fn generate_xdr_handler(
    State(state): State<AppState>,
    Query(params): Query<XdrRequest>,
) -> Result<Json<XdrResponse>> {
    let wallet_info = params.wallet_type.as_deref().unwrap_or("unknown");
    info!("XDR generation request received for account: {} (wallet: {})", params.source_account, wallet_info);

    params.validate()?;

    let function = params.get_function();
    let source_account = params.source_account;

    info!("Selected function: {} ({})", function.name(), function.signature());

    // Use XdrConfig from shared AppState instead of creating a new default
    let xdr_config = state.xdr_config.clone();

    info!("Generating XDR for account: {}, function: {}",
          truncate_address(&source_account), function.name());
    info!("Using contract: {}", xdr_config.contract_id);
    info!("Network: {}", xdr_config.network_passphrase);

    // Directly await the async function - no need for spawn_blocking
    let xdr = generate_hello_yew_xdr(&xdr_config, &source_account, &function).await?;

    info!("XDR generated successfully for {} wallet signing", wallet_info);
    Ok(Json(XdrResponse::success(
        xdr,
        format!("XDR generated successfully for {} wallet", wallet_info),
    )))
}

pub async fn submit_transaction_handler(
    State(state): State<AppState>,
    Json(payload): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>> {
    let wallet_info = payload.wallet_type.as_deref().unwrap_or("unknown");
    info!("Transaction submission request received from {} wallet", wallet_info);
    info!("Signed XDR length: {} characters", payload.signed_xdr.len());

    payload.validate()?;

    let function = payload.get_function();

    // Clone contract_id from shared state for logging/validation
    let contract_id = state.xdr_config.contract_id.clone();
    info!("Submitting transaction for contract: {}", contract_id);

    // Directly await the async function - no need for spawn_blocking
    let (hash, contract_result) = submit_signed_transaction(&payload.signed_xdr, &function).await?;

    info!("Transaction submitted successfully from {} wallet: {}", wallet_info, hash);
    Ok(Json(SubmitResponse::success(
        contract_result,
        hash,
        format!("Contract executed successfully via {} wallet", wallet_info),
    )))
}


pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse::healthy())
}