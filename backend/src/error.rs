use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Stellar RPC error: {0}")]
    StellarRpc(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Account error: {0}")]
    Account(String),

    #[error("XDR encoding error: {0}")]
    XdrEncoding(String),

    #[error("XDR decoding error: {0}")]
    XdrDecoding(String),

    #[error("Task execution error: {0}")]
    TaskExecution(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Config(_) | AppError::Internal(_) | AppError::TaskExecution(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::StellarRpc(_) | AppError::Transaction(_) | AppError::Account(_) => {
                StatusCode::BAD_GATEWAY
            }
            AppError::XdrEncoding(_) | AppError::XdrDecoding(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    pub fn error_type(&self) -> &'static str {
        match self {
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::StellarRpc(_) => "STELLAR_RPC_ERROR",
            AppError::Transaction(_) => "TRANSACTION_ERROR",
            AppError::Account(_) => "ACCOUNT_ERROR",
            AppError::XdrEncoding(_) => "XDR_ENCODING_ERROR",
            AppError::XdrDecoding(_) => "XDR_DECODING_ERROR",
            AppError::TaskExecution(_) => "TASK_EXECUTION_ERROR",
            AppError::InvalidInput(_) => "INVALID_INPUT",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error_type: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = ErrorResponse {
            success: false,
            error_type: self.error_type(),
            message: self.to_string(),
            details: None,
        };

        (status, Json(error_response)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        AppError::TaskExecution(err.to_string())
    }
}