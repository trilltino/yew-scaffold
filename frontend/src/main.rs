/// Simple Stellar Soroban dApp with Freighter wallet integration
///
/// This is a clean, focused implementation that provides:
/// - Direct Freighter wallet connection
/// - XDR generation and signing for hello_yew contract
/// - Clean, modern UI without unnecessary complexity
/// - Simple component architecture with routing
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod components;
mod pages;
mod services;
mod wallet;
mod types;
mod state;
mod router;

use components::Navigation;
use wallet::is_freighter_available;
use state::{AppState, AppMessage};
use router::{Route, switch_with_state};

/// Main application component with routing
#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(AppState::default);

    let on_toggle_dark_mode = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(AppMessage::ToggleDarkMode))
    };

    // Handle dark mode body class
    use_effect_with(state.dark_mode, |&dark_mode| {
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            body.set_class_name(if dark_mode { "dark-mode" } else { "" });
        }
    });

    // Check if Freighter is available on mount
    use_effect_with((), |_| {
        spawn_local(async { let _ = is_freighter_available().await; });
    });

    html! {
        <BrowserRouter>
            <style>
                {include_str!("styles.css")}
            </style>
            <div class="app">
                <Navigation
                    dark_mode={state.dark_mode}
                    on_toggle_dark_mode={on_toggle_dark_mode}
                    connected_wallet={state.connected_wallet.as_ref().map(|w| w.address.clone())}
                />
                <Switch<Route> render={switch_with_state(state.clone())} />
                <footer class="footer">
                    <p>{"Built with "}<strong>{"Yew"}</strong>{" & "}<strong>{"Stellar"}</strong></p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}