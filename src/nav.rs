use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use wasm_bindgen::JsValue;
use crate::freighter::{is_freighter_available, connect_wallet, FreighterError};
use crate::helloworld_bindings::call_hello_contract;

#[derive(Properties, PartialEq, Clone)]
pub struct NavProps {
    pub contract_result: UseStateHandle<String>,
    pub wallet_result: UseStateHandle<String>,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let nav_class = "display:flex;justify-content:space-between;align-items:center;background-color:#3d2f1f;font-family:'Fira Sans',Helvetica,Arial,sans-serif;margin:0;padding:10px 20px;";
    let wallet_address = use_state(|| "Not connected".to_string());
    let wallet_status = use_state(|| "disconnected".to_string()); // For styling different states


    let connect_wallet_click = {
        let wallet_address = wallet_address.clone();
        let wallet_status = wallet_status.clone();
        let wallet_result = props.wallet_result.clone();
        Callback::from(move |_| {
            let wallet_address = wallet_address.clone();
            let wallet_status = wallet_status.clone();
            let wallet_result = wallet_result.clone();
            
            if !is_freighter_available() {
                wallet_address.set("Freighter not installed".to_string());
                wallet_status.set("error".to_string());
                wallet_result.set("Freighter wallet extension not found".to_string());
                return;
            }
            
            wallet_address.set("Connecting...".to_string());
            wallet_status.set("connecting".to_string());
            wallet_result.set("Connecting to Freighter wallet...".to_string());
            
            spawn_local(async move {
                match connect_wallet().await {
                    Ok(address) => {
                        console::log_1(&JsValue::from_str(&format!("Connected: {}", address)));
                        wallet_address.set(format!("{}...{}", &address[..4], &address[address.len()-4..]));
                        wallet_status.set("connected".to_string());
                        wallet_result.set(format!("Wallet connected: {}", address));
                    }
                    Err(FreighterError::UserRejected) => {
                        wallet_address.set("User rejected".to_string());
                        wallet_status.set("error".to_string());
                        wallet_result.set("User rejected the wallet connection request".to_string());
                        console::log_1(&JsValue::from_str("User rejected wallet connection"));
                    }
                    Err(FreighterError::FreighterExtNotFound) => {
                        wallet_address.set("Install Freighter".to_string());
                        wallet_status.set("error".to_string());
                        wallet_result.set("Freighter wallet extension not found. Install from https://freighter.app/".to_string());
                    }
                    Err(e) => {
                        let error_msg = format!("Error: {}", e);
                        console::log_1(&JsValue::from_str(&error_msg));
                        wallet_address.set("Connection failed".to_string());
                        wallet_status.set("error".to_string());
                        wallet_result.set(format!("Wallet connection failed: {}", error_msg));
                    }
                }
            });
        })
    };

    let stellar_contract_click = {
        let contract_result = props.contract_result.clone();
        Callback::from(move |_| {
            let contract_result = contract_result.clone();
            spawn_local(async move {
                console::log_1(&JsValue::from_str("Calling Stellar contract"));
                match call_hello_contract().await {
                    Ok(result) => contract_result.set(format!("Stellar Contract: {}", result)),
                    Err(e) => contract_result.set(format!("Stellar Error: {:?}", e)),
                }
            });
        })
    };

    let wallet_text_color = match wallet_status.as_str() {
        "connected" => "#4CAF50",
        "connecting" => "#FF9800", 
        "error" => "#f44336",
        _ => "#f5f3f0",
    };

    let button_text = match wallet_status.as_str() {
        "connected" => "Connected",
        "connecting" => "Connecting...",
        _ => "Connect Freighter",
    };

    let button_style = match wallet_status.as_str() {
        "connected" => "padding:8px 16px;background:#4CAF50;color:white;border:none;border-radius:4px;cursor:pointer;font-family:'Fira Sans',Helvetica,Arial,sans-serif;",
        "connecting" => "padding:8px 16px;background:#FF9800;color:white;border:none;border-radius:4px;cursor:not-allowed;font-family:'Fira Sans',Helvetica,Arial,sans-serif;",
        _ => "padding:8px 16px;background:#4CAF50;color:white;border:none;border-radius:4px;cursor:pointer;font-family:'Fira Sans',Helvetica,Arial,sans-serif;",
    };

    html! {
        <nav style={nav_class}>
            <div>
                <img src="/static/logo.png" alt="Logo" height="36"/>
            </div>
            <div style={"display:flex;gap:15px;align-items:center;"}>
                <div style={"display:flex;align-items:center;gap:10px;"}>
                    <button
                        onclick={connect_wallet_click}
                        disabled={*wallet_status == "connecting"}
                        style={button_style}
                    >
                        {button_text}
                    </button>
                    <span style={format!("font-size:12px;max-width:150px;overflow:hidden;text-overflow:ellipsis;font-family:'Fira Sans',Helvetica,Arial,sans-serif;color:{};", wallet_text_color)}>
                        {(*wallet_address).clone()}
                    </span>
                </div>
                <div style={"display:flex;align-items:center;gap:10px;"}>
                    <button
                        onclick={stellar_contract_click}
                        style={"padding:8px 16px;background:#9C27B0;color:white;border:none;border-radius:4px;cursor:pointer;font-family:'Fira Sans',Helvetica,Arial,sans-serif;"}
                    >
                        {"Call Contract"}
                    </button>
                </div>
            </div>
        </nav>
    }
}