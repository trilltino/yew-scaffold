use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPublic {
    pub id: String,
    pub username: String,
    pub wallet_address: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignUpResponse {
    pub user: UserPublic,
    pub message: String,
}