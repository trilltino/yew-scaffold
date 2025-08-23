use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use wasm_bindgen::JsValue;
use crate::freighter::{is_freighter_available, connect_wallet, get_public_key, FreighterError};

#[derive(Clone, PartialEq)]
pub struct WalletState {
    pub address: Option<String>,
    pub status: &'static str,        // "disconnected" | "connecting" | "connected" | "error"
    pub last_message: String,
}

#[derive(Clone, PartialEq)]
pub struct WalletCtx {
    pub state: UseStateHandle<WalletState>,
    pub connect: Callback<()>,
}

#[hook]
pub fn use_wallet() -> WalletCtx {
    let state = use_state(|| WalletState {
        address: None,
        status: "disconnected",
        last_message: "No wallet connected".to_string(),
    });

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if is_freighter_available() {
                    match get_public_key().await {
                        Ok(addr) => {
                            let short = format!("{}...{}", &addr[..4], &addr[addr.len()-4..]);
                            state.set(WalletState {
                                address: Some(short),
                                status: "connected",
                                last_message: format!("Connected to Freighter: {}", addr),
                            });
                        }
                        Err(_) => {
                            state.set(WalletState {
                                address: None,
                                status: "disconnected",
                                last_message: "Freighter available but not connected".to_string(),
                            });
                        }
                    }
                } else {
                    state.set(WalletState {
                        address: None,
                        status: "error",
                        last_message: "Freighter wallet extension not found".to_string(),
                    });
                }
            });
            || ()
        });
    }

    let connect = {
        let state = state.clone();
        Callback::from(move |_| {
            if state.status == "connected" || state.status == "connecting" {
                return;
            }
            if !is_freighter_available() {
                state.set(WalletState {
                    address: Some("Freighter not installed".to_string()),
                    status: "error",
                    last_message: "Freighter wallet extension not found".to_string(),
                });
                return;
            }
            state.set(WalletState {
                address: Some("Connecting...".to_string()),
                status: "connecting",
                last_message: "Connecting to Freighter wallet...".to_string(),
            });
            let state2 = state.clone();
            spawn_local(async move {
                match connect_wallet().await {
                    Ok(addr) => {
                        let short = format!("{}...{}", &addr[..4], &addr[addr.len()-4..]);
                        state2.set(WalletState {
                            address: Some(short),
                            status: "connected",
                            last_message: format!("Successfully connected to Freighter: {}", addr),
                        });
                        console::log_1(&JsValue::from_str(&format!("Connected to wallet: {}", addr)));
                    }
                    Err(FreighterError::UserRejected) => {
                        state2.set(WalletState {
                            address: Some("User rejected".to_string()),
                            status: "error",
                            last_message: "User rejected the wallet connection request".to_string(),
                        });
                    }
                    Err(FreighterError::FreighterExtNotFound) => {
                        state2.set(WalletState {
                            address: Some("Install Freighter".to_string()),
                            status: "error",
                            last_message: "Freighter wallet extension not found. Install from https://freighter.app/".to_string(),
                        });
                    }
                    Err(e) => {
                        state2.set(WalletState {
                            address: Some("Connection failed".to_string()),
                            status: "error",
                            last_message: format!("Connection failed: {:?}", e),
                        });
                    }
                }
            });
        })
    };

    WalletCtx { state, connect }
}
