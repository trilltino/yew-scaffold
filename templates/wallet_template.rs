/// Template for new wallet implementation
///
/// To create a new wallet:
/// 1. Copy this file to `frontend/src/wallet/newwallet.rs`
/// 2. Replace WALLET_NAME with actual wallet name
/// 3. Implement the browser extension API bindings
/// 4. Add to `wallet/mod.rs` enum and manager
/// 5. Update navigation component

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

use super::{StellarWallet, WalletType, WalletResult, WalletError};

/// WALLET_NAME wallet API bindings
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "WALLET_API_NAMESPACE"], js_name = isConnected)]
    fn wallet_is_connected() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "WALLET_API_NAMESPACE"], js_name = getPublicKey)]
    fn wallet_get_public_key() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "WALLET_API_NAMESPACE"], js_name = signTransaction)]
    fn wallet_sign_transaction(xdr: &str) -> js_sys::Promise;
}

/// Check if WALLET_NAME API is available
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window"], js_name = WALLET_API_NAMESPACE)]
    static WALLET_API: JsValue;
}

/// WALLET_NAME wallet implementation
pub struct WalletNameWallet;

impl WalletNameWallet {
    pub fn new() -> Self {
        Self
    }

    /// Validate Stellar address format
    fn validate_stellar_address(address: &str) -> WalletResult<()> {
        if address.is_empty() {
            return Err(WalletError::InvalidAddress("Address is empty".to_string()));
        }

        if !address.starts_with('G') || address.len() != 56 {
            return Err(WalletError::InvalidAddress(
                format!("Invalid Stellar address format: {}", address)
            ));
        }

        Ok(())
    }

    /// Extract address from wallet response
    fn extract_address(response: JsValue) -> WalletResult<String> {
        let address = response.as_string()
            .ok_or_else(|| WalletError::InvalidAddress("Response is not a string".to_string()))?;

        Self::validate_stellar_address(&address)?;
        Ok(address)
    }

    /// Extract signed XDR from wallet response
    fn extract_signed_xdr(response: JsValue) -> WalletResult<String> {
        console::log_1(&"🔍 WALLET_NAME signing response received:".into());
        console::log_1(&response);

        let signed_xdr = response.as_string()
            .ok_or_else(|| WalletError::SigningFailed(
                WalletType::WALLET_ENUM_VARIANT,
                "Signing response is not a string".to_string()
            ))?;

        if signed_xdr.is_empty() {
            return Err(WalletError::SigningFailed(
                WalletType::WALLET_ENUM_VARIANT,
                "Returned empty signed XDR".to_string()
            ));
        }

        Ok(signed_xdr)
    }
}

#[async_trait::async_trait(?Send)]
impl StellarWallet for WalletNameWallet {
    fn wallet_type(&self) -> WalletType {
        WalletType::WALLET_ENUM_VARIANT
    }

    async fn is_available(&self) -> bool {
        if WALLET_API.is_undefined() || WALLET_API.is_null() {
            console::log_1(&"🔍 WALLET_NAME API not found in window object".into());
            return false;
        }

        match JsFuture::from(wallet_is_connected()).await {
            Ok(connected) => {
                let is_available = connected.as_bool().unwrap_or(false);
                console::log_1(&format!("🔍 WALLET_NAME extension available: {}", is_available).into());
                true
            }
            Err(_) => {
                console::log_1(&"🔍 WALLET_NAME extension not properly installed".into());
                false
            }
        }
    }

    async fn connect(&self) -> WalletResult<String> {
        console::log_1(&"🚀 Starting WALLET_NAME wallet connection...".into());

        if WALLET_API.is_undefined() || WALLET_API.is_null() {
            return Err(WalletError::NotInstalled(WalletType::WALLET_ENUM_VARIANT));
        }

        let key_result = JsFuture::from(wallet_get_public_key()).await;

        match key_result {
            Ok(key_response) => {
                match Self::extract_address(key_response) {
                    Ok(address) => {
                        console::log_1(&format!("✅ Successfully connected to WALLET_NAME: {}...{}",
                                               &address[..6], &address[address.len()-6..]).into());
                        Ok(address)
                    }
                    Err(e) => Err(e)
                }
            }
            Err(_) => {
                Err(WalletError::ConnectionFailed(
                    WalletType::WALLET_ENUM_VARIANT,
                    "Failed to get WALLET_NAME public key".to_string()
                ))
            }
        }
    }

    async fn get_address(&self) -> WalletResult<String> {
        if !self.is_available().await {
            return Err(WalletError::NotInstalled(WalletType::WALLET_ENUM_VARIANT));
        }

        let key_result = JsFuture::from(wallet_get_public_key()).await;

        match key_result {
            Ok(key_response) => Self::extract_address(key_response),
            Err(_) => Err(WalletError::ConnectionFailed(
                WalletType::WALLET_ENUM_VARIANT,
                "Failed to get current address".to_string()
            )),
        }
    }

    async fn sign_transaction(&self, xdr: &str, network: &str) -> WalletResult<String> {
        console::log_1(&"🔐 Signing transaction with WALLET_NAME".into());

        if !self.is_available().await {
            return Err(WalletError::NotInstalled(WalletType::WALLET_ENUM_VARIANT));
        }

        let sign_result = JsFuture::from(wallet_sign_transaction(xdr)).await;

        match sign_result {
            Ok(signed_response) => {
                match Self::extract_signed_xdr(signed_response) {
                    Ok(signed_xdr) => {
                        console::log_1(&"✅ Transaction signed successfully with WALLET_NAME!".into());
                        Ok(signed_xdr)
                    }
                    Err(e) => Err(e)
                }
            }
            Err(js_error) => {
                console::log_1(&"❌ WALLET_NAME signing error:".into());
                console::log_1(&js_error);

                let error_str = format!("{:?}", js_error);
                if error_str.contains("reject") || error_str.contains("cancel") {
                    Err(WalletError::UserRejected(WalletType::WALLET_ENUM_VARIANT))
                } else {
                    Err(WalletError::SigningFailed(
                        WalletType::WALLET_ENUM_VARIANT,
                        "Transaction signing failed".to_string()
                    ))
                }
            }
        }
    }

    async fn disconnect(&self) -> WalletResult<()> {
        console::log_1(&"ℹ️ WALLET_NAME wallet disconnected".into());
        Ok(())
    }

    fn display_name(&self) -> &'static str {
        "WALLET_NAME"
    }

    fn install_url(&self) -> &'static str {
        "https://example.com/install-wallet"
    }
}

impl Default for WalletNameWallet {
    fn default() -> Self {
        Self::new()
    }
}