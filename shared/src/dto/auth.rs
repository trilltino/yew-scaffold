use serde::{Deserialize, Serialize};

/// Legacy wallet-only guest registration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Guest {
    pub username: String,
    pub wallet_address: String,
}

/// Request to sign up with email and password
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Request to login with email and password
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response after successful login (includes JWT token)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub user_id: i32,
    pub username: String,
    pub email: Option<String>,
    pub wallet_address: Option<String>,
    pub role: String,
    pub token: String,
}

/// Request to link a wallet address to the current authenticated user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkWalletRequest {
    pub wallet_address: String,
}