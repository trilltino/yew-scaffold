use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use wasm_bindgen::JsValue;
use crate::wallet::WalletCtx;
use crate::helloworld_bindings::call_hello_contract;

#[derive(Properties, PartialEq, Clone)]
pub struct NavProps {
    pub contract_result: UseStateHandle<String>,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let nav_style = "display:flex;justify-content:space-between;align-items:center;background-color:#3d2f1f;font-family:'Fira Sans',Helvetica,Arial,sans-serif;margin:0;padding:10px 20px;";
    let wallet = use_context::<WalletCtx>().expect("WalletCtx missing");

    let wallet_text_color = match wallet.state.status {
        "connected" => "#4CAF50",
        "connecting" => "#FF9800",
        "error" => "#f44336",
        _ => "#f5f3f0",
    };

    let button_text = match wallet.state.status {
        "connected" => "Connected",
        "connecting" => "Connecting...",
        _ => "Connect Freighter",
    };

    let button_style = match wallet.state.status {
        "connected" => "padding:8px 16px;background:#4CAF50;color:white;border:none;border-radius:4px;cursor:default;font-family:'Fira Sans',Helvetica,Arial,sans-serif;",
        "connecting" => "padding:8px 16px;background:#FF9800;color:white;border:none;border-radius:4px;cursor:not-allowed;font-family:'Fira Sans',Helvetica,Arial,sans-serif;",
        "error" => "padding:8px 16px;background:#f44336;color:white;border:none;border-radius:4px;cursor:pointer;font-family:'Fira Sans',Helvetica,Arial,sans-serif;",
        _ => "padding:8px 16px;background:#4CAF50;color:white;border:none;border-radius:4px;cursor:pointer;font-family:'Fira Sans',Helvetica,Arial,sans-serif;",
    };

    let call_contract = {
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

    html! {
        <nav style={nav_style}>
            <div style="display:flex;align-items:center;">
                <h1 style="color:#f5f3f0;margin:0;font-size:24px;">{"Stellar dApp"}</h1>
            </div>
            <div style="display:flex;align-items:center;gap:15px;">
                <span style={format!("color:{};font-size:14px;", wallet_text_color)}>
                    { wallet.state.address.clone().unwrap_or_else(|| "Not connected".to_string()) }
                </span>
                <button
                    style={button_style}
                    onclick={Callback::from(move |_| wallet.connect.emit(()))}
                    disabled={wallet.state.status == "connecting"}
                >
                    { button_text }
                </button>
                <button
                    style="padding:8px 16px;background:#2196F3;color:white;border:none;border-radius:4px;cursor:pointer;font-family:'Fira Sans',Helvetica,Arial,sans-serif;"
                    onclick={call_contract}
                >
                    {"Call Contract"}
                </button>
            </div>
        </nav>
    }
}
