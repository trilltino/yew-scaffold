use serde::{Deserialize, Serialize};
use soroban_client::xdr::ScVal;

/// Available contract functions with their signatures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContractFunction {
    /// hello(to: string) -> vec<string>
    Hello { to: String },
    /// hello_yew(to: string) -> vec<string>
    HelloYew { to: String },
    /// simple() -> string
    Simple,
    /// test_func_123(param_1: string, param_2: u32) -> string
    TestFunc123 { param_1: String, param_2: u32 },
    /// x(y: string) -> string
    X { y: String },
    /// edge_case_test_yew_123_end(edge_input: string) -> string
    EdgeCaseTestYew123End { edge_input: String },
}

impl ContractFunction {
    /// Get the function name as it appears in the contract
    pub fn name(&self) -> &'static str {
        match self {
            ContractFunction::Hello { .. } => "hello",
            ContractFunction::HelloYew { .. } => "hello_yew",
            ContractFunction::Simple => "simple",
            ContractFunction::TestFunc123 { .. } => "test_func_123",
            ContractFunction::X { .. } => "x",
            ContractFunction::EdgeCaseTestYew123End { .. } => "edge_case_test_yew_123_end",
        }
    }

    /// Get the function signature for display
    pub fn signature(&self) -> &'static str {
        match self {
            ContractFunction::Hello { .. } => "hello(to: string) -> vec<string>",
            ContractFunction::HelloYew { .. } => "hello_yew(to: string) -> vec<string>",
            ContractFunction::Simple => "simple() -> string",
            ContractFunction::TestFunc123 { .. } => "test_func_123(param_1: string, param_2: u32) -> string",
            ContractFunction::X { .. } => "x(y: string) -> string",
            ContractFunction::EdgeCaseTestYew123End { .. } => "edge_case_test_yew_123_end(edge_input: string) -> string",
        }
    }

    /// Get function description
    pub fn description(&self) -> &'static str {
        match self {
            ContractFunction::Hello { .. } => "Original hello function",
            ContractFunction::HelloYew { .. } => "Our test function that matches the expected behavior",
            ContractFunction::Simple => "Simple function for baseline testing",
            ContractFunction::TestFunc123 { .. } => "Function with numbers and underscores to test encoding",
            ContractFunction::X { .. } => "Function with single character name",
            ContractFunction::EdgeCaseTestYew123End { .. } => "Function that might trigger encoding edge cases",
        }
    }

    /// Convert to ScVal parameters for Soroban
    ///
    /// Returns Result to handle conversion errors gracefully instead of panicking
    pub fn to_scval_params(&self) -> crate::error::Result<Vec<ScVal>> {
        match self {
            ContractFunction::Hello { to } => {
                Ok(vec![ScVal::String(
                    to.as_bytes().to_vec().try_into()
                        .map_err(|_| crate::error::AppError::XdrEncoding(
                            format!("Failed to convert 'to' parameter (length: {})", to.len())
                        ))?
                )])
            }
            ContractFunction::HelloYew { to } => {
                Ok(vec![ScVal::String(
                    to.as_bytes().to_vec().try_into()
                        .map_err(|_| crate::error::AppError::XdrEncoding(
                            format!("Failed to convert 'to' parameter (length: {})", to.len())
                        ))?
                )])
            }
            ContractFunction::Simple => Ok(vec![]),
            ContractFunction::TestFunc123 { param_1, param_2 } => {
                Ok(vec![
                    ScVal::String(
                        param_1.as_bytes().to_vec().try_into()
                            .map_err(|_| crate::error::AppError::XdrEncoding(
                                format!("Failed to convert 'param_1' (length: {})", param_1.len())
                            ))?
                    ),
                    ScVal::U32(*param_2),
                ])
            }
            ContractFunction::X { y } => {
                Ok(vec![ScVal::String(
                    y.as_bytes().to_vec().try_into()
                        .map_err(|_| crate::error::AppError::XdrEncoding(
                            format!("Failed to convert 'y' parameter (length: {})", y.len())
                        ))?
                )])
            }
            ContractFunction::EdgeCaseTestYew123End { edge_input } => {
                Ok(vec![ScVal::String(
                    edge_input.as_bytes().to_vec().try_into()
                        .map_err(|_| crate::error::AppError::XdrEncoding(
                            format!("Failed to convert 'edge_input' (length: {})", edge_input.len())
                        ))?
                )])
            }
        }
    }

    /// Get all available functions with default parameters for UI
    pub fn all_functions() -> Vec<ContractFunction> {
        vec![
            ContractFunction::Hello { to: "World".to_string() },
            ContractFunction::HelloYew { to: "Yew".to_string() },
            ContractFunction::Simple,
            ContractFunction::TestFunc123 {
                param_1: "hello".to_string(),
                param_2: 42
            },
            ContractFunction::X { y: "test".to_string() },
            ContractFunction::EdgeCaseTestYew123End {
                edge_input: "edge_test".to_string()
            },
        ]
    }

    /// Parse function name string to ContractFunction with default parameters
    /// Centralized logic to avoid duplication in XdrRequest and SubmitRequest
    pub fn from_name(name: Option<&str>) -> Self {
        match name {
            Some("hello") => ContractFunction::Hello { to: "World".to_string() },
            Some("hello_yew") => ContractFunction::HelloYew { to: "Yew".to_string() },
            Some("simple") => ContractFunction::Simple,
            Some("test_func_123") => ContractFunction::TestFunc123 {
                param_1: "hello".to_string(),
                param_2: 42,
            },
            Some("x") => ContractFunction::X { y: "test".to_string() },
            Some("edge_case_test_yew_123_end") => ContractFunction::EdgeCaseTestYew123End {
                edge_input: "edge_test".to_string(),
            },
            _ => ContractFunction::TestFunc123 {
                param_1: "hello".to_string(),
                param_2: 42,
            },
        }
    }
}

/// Request for generating transaction XDR
/// Works with any Stellar wallet - Freighter, Lobstr, Albedo, etc.
#[derive(Debug, Deserialize)]
pub struct XdrRequest {
    /// The Stellar public key (account ID) to create the transaction for
    pub source_account: String,
    /// Optional wallet type for logging and analytics (e.g. "freighter", "lobstr")
    #[serde(default)]
    pub wallet_type: Option<String>,
    /// The contract function to call (simple name)
    #[serde(default)]
    pub function_name: Option<String>,
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

    /// Convert function name to ContractFunction (delegates to centralized logic)
    pub fn get_function(&self) -> ContractFunction {
        ContractFunction::from_name(self.function_name.as_deref())
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

/// Request for submitting a signed transaction
/// Accepts signed XDR from any Stellar wallet
#[derive(Debug, Deserialize)]
pub struct SubmitRequest {
    /// The signed transaction XDR from any Stellar wallet
    pub signed_xdr: String,
    /// Optional wallet type for logging and analytics (e.g. "freighter", "lobstr")
    #[serde(default)]
    pub wallet_type: Option<String>,
    /// The contract function name being submitted
    #[serde(default)]
    pub function_name: Option<String>,
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

    /// Convert function name to ContractFunction (delegates to centralized logic)
    pub fn get_function(&self) -> ContractFunction {
        ContractFunction::from_name(self.function_name.as_deref())
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

