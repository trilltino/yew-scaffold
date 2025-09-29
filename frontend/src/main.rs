/// Simple Stellar Soroban dApp with Freighter wallet integration
///
/// This is a clean, focused implementation that provides:
/// - Direct Freighter wallet connection
/// - XDR generation and signing for hello_yew contract
/// - Clean, modern UI without unnecessary complexity
/// - Simple component architecture with routing

use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

mod components;
mod wallet;
mod types;

use components::{Navigation, ContractSection, AboutPage};
use wallet::{WalletType, ConnectedWallet, WalletStatus, is_freighter_available, connect_wallet, sign_transaction};
use types::ContractFunction;

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


/// Simple application state for Freighter wallet
#[derive(Clone)]
struct AppState {
    connected_wallet: Option<ConnectedWallet>,
    is_connecting: bool,
    result_message: String,
    is_processing: bool,
    selected_function: Option<ContractFunction>,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.connected_wallet == other.connected_wallet
            && self.is_connecting == other.is_connecting
            && self.result_message == other.result_message
            && self.is_processing == other.is_processing
            && self.selected_function == other.selected_function
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            connected_wallet: None,
            is_connecting: false,
            result_message: "Ready to connect with Freighter wallet".to_string(),
            is_processing: false,
            selected_function: None,
        }
    }
}

/// Simple application messages for Freighter wallet
#[derive(Debug, Clone)]
enum AppMessage {
    ConnectWallet,
    WalletConnected(ConnectedWallet),
    WalletConnectionFailed(String),
    DisconnectWallet,
    SelectFunction(ContractFunction),
    SignTransaction,
    TransactionResult(String),
}

/// Main application component with routing
#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| AppState::default());

    let on_connect_wallet = {
        let state = state.clone();
        Callback::from(move |_wallet_type: WalletType| {
            state.dispatch(AppMessage::ConnectWallet);
        })
    };

    let on_disconnect_wallet = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(AppMessage::DisconnectWallet);
        })
    };

    // Check if Freighter is available on component mount
    {
        use_effect_with((), move |_| {
            spawn_local(async move {
                let _ = is_freighter_available().await;
            });
            || {}
        });
    }

    html! {
        <BrowserRouter>
            <style>
                {include_str!("styles.css")}
            </style>
            <div class="app">
                <Navigation
                    connected_wallet={state.connected_wallet.clone()}
                    is_connecting={state.is_connecting}
                    on_connect_wallet={on_connect_wallet}
                    on_disconnect_wallet={on_disconnect_wallet}
                />
                <Switch<Route> render={switch_with_state(state.clone())} />
                <footer class="footer">
                    <p>{"Built with "}<strong>{"Yew"}</strong>{" & "}<strong>{"Stellar"}</strong></p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

/// Route switching logic with state
fn switch_with_state(state: yew::UseReducerHandle<AppState>) -> impl Fn(Route) -> Html {
    move |routes: Route| match routes {
        Route::Home => html! { <HomePage state={state.clone()} /> },
        Route::About => html! { <AboutPage /> },
    }
}

/// Home page component with wallet and contract functionality
#[derive(Properties, PartialEq)]
struct HomePageProps {
    state: yew::UseReducerHandle<AppState>,
}

#[function_component(HomePage)]
fn home_page(props: &HomePageProps) -> Html {
    let state = props.state.clone();

    let on_sign_transaction = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(AppMessage::SignTransaction);
        })
    };

    // Handle async operations (wallet connection and transaction signing)
    {
        let state = state.clone();
        use_effect_with((state.is_connecting, state.is_processing), {
            let state = state.clone();
            move |(is_connecting, is_processing)| {
                // Handle wallet connection
                if *is_connecting && state.connected_wallet.is_none() {
                    let state_inner = state.clone();
                    spawn_local(async move {
                        match connect_wallet().await {
                            Ok(address) => {
                                let connected_wallet = ConnectedWallet {
                                    wallet_type: WalletType::Freighter,
                                    address,
                                    status: WalletStatus::Connected("Freighter".to_string()),
                                };
                                state_inner.dispatch(AppMessage::WalletConnected(connected_wallet));
                            }
                            Err(err) => {
                                state_inner.dispatch(AppMessage::WalletConnectionFailed(err.to_string()));
                            }
                        }
                    });
                }

                // Handle transaction signing
                if *is_processing {
                    let connected_wallet = state.connected_wallet.clone();
                    let selected_function = state.selected_function.clone();
                    let state_inner = state.clone();
                    spawn_local(async move {
                        let result = if let (Some(wallet), Some(function)) = (connected_wallet, selected_function) {
                            sign_hello_transaction(&wallet, &function).await
                        } else if state.connected_wallet.is_none() {
                            "No wallet connected".to_string()
                        } else {
                            "No function selected".to_string()
                        };
                        state_inner.dispatch(AppMessage::TransactionResult(result));
                    });
                }
                || {}
            }
        });
    }

    html! {
        <main class="main">
            <ContractSection
                connected_wallet={state.connected_wallet.clone()}
                is_processing={state.is_processing}
                result_message={state.result_message.clone()}
                selected_function={state.selected_function.clone()}
                on_sign_transaction={on_sign_transaction}
                on_select_function={{
                    let state = state.clone();
                    Callback::from(move |function: ContractFunction| {
                        state.dispatch(AppMessage::SelectFunction(function));
                    })
                }}
            />
        </main>
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

            AppMessage::WalletConnected(connected_wallet) => Self {
                connected_wallet: Some(connected_wallet.clone()),
                is_connecting: false,
                result_message: format!("✅ Connected to {}...{}",
                                      &connected_wallet.address[..6],
                                      &connected_wallet.address[connected_wallet.address.len()-6..]),
                ..(*self).clone()
            }.into(),

            AppMessage::WalletConnectionFailed(error) => Self {
                is_connecting: false,
                result_message: format!("❌ Connection failed: {}", error),
                ..(*self).clone()
            }.into(),

            AppMessage::DisconnectWallet => Self {
                connected_wallet: None,
                result_message: "Wallet disconnected".to_string(),
                ..(*self).clone()
            }.into(),

            AppMessage::SelectFunction(function) => Self {
                selected_function: Some(function.clone()),
                result_message: format!("Selected function: {} - {}", function.display_name(), function.description()),
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

        }
    }
}



/// Submit signed transaction to backend and get contract result
async fn submit_signed_transaction(signed_xdr: &str, wallet_type: &str, function: &ContractFunction) -> Result<String, String> {
    #[derive(Serialize)]
    struct SubmitRequest {
        signed_xdr: String,
        wallet_type: Option<String>,
        function_name: Option<String>,
    }

    let payload = SubmitRequest {
        signed_xdr: signed_xdr.to_string(),
        wallet_type: Some(wallet_type.to_string()),
        function_name: Some(function.name().to_string()),
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

/// Sign transaction with Freighter
async fn sign_hello_transaction(connected_wallet: &ConnectedWallet, function: &ContractFunction) -> String {
    let source_account = &connected_wallet.address;

    // Step 1: Generate XDR via backend
    let xdr = match generate_xdr(source_account, "freighter", function).await {
        Ok(xdr) => xdr,
        Err(error) => return format!("XDR generation failed: {}", error),
    };

    // Step 2: Sign with Freighter
    let network = "Test SDF Network ; September 2015";
    let signed_xdr = match sign_transaction(&xdr, network).await {
        Ok(signed_xdr) => signed_xdr,
        Err(error) => return format!("Transaction signing failed: {}", error),
    };

    // Step 3: Submit signed transaction to backend
    match submit_signed_transaction(&signed_xdr, "freighter", function).await {
        Ok(result) => result,
        Err(error) => format!("Transaction submission failed: {}", error),
    }
}


/// Generate XDR via backend service
async fn generate_xdr(source_account: &str, wallet_type: &str, function: &ContractFunction) -> Result<String, String> {
    let function_name = function.name();

    let url = format!("http://127.0.0.1:3001/generate-xdr?source_account={}&wallet_type={}&function_name={}",
                     source_account, wallet_type, function_name);

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
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}