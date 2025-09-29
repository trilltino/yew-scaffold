use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::wallet::{WalletType, ConnectedWallet};

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub connected_wallet: Option<ConnectedWallet>,
    pub is_connecting: bool,
    pub on_connect_wallet: Callback<WalletType>,
    pub on_disconnect_wallet: Callback<()>,
}

#[function_component(Navigation)]
pub fn navigation(props: &NavigationProps) -> Html {
    let location = use_location().unwrap();
    let current_route = &location.path();

    html! {
        <nav class="nav">
            <div class="nav-content">
                <ul class="nav-links">
                    <li>
                        <Link<Route>
                            to={Route::About}
                            classes={if *current_route == "/about" { "nav-link active" } else { "nav-link" }}
                        >
                            {"About"}
                        </Link<Route>>
                    </li>
                    <li class="nav-wallet-item">
                        <FreighterButton
                            connected_wallet={props.connected_wallet.clone()}
                            is_connecting={props.is_connecting}
                            on_connect_wallet={props.on_connect_wallet.clone()}
                            on_disconnect_wallet={props.on_disconnect_wallet.clone()}
                        />
                    </li>
                </ul>
            </div>
        </nav>
    }
}

/// Simple Freighter wallet button
#[derive(Properties, PartialEq)]
pub struct FreighterButtonProps {
    pub connected_wallet: Option<ConnectedWallet>,
    pub is_connecting: bool,
    pub on_connect_wallet: Callback<WalletType>,
    pub on_disconnect_wallet: Callback<()>,
}

#[function_component(FreighterButton)]
pub fn freighter_button(props: &FreighterButtonProps) -> Html {
    let (image_class, button_disabled) = match (&props.connected_wallet, props.is_connecting) {
        (Some(_wallet), false) => ("freighter-btn freighter-btn-connected", false),
        (None, true) => ("freighter-btn freighter-btn-connecting", true),
        (None, false) => ("freighter-btn", false),
        (Some(_), true) => ("freighter-btn freighter-btn-connecting", true),
    };

    let on_click = {
        let connected_wallet = props.connected_wallet.clone();
        let on_disconnect = props.on_disconnect_wallet.clone();
        let on_connect_wallet = props.on_connect_wallet.clone();

        Callback::from(move |_| {
            if !button_disabled {
                if connected_wallet.is_some() {
                    on_disconnect.emit(());
                } else {
                    on_connect_wallet.emit(WalletType::Freighter);
                }
            }
        })
    };

    let title = if props.connected_wallet.is_some() {
        "Click to disconnect from Freighter wallet"
    } else {
        "Click to connect Freighter wallet"
    };

    html! {
        <img
            class={image_class}
            src="/freighter.webp"
            alt="Freighter Wallet"
            width="32"
            height="32"
            onclick={on_click}
            title={title}
            style={if button_disabled { "pointer-events: none; opacity: 0.5;" } else { "cursor: pointer;" }}
        />
    }
}

