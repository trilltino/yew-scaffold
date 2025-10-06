/// Frontend types that mirror the backend ContractFunction enum
/// This ensures type safety when communicating with the backend
use serde::{Serialize, Deserialize};

/// Backend response for XDR generation
#[derive(Debug, Deserialize)]
pub struct XdrResponse {
    pub success: bool,
    pub xdr: String,
    pub message: String,
}

/// Backend response for transaction submission
#[derive(Debug, Deserialize)]
pub struct SubmitResponse {
    pub success: bool,
    pub result: String,
    pub transaction_hash: String,
    pub message: String,
}

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

// Include tests module
#[cfg(test)]
#[path = "types_test.rs"]
mod types_test;

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

    /// Get the display name for UI buttons
    pub fn display_name(&self) -> &'static str {
        match self {
            ContractFunction::Hello { .. } => "Hello",
            ContractFunction::HelloYew { .. } => "Hello Yew",
            ContractFunction::Simple => "Simple",
            ContractFunction::TestFunc123 { .. } => "Test Func 123",
            ContractFunction::X { .. } => "X",
            ContractFunction::EdgeCaseTestYew123End { .. } => "Edge Case Test",
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
}