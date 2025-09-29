use axum::{extract::Query, response::Json, extract::State};
use tracing::{info, warn};

use crate::config::AppState;
use crate::error::{AppError, Result};
use crate::types::{XdrRequest, XdrResponse, SubmitRequest, SubmitResponse, HealthResponse};
use crate::services::stellar::{generate_hello_yew_xdr, submit_signed_transaction};
use crate::services::stellar::XdrConfig;

pub async fn generate_xdr_handler(
    State(_state): State<AppState>,
    Query(params): Query<XdrRequest>,
) -> Result<Json<XdrResponse>> {
    let wallet_info = params.wallet_type.as_deref().unwrap_or("unknown");
    info!("XDR generation request received for account: {} (wallet: {})", params.source_account, wallet_info);

    params.validate()?;

    let function = params.get_function();
    let source_account = params.source_account;

    info!("Selected function: {} ({})", function.name(), function.signature());

    let test_config = XdrConfig::default();

    info!("Generating XDR for test account: {}...{}, function: {}",
          &source_account[..6], &source_account[source_account.len()-6..], function.name());

    let result = tokio::task::spawn_blocking(move || {
        tokio::runtime::Handle::current().block_on(async move {
            generate_hello_yew_xdr(&test_config, &source_account, &function).await
        })
    }).await;

    match result {
        Ok(Ok(xdr)) => {
            info!("✅ XDR generated successfully for Freighter wallet signing");
            Ok(Json(XdrResponse::success(
                xdr,
                "XDR generated successfully for Freighter wallet".to_string(),
            )))
        }
        Ok(Err(error)) => {
            warn!("❌ XDR generation failed: {}", error);
            Err(AppError::XdrEncoding(error.to_string()))
        }
        Err(join_error) => {
            warn!("❌ Task join failed: {}", join_error);
            Err(AppError::TaskExecution(join_error.to_string()))
        }
    }
}

pub async fn submit_transaction_handler(
    State(_state): State<AppState>,
    Json(payload): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>> {
    let wallet_info = payload.wallet_type.as_deref().unwrap_or("unknown");
    info!("Transaction submission request received from {} wallet", wallet_info);
    info!("Signed XDR length: {} characters", payload.signed_xdr.len());

    payload.validate()?;

    let function = payload.get_function();

    let result = tokio::task::spawn_blocking(move || {
        tokio::runtime::Handle::current().block_on(async move {
            submit_signed_transaction(&payload.signed_xdr, &function).await
        })
    }).await;

    match result {
        Ok(Ok((hash, contract_result))) => {
            info!("✅ Transaction submitted successfully: {}", hash);
            Ok(Json(SubmitResponse::success(
                contract_result,
                hash,
                "Contract executed successfully".to_string(),
            )))
        }
        Ok(Err(error)) => {
            warn!("❌ Transaction submission failed: {}", error);
            Err(AppError::Transaction(error.to_string()))
        }
        Err(join_error) => {
            warn!("❌ Task join failed: {}", join_error);
            Err(AppError::TaskExecution(join_error.to_string()))
        }
    }
}

pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse::healthy())
}