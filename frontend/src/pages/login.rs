use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::InputEvent;
use crate::wallet::{connect_wallet, WalletType, ConnectedWallet, WalletStatus};
use crate::services::ApiClient;
use crate::{Route, AppState, AppMessage};
use shared::dto::auth::Guest;

#[derive(Debug, Clone, PartialEq)]
enum LoginStep {
    EnterUsername,
    ConnectingWallet,
    Success,
}

#[derive(Debug, Clone, PartialEq)]
enum UsernameValidation {
    Empty,
    TooShort,
    TooLong,
    InvalidChars,
    Valid,
}

impl UsernameValidation {
    fn message(&self) -> &'static str {
        match self {
            Self::Empty => "Please enter a username",
            Self::TooShort => "Username must be at least 3 characters",
            Self::TooLong => "Username must be 20 characters or less",
            Self::InvalidChars => "Username can only contain letters, numbers, and underscores",
            Self::Valid => "Username looks good",
        }
    }

    fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    fn css_class(&self) -> &'static str {
        match self {
            Self::Valid => "validation-hint valid",
            Self::Empty => "validation-hint",
            _ => "validation-hint invalid",
        }
    }
}

fn validate_username(username: &str) -> UsernameValidation {
    if username.is_empty() {
        return UsernameValidation::Empty;
    }
    if username.len() < 3 {
        return UsernameValidation::TooShort;
    }
    if username.len() > 20 {
        return UsernameValidation::TooLong;
    }
    // Check if username only contains alphanumeric and underscores
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return UsernameValidation::InvalidChars;
    }
    UsernameValidation::Valid
}

#[derive(Properties, PartialEq)]
pub struct LoginPageProps {
    pub state: yew::UseReducerHandle<AppState>,
}

#[function_component(LoginPage)]
pub fn login_page(props: &LoginPageProps) -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(String::new);
    let loading = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let current_step = use_state(|| LoginStep::EnterUsername);

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_continue = {
        let username = username.clone();
        let current_step = current_step.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();
        let state = props.state.clone();

        Callback::from(move |_| {
            // Validate username before proceeding
            let validation = validate_username(&username);
            if !validation.is_valid() {
                return;
            }

            let username = username.clone();
            let current_step = current_step.clone();
            let loading = loading.clone();
            let error_message = error_message.clone();
            let navigator = navigator.clone();
            let state = state.clone();

            loading.set(true);
            error_message.set(None);
            current_step.set(LoginStep::ConnectingWallet);

            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&"Connecting to Freighter...".into());

                // Connect to Freighter
                match connect_wallet().await {
                    Ok(wallet_address) => {
                        web_sys::console::log_1(&format!("Wallet connected: {}", wallet_address).into());

                        let username_val = (*username).clone();

                        // Save to backend
                        let api_client = ApiClient::new();
                        let guest = Guest {
                            username: username_val.clone(),
                            wallet_address: wallet_address.clone(),
                        };

                        match api_client.register_guest(guest).await {
                            Ok(response) => {
                                web_sys::console::log_1(&format!("User saved to backend: {} (ID: {})", response.message, response.user.id).into());

                                // Update main AppState with connected wallet
                                let connected_wallet = ConnectedWallet {
                                    wallet_type: WalletType::Freighter,
                                    address: wallet_address.clone(),
                                    status: WalletStatus::Connected("Freighter".to_string()),
                                };
                                state.dispatch(AppMessage::WalletConnected(connected_wallet));

                                // Show success briefly
                                current_step.set(LoginStep::Success);

                                // Navigate back to home after delay
                                gloo::timers::callback::Timeout::new(2000, move || {
                                    navigator.push(&Route::Home);
                                }).forget();
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Backend registration failed: {}", e).into());
                                error_message.set(Some(format!("Registration failed: {}", e)));
                                loading.set(false);
                                current_step.set(LoginStep::EnterUsername);
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Wallet connection failed: {}", e).into());
                        error_message.set(Some(format!("Failed to connect: {}", e)));
                        loading.set(false);
                        current_step.set(LoginStep::EnterUsername);
                    }
                }
            });
        })
    };

    // Calculate validation outside html! macro
    let validation = validate_username(&username);

    html! {
        <div class="login-page">
            <div class="login-container">
                <div class="login-header">
                    <h1>{"Welcome to Stellar dApp"}</h1>
                    <p class="login-subtitle">{"Connect your wallet to get started"}</p>
                </div>

                {
                    if let Some(error) = (*error_message).clone() {
                        html! {
                            <div class="login-error">
                                <span>{"Warning: "}</span>
                                <span>{error}</span>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="login-content">
                    {
                        match *current_step {
                            LoginStep::EnterUsername => html! {
                                <div class="login-step">
                                    <h2>{"Step 1: Choose Your Username"}</h2>
                                    <p class="step-description">{"This will be your display name on the platform"}</p>

                                    <div class="form-group">
                                        <label for="username">{"Username"}</label>
                                        <input
                                            id="username"
                                            type="text"
                                            class="form-input"
                                            placeholder="Enter your username..."
                                            value={(*username).clone()}
                                            oninput={on_username_input}
                                            maxlength="20"
                                            autofocus=true
                                        />
                                        {
                                            if !username.is_empty() {
                                                html! {
                                                    <div class={validation.css_class()}>
                                                        {validation.message()}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>

                                    {
                                        if validation.is_valid() {
                                            html! {
                                                <button
                                                    class="btn btn-primary btn-large btn-fade-in"
                                                    onclick={on_continue}
                                                    disabled={*loading}
                                                >
                                                    {"Continue to Wallet Connection"}
                                                </button>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }

                                    <div class="login-footer">
                                        <p>{"Step 2: Connect your Freighter wallet"}</p>
                                        <p>{"Step 3: Start using the dApp"}</p>
                                    </div>
                                </div>
                            },
                            LoginStep::ConnectingWallet => html! {
                                <div class="login-step">
                                    <div class="loading-animation">
                                        <div class="spinner-large"></div>
                                    </div>

                                    <h2>{"Connecting to Freighter..."}</h2>
                                    <p class="step-description">{"Please approve the connection in your Freighter wallet extension"}</p>

                                    <div class="connecting-steps">
                                        <div class="step-item active">
                                            <span class="step-icon">{"[✓]"}</span>
                                            <span>{"Username set: "}{(*username).clone()}</span>
                                        </div>
                                        <div class="step-item active">
                                            <span class="step-icon">{"[...]"}</span>
                                            <span>{"Waiting for wallet approval..."}</span>
                                        </div>
                                        <div class="step-item">
                                            <span class="step-icon">{"[ ]"}</span>
                                            <span>{"Saving to backend..."}</span>
                                        </div>
                                    </div>
                                </div>
                            },
                            LoginStep::Success => html! {
                                <div class="login-step">
                                    <div class="success-animation">
                                        <div class="success-checkmark">{"SUCCESS"}</div>
                                    </div>

                                    <h2>{"Welcome, "}{(*username).clone()}{"!"}</h2>
                                    <p class="step-description">{"Your wallet is connected. Redirecting to home..."}</p>

                                    <div class="success-message">
                                        <p>{"Successfully logged in!"}</p>
                                        <p>{"You can now interact with Stellar contracts"}</p>
                                    </div>
                                </div>
                            },
                        }
                    }
                </div>
            </div>
        </div>
    }
}