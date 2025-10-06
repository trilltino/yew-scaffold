/// Idiomatic Freighter wallet integration for Stellar dApps
///
/// This module provides a clean, type-safe interface for Freighter wallet
/// connection and transaction signing using direct WASM bindings.
pub mod freighter;

pub use freighter::{is_freighter_available, connect_wallet, sign_transaction};

/// Simple wallet type for Freighter only
#[derive(Debug, Clone, PartialEq)]
pub enum WalletType {
    Freighter,
}

impl std::fmt::Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Freighter")
    }
}

/// Connection status for wallet
#[derive(Debug, Clone, PartialEq)]
pub enum WalletStatus {
    Connected(String), // Contains the wallet name
}

/// Represents a connected Freighter wallet
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectedWallet {
    pub wallet_type: WalletType,
    pub address: String,
    pub status: WalletStatus,
}