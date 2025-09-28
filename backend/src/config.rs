use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub allowed_origins: Vec<String>,
    pub contract_id: String,
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

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .unwrap_or(3001),
            allowed_origins: std::env::var("ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:8080,http://127.0.0.1:8080,http://localhost:8081,http://127.0.0.1:8081".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            contract_id: std::env::var("CONTRACT_ID")
                .unwrap_or_else(|_| "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF".to_string()),
            network_passphrase: std::env::var("NETWORK_PASSPHRASE")
                .unwrap_or_else(|_| "Test SDF Network ; September 2015".to_string()),
        }
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        if self.port == 0 {
            return Err(crate::error::AppError::Config("Port cannot be 0".to_string()));
        }

        if self.contract_id.is_empty() {
            return Err(crate::error::AppError::Config("Contract ID cannot be empty".to_string()));
        }

        if self.network_passphrase.is_empty() {
            return Err(crate::error::AppError::Config("Network passphrase cannot be empty".to_string()));
        }

        if !self.contract_id.starts_with('C') || self.contract_id.len() != 56 {
            return Err(crate::error::AppError::Config("Invalid contract ID format".to_string()));
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> crate::error::Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }
}