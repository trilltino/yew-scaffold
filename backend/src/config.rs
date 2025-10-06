use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn, debug};
use crate::services::stellar::XdrConfig;
use crate::services::soroban::ScalableContractManager;
use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    // Server configuration
    pub port: u16,
    pub allowed_origins: Vec<String>,

    // Stellar/Soroban configuration
    pub contract_id: String,
    pub network_passphrase: String,
    pub rpc_url: String,

    // JWT authentication configuration
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,

    // Cookie configuration
    pub cookie_name: String,
    pub cookie_domain: String,
    pub cookie_secure: bool,
    pub cookie_http_only: bool,
    pub cookie_same_site: String,
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
                "http://localhost:8083".to_string(),
                "http://127.0.0.1:8083".to_string(),
            ],
            contract_id: "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF".to_string(),
            network_passphrase: "Test SDF Network ; September 2015".to_string(),
            rpc_url: "https://soroban-testnet.stellar.org".to_string(),
            jwt_secret: "change-this-in-production-min-32-characters!".to_string(),
            jwt_expiration_hours: 24,
            cookie_name: "yew_auth".to_string(),
            cookie_domain: "localhost".to_string(),
            cookie_secure: false,
            cookie_http_only: true,
            cookie_same_site: "Lax".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        debug!("[CONFIG] Loading configuration from environment variables");

        Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .unwrap_or(3001),
            allowed_origins: std::env::var("ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:8080,http://127.0.0.1:8080,http://localhost:8081,http://127.0.0.1:8081,http://localhost:8083,http://127.0.0.1:8083".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            contract_id: std::env::var("CONTRACT_ID")
                .unwrap_or_else(|_| "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF".to_string()),
            network_passphrase: std::env::var("NETWORK_PASSPHRASE")
                .unwrap_or_else(|_| "Test SDF Network ; September 2015".to_string()),
            rpc_url: std::env::var("RPC_URL")
                .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string()),

            // JWT configuration
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| {
                    warn!("[CONFIG] JWT_SECRET not set, using default (INSECURE!)");
                    "change-this-in-production-min-32-characters!".to_string()
                }),
            jwt_expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),

            // Cookie configuration
            cookie_name: std::env::var("COOKIE_NAME")
                .unwrap_or_else(|_| "yew_auth".to_string()),
            cookie_domain: std::env::var("COOKIE_DOMAIN")
                .unwrap_or_else(|_| "localhost".to_string()),
            cookie_secure: std::env::var("COOKIE_SECURE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            cookie_http_only: std::env::var("COOKIE_HTTP_ONLY")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            cookie_same_site: std::env::var("COOKIE_SAME_SITE")
                .unwrap_or_else(|_| "Lax".to_string()),
        }
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        debug!("[CONFIG] Validating configuration");

        // Server validation
        if self.port == 0 {
            return Err(crate::error::AppError::Config("Port cannot be 0".to_string()));
        }

        // Stellar/Soroban validation
        if self.contract_id.is_empty() {
            return Err(crate::error::AppError::Config("Contract ID cannot be empty".to_string()));
        }

        if self.network_passphrase.is_empty() {
            return Err(crate::error::AppError::Config("Network passphrase cannot be empty".to_string()));
        }

        if !self.contract_id.starts_with('C') || self.contract_id.len() != 56 {
            return Err(crate::error::AppError::Config("Invalid contract ID format".to_string()));
        }

        if self.rpc_url.is_empty() {
            return Err(crate::error::AppError::Config("RPC URL cannot be empty".to_string()));
        }

        // JWT validation
        if self.jwt_secret.len() < 32 {
            warn!("[CONFIG] ❌ JWT_SECRET is too short ({} characters), must be at least 32 characters", self.jwt_secret.len());
            return Err(crate::error::AppError::Config("JWT_SECRET must be at least 32 characters for security".to_string()));
        }

        if self.jwt_secret.contains("change-this") {
            warn!("[CONFIG] ⚠️  SECURITY WARNING: JWT_SECRET appears to be a default value. Change it in production!");
        }

        if self.jwt_expiration_hours < 1 || self.jwt_expiration_hours > 720 {
            warn!("[CONFIG] ⚠️  JWT_EXPIRATION_HOURS should be between 1 and 720 (30 days), got: {}", self.jwt_expiration_hours);
        }

        // Cookie validation
        if !self.cookie_secure && self.cookie_domain != "localhost" && !self.cookie_domain.starts_with("127.") {
            warn!("[CONFIG] ⚠️  SECURITY WARNING: COOKIE_SECURE=false on non-localhost domain. Set to true in production!");
        }

        if !["Strict", "Lax", "None"].contains(&self.cookie_same_site.as_str()) {
            return Err(crate::error::AppError::Config(format!("COOKIE_SAME_SITE must be 'Strict', 'Lax', or 'None', got: {}", self.cookie_same_site)));
        }

        info!("[CONFIG] ✅ Configuration validation passed");
        Ok(())
    }

    /// Get JWT expiration in seconds
    pub fn jwt_expiration_seconds(&self) -> i64 {
        self.jwt_expiration_hours * 3600
    }
}

/// AppState is the shared application state available to all handlers
/// It's Arc-wrapped internally by Axum for efficient, thread-safe sharing
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub xdr_config: XdrConfig,
    pub soroban_manager: Option<Arc<ScalableContractManager>>,
    pub pool: Pool<Postgres>,
}

// Implement FromRef to allow extracting Config from AppState
impl FromRef<AppState> for AppConfig {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

// Implement FromRef to allow extracting Pool from AppState
impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl AppState {
    pub fn new(config: AppConfig, pool: Pool<Postgres>) -> crate::error::Result<Self> {
        config.validate()?;

        // Create XdrConfig from AppConfig to avoid duplication
        let xdr_config = XdrConfig {
            contract_id: config.contract_id.clone(),
            network_passphrase: config.network_passphrase.clone(),
            rpc_url: config.rpc_url.clone(),
        };

        Ok(Self {
            config,
            xdr_config,
            soroban_manager: None,
            pool,
        })
    }

    /// Create AppState with ScalableContractManager for advanced features
    pub async fn with_soroban_manager(config: AppConfig, pool: Pool<Postgres>) -> crate::error::Result<Self> {
        config.validate()?;

        let xdr_config = XdrConfig {
            contract_id: config.contract_id.clone(),
            network_passphrase: config.network_passphrase.clone(),
            rpc_url: config.rpc_url.clone(),
        };

        // Initialize the ScalableContractManager
        let manager = ScalableContractManager::new().await?;

        Ok(Self {
            config,
            xdr_config,
            soroban_manager: Some(Arc::new(manager)),
            pool,
        })
    }
}