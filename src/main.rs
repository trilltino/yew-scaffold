use crate::freighter::{connect_wallet, is_freighter_available};
use crate::helloworld_bindings::HelloContract;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

mod freighter;
mod helloworld_bindings;

#[function_component(App)]
fn app() -> Html {
    let connected_address = use_state(|| Option::<String>::None);
    let contract_result = use_state(|| Option::<String>::None); // Changed to String for minimal test
    let loading = use_state(|| false);

    // Connect wallet button (unchanged)
    let connect_onclick = {
        let connected_address = connected_address.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let connected_address = connected_address.clone();
            let loading = loading.clone();

            if !is_freighter_available() {
                console::log_1(&JsValue::from_str("Freighter wallet extension not found"));
                return;
            }

            loading.set(true);
            spawn_local(async move {
                match connect_wallet().await {
                    Ok(public_key) => {
                        console::log_1(&JsValue::from_str(&format!("Connected: {}", public_key)));
                        connected_address.set(Some(public_key));
                    }
                    Err(e) => {
                        console::log_1(&JsValue::from_str(&format!("Connection failed: {:?}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Simple contract test button
    let contract_onclick = {
        let contract_result = contract_result.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let contract_result = contract_result.clone();
            let loading = loading.clone();

            loading.set(true);
            spawn_local(async move {
                console::log_1(&JsValue::from_str("🚀 Testing contract call..."));

                let contract = HelloContract::new(); // No Result wrapping in minimal version
                let result = contract.call_hello("World").await; // Returns String directly

                console::log_1(&JsValue::from_str(&format!(
                    "✅ Contract returned: {}",
                    result
                )));
                contract_result.set(Some(result));

                loading.set(false);
            });
        })
    };

    // Custom message test
    let custom_onclick = {
        let contract_result = contract_result.clone();
        let loading = loading.clone();
        let connected_address = connected_address.clone();

        Callback::from(move |_| {
            let contract_result = contract_result.clone();
            let loading = loading.clone();
            let connected_address = connected_address.clone();

            // Use wallet address if connected, otherwise use "Stranger"
            let to_message = if let Some(address) = (*connected_address).clone() {
                format!("{}...", &address[..8.min(address.len())])
            } else {
                "Stranger".to_string()
            };

            loading.set(true);
            spawn_local(async move {
                console::log_1(&JsValue::from_str(&format!(
                    "🚀 Testing contract with: {}",
                    to_message
                )));

                let contract = HelloContract::new();
                let result = contract.call_hello(&to_message).await;

                console::log_1(&JsValue::from_str(&format!(
                    "✅ Contract returned: {}",
                    result
                )));
                contract_result.set(Some(result));

                loading.set(false);
            });
        })
    };

    html! {
        <div style="padding: 20px; font-family: Arial, sans-serif; max-width: 600px;">
            <h1>{ "Stellar DApp Demo - Minimal Test" }</h1>

            // Wallet Section (unchanged)
            <div style="margin-bottom: 30px; padding: 20px; border: 1px solid #ddd; border-radius: 8px;">
                <h2>{ "🔗 Wallet Connection" }</h2>
                <button
                    onclick={connect_onclick}
                    disabled={*loading}
                    style="
                        padding: 12px 24px; 
                        font-size: 16px; 
                        background-color: #4CAF50; 
                        color: white; 
                        border: none; 
                        border-radius: 6px; 
                        cursor: pointer;
                        margin-right: 10px;
                    "
                >
                    { "Connect Freighter Wallet" }
                </button>

                {
                    if let Some(address) = (*connected_address).clone() {
                        html! {
                            <div style="margin-top: 10px;">
                                <p style="color: green;"><strong>{ "✅ Connected:" }</strong></p>
                                <code style="background: #f0f0f0; padding: 5px; border-radius: 3px; word-break: break-all;">
                                    { address }
                                </code>
                            </div>
                        }
                    } else {
                        html! { <p style="color: #666;">{ "❌ Not connected" }</p> }
                    }
                }
            </div>

            // Contract Section (simplified)
            <div style="margin-bottom: 30px; padding: 20px; border: 1px solid #ddd; border-radius: 8px;">
                <h2>{ "🚀 Contract Test" }</h2>

                <div style="margin-bottom: 15px;">
                    <button
                        onclick={contract_onclick}
                        disabled={*loading}
                        style="
                            padding: 12px 24px; 
                            font-size: 16px; 
                            background-color: #2196F3; 
                            color: white; 
                            border: none; 
                            border-radius: 6px; 
                            cursor: pointer;
                            margin-right: 10px;
                        "
                    >
                        { if *loading { "Testing..." } else { "Test Contract" } }
                    </button>

                    <button
                        onclick={custom_onclick}
                        disabled={*loading}
                        style="
                            padding: 12px 24px; 
                            font-size: 16px; 
                            background-color: #FF9800; 
                            color: white; 
                            border: none; 
                            border-radius: 6px; 
                            cursor: pointer;
                        "
                    >
                        { if *loading { "Testing..." } else { "Test with Address" } }
                    </button>
                </div>

                {
                    if let Some(result) = (*contract_result).clone() {
                        html! {
                            <div style="
                                background-color: #e8f5e8; 
                                padding: 15px; 
                                border-radius: 6px; 
                                margin-top: 15px;
                                border-left: 4px solid #4caf50;
                            ">
                                <h3>{ "📤 Contract Response:" }</h3>
                                <div style="font-family: monospace; font-size: 16px; background: #f0f0f0; padding: 10px; border-radius: 4px;">
                                    { result }
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <p style="color: #666; font-style: italic;">
                                { "No contract calls yet. Click a button above to test!" }
                            </p>
                        }
                    }
                }
            </div>

            // Status Section
            {
                if *loading {
                    html! {
                        <div style="
                            background-color: #fff3cd; 
                            padding: 15px; 
                            border-radius: 6px;
                            border-left: 4px solid #ffc107;
                        ">
                            <p>{ "⏳ Testing... Check console for details" }</p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

