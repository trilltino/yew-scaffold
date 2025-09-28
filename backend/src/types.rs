use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct XdrRequest {
    pub source_account: String,
}

impl XdrRequest {
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.source_account.is_empty() {
            return Err(crate::error::AppError::InvalidInput("Source account cannot be empty".to_string()));
        }

        if !self.source_account.starts_with('G') || self.source_account.len() != 56 {
            return Err(crate::error::AppError::InvalidInput("Invalid Stellar account format".to_string()));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct XdrResponse {
    pub success: bool,
    pub xdr: String,
    pub message: String,
}

impl XdrResponse {
    pub fn success(xdr: String, message: String) -> Self {
        Self {
            success: true,
            xdr,
            message,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            xdr: String::new(),
            message,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SubmitRequest {
    pub signed_xdr: String,
}

impl SubmitRequest {
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.signed_xdr.is_empty() {
            return Err(crate::error::AppError::InvalidInput("Signed XDR cannot be empty".to_string()));
        }

        if self.signed_xdr.len() < 100 {
            return Err(crate::error::AppError::InvalidInput("Signed XDR appears too short".to_string()));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SubmitResponse {
    pub success: bool,
    pub result: String,
    pub transaction_hash: String,
    pub message: String,
}

impl SubmitResponse {
    pub fn success(result: String, transaction_hash: String, message: String) -> Self {
        Self {
            success: true,
            result,
            transaction_hash,
            message,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            result: String::new(),
            transaction_hash: String::new(),
            message,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub uptime: Option<u64>,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            service: "stellar-xdr-service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: None,
        }
    }

    pub fn with_uptime(mut self, uptime_seconds: u64) -> Self {
        self.uptime = Some(uptime_seconds);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error_type: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}