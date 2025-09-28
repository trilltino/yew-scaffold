/// Simplified Stellar Soroban dApp with Freighter integration
///
/// This is a clean, focused implementation that provides:
/// - Freighter wallet connection
/// - XDR generation and signing for hello_yew contract
/// - Clean, modern UI without unnecessary complexity
/// - Modular component architecture with routing

use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use serde::{Deserialize, Serialize};

mod components;
use components::{Navigation, WalletSection, ContractSection, AboutPage};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

/// Backend response for XDR generation
#[derive(Debug, Deserialize)]
struct XdrResponse {
    success: bool,
    xdr: String,
    message: String,
}

/// Backend response for transaction submission
#[derive(Debug, Deserialize)]
struct SubmitResponse {
    success: bool,
    result: String,
    transaction_hash: String,
    message: String,
}

/// Freighter wallet API access
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "freighterApi"], js_name = requestAccess)]
    fn freighter_request_access() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "freighterApi"], js_name = getAddress)]
    fn freighter_get_address() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "freighterApi"], js_name = signTransaction)]
    fn freighter_sign_transaction(xdr: &str, network: &str) -> js_sys::Promise;
}

/// Application state
#[derive(Debug, Clone, PartialEq)]
struct AppState {
    wallet_address: Option<String>,
    is_connecting: bool,
    result_message: String,
    is_processing: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            wallet_address: None,
            is_connecting: false,
            result_message: "Ready to connect wallet".to_string(),
            is_processing: false,
        }
    }
}

/// Application messages
#[derive(Debug, Clone)]
enum AppMessage {
    ConnectWallet,
    WalletConnected(String),
    WalletConnectionFailed(String),
    SignTransaction,
    TransactionResult(String),
    ProcessingStateChanged(bool),
}

/// Main application component with routing
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <style>
                {include_str!("styles.css")}
            </style>
            <div class="app">
                <Navigation />
                <Switch<Route> render={switch} />
                <footer class="footer">
                    <p>{"Built with "}<strong>{"Yew"}</strong>{" & "}<strong>{"Stellar"}</strong></p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

/// Route switching logic
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
    }
}

/// Home page component with wallet and contract functionality
#[function_component(HomePage)]
fn home_page() -> Html {
    let state = use_reducer(|| AppState::default());

    let on_connect_wallet = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(AppMessage::ConnectWallet);
        })
    };

    let on_sign_transaction = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(AppMessage::SignTransaction);
        })
    };

    // Handle wallet connection
    use_effect_with(state.clone(), {
        let state = state.clone();
        move |current_state| {
            if current_state.is_connecting {
                spawn_local(async move {
                    match connect_freighter().await {
                        Ok(address) => {
                            state.dispatch(AppMessage::WalletConnected(address));
                        }
                        Err(error) => {
                            state.dispatch(AppMessage::WalletConnectionFailed(error));
                        }
                    }
                });
            }
        }
    });

    // Handle transaction signing
    use_effect_with(state.clone(), {
        let state = state.clone();
        move |current_state| {
            if current_state.is_processing {
                let wallet_address = current_state.wallet_address.clone();
                spawn_local(async move {
                    let result = if let Some(address) = wallet_address {
                        sign_hello_transaction(&address).await
                    } else {
                        "No wallet connected".to_string()
                    };
                    state.dispatch(AppMessage::TransactionResult(result));
                });
            }
        }
    });

    html! {
        <>
            <header class="header">
                <h1>{"🚀 Stellar Soroban dApp"}</h1>
                <p>{"Simple, secure interaction with hello_yew contract using Freighter wallet"}</p>
            </header>

            <main class="main">
                <WalletSection
                    wallet_address={state.wallet_address.clone()}
                    is_connecting={state.is_connecting}
                    on_connect_wallet={on_connect_wallet}
                />

                <ContractSection
                    wallet_address={state.wallet_address.clone()}
                    is_processing={state.is_processing}
                    result_message={state.result_message.clone()}
                    on_sign_transaction={on_sign_transaction}
                />
            </main>
        </>
    }
}

impl Reducible for AppState {
    type Action = AppMessage;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AppMessage::ConnectWallet => Self {
                is_connecting: true,
                result_message: "Connecting to Freighter...".to_string(),
                ..(*self).clone()
            }.into(),
            AppMessage::WalletConnected(address) => Self {
                wallet_address: Some(address.clone()),
                is_connecting: false,
                result_message: format!("Wallet connected: {}...{}", &address[..6], &address[address.len()-6..]),
                ..(*self).clone()
            }.into(),
            AppMessage::WalletConnectionFailed(error) => Self {
                is_connecting: false,
                result_message: format!("Connection failed: {}", error),
                ..(*self).clone()
            }.into(),
            AppMessage::SignTransaction => Self {
                is_processing: true,
                result_message: "Generating XDR and signing transaction...".to_string(),
                ..(*self).clone()
            }.into(),
            AppMessage::TransactionResult(result) => Self {
                is_processing: false,
                result_message: result,
                ..(*self).clone()
            }.into(),
            AppMessage::ProcessingStateChanged(processing) => Self {
                is_processing: processing,
                ..(*self).clone()
            }.into(),
        }
    }
}

/// Connect to Freighter wallet
async fn connect_freighter() -> Result<String, String> {
    console::log_1(&"Starting Freighter connection...".into());

    // Request access
    let access_result = wasm_bindgen_futures::JsFuture::from(freighter_request_access()).await;

    if let Err(e) = &access_result {
        console::log_1(&format!("Access request failed: {:?}", e).into());
        return Err("Access denied by user or Freighter not installed. Please install from https://freighter.app".to_string());
    }

    console::log_1(&"Access granted, getting address...".into());

    // Get address
    let address_result = wasm_bindgen_futures::JsFuture::from(freighter_get_address()).await;

    match address_result {
        Ok(address) => {
            console::log_1(&format!("Address response: {:?}", address).into());

            // Freighter returns an object with an "address" property
            let address_obj = js_sys::Object::from(address);
            let address_value = js_sys::Reflect::get(&address_obj, &"address".into())
                .unwrap_or(JsValue::UNDEFINED);

            let address_str = address_value.as_string().unwrap_or_default();

            if address_str.is_empty() {
                console::log_1(&"Address string is empty".into());
                Err("Failed to get wallet address".to_string())
            } else {
                console::log_1(&format!("Got address: {}", address_str).into());
                Ok(address_str)
            }
        }
        Err(e) => {
            console::log_1(&format!("Get address error: {:?}", e).into());
            Err("Failed to get wallet address from Freighter".to_string())
        }
    }
}

/// Submit signed transaction to backend and get contract result
async fn submit_signed_transaction(signed_xdr: &str) -> Result<String, String> {
    #[derive(Serialize)]
    struct SubmitRequest {
        signed_xdr: String,
    }

    let payload = SubmitRequest {
        signed_xdr: signed_xdr.to_string(),
    };

    let response = Request::post("http://127.0.0.1:3001/submit-transaction")
        .json(&payload)
        .map_err(|e| format!("Failed to serialize request: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Network request failed: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Backend error: HTTP {}", response.status()));
    }

    let submit_response: SubmitResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    if submit_response.success {
        Ok(format!("🎉 Contract executed successfully!\n\n📋 Contract Result: {}\n🔗 Transaction Hash: {}\n\n💬 {}",
            submit_response.result,
            submit_response.transaction_hash,
            submit_response.message))
    } else {
        Err(submit_response.message)
    }
}

/// Sign hello_yew transaction
async fn sign_hello_transaction(source_account: &str) -> String {
    // Step 1: Generate XDR via backend
    let xdr = match generate_xdr(source_account).await {
        Ok(xdr) => xdr,
        Err(error) => return format!("❌ XDR generation failed: {}", error),
    };

    console::log_1(&"✅ XDR generated, now signing with Freighter...".into());

    // Step 2: Sign with Freighter
    // Use the correct network format for Freighter (TESTNET or PUBLIC)
    let network = "TESTNET";
    console::log_1(&format!("🔐 Signing transaction with network: {}", network).into());
    let sign_result = wasm_bindgen_futures::JsFuture::from(freighter_sign_transaction(&xdr, network)).await;

    let signed_xdr = match sign_result {
        Ok(signed_response) => {
            // Log the full response structure for debugging
            console::log_1(&"🔍 Freighter response received:".into());
            console::log_1(&signed_response);

            // Check for error first
            if let Ok(error_val) = js_sys::Reflect::get(&signed_response, &"error".into()) {
                if !error_val.is_undefined() && !error_val.is_null() {
                    console::log_1(&"❌ Freighter returned an error:".into());
                    console::log_1(&error_val);
                    return "❌ Transaction signing failed. Check console for error details.".to_string();
                }
            }

            // Try multiple possible property names for the signed XDR
            let signed_str = if let Ok(xdr_val) = js_sys::Reflect::get(&signed_response, &"signedTxXdr".into()) {
                // Try signedTxXdr (current attempt)
                xdr_val.as_string().unwrap_or_default()
            } else if let Ok(xdr_val) = js_sys::Reflect::get(&signed_response, &"xdr".into()) {
                // Try xdr property
                xdr_val.as_string().unwrap_or_default()
            } else if let Ok(xdr_val) = js_sys::Reflect::get(&signed_response, &"signedXdr".into()) {
                // Try signedXdr property
                xdr_val.as_string().unwrap_or_default()
            } else if let Ok(xdr_val) = js_sys::Reflect::get(&signed_response, &"result".into()) {
                // Try result property
                xdr_val.as_string().unwrap_or_default()
            } else if signed_response.is_string() {
                // Maybe the response is directly the XDR string
                signed_response.as_string().unwrap_or_default()
            } else {
                // Log available properties for debugging
                console::log_1(&"❌ Could not find XDR in response. Available properties:".into());
                let object = js_sys::Object::from(signed_response.clone());
                let keys = js_sys::Object::keys(&object);
                console::log_1(&keys);
                String::new()
            };

            if signed_str.is_empty() {
                return "❌ No signed XDR found in Freighter response. Check console for details.".to_string();
            }

            console::log_1(&format!("✅ Transaction signed! XDR length: {}", signed_str.len()).into());
            signed_str
        }
        Err(js_error) => {
            console::log_1(&"❌ Freighter signing error:".into());
            console::log_1(&js_error);
            return "❌ Transaction signing failed or was cancelled. Check console for details.".to_string();
        }
    };

    // Step 3: Submit signed transaction to backend
    console::log_1(&"📤 Submitting signed transaction to backend...".into());

    match submit_signed_transaction(&signed_xdr).await {
        Ok(result) => {
            console::log_1(&"✅ Transaction submitted successfully!".into());
            result
        }
        Err(error) => {
            console::log_1(&format!("❌ Transaction submission failed: {}", error).into());
            format!("❌ Transaction submission failed: {}", error)
        }
    }
}

/// Generate XDR via backend service
async fn generate_xdr(source_account: &str) -> Result<String, String> {
    let url = format!("http://127.0.0.1:3001/generate-xdr?source_account={}", source_account);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Backend error: HTTP {}", response.status()));
    }

    let xdr_response: XdrResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    if xdr_response.success {
        Ok(xdr_response.xdr)
    } else {
        Err(xdr_response.message)
    }
}

fn main() {
    // Initialize console logging for development
    console_error_panic_hook::set_once();

    console::log_1(&"🚀 Starting Stellar Soroban dApp with routing".into());

    yew::Renderer::<App>::new().render();
}