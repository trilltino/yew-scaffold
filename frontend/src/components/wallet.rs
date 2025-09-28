use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WalletProps {
    pub wallet_address: Option<String>,
    pub is_connecting: bool,
    pub on_connect_wallet: Callback<()>,
}

#[function_component(WalletSection)]
pub fn wallet_section(props: &WalletProps) -> Html {
    let (connect_button_text, connect_button_disabled) = match (&props.wallet_address, props.is_connecting) {
        (Some(address), false) => (format!("Connected: {}...{}", &address[..6], &address[address.len()-6..]), true),
        (None, true) => ("Connecting...".to_string(), true),
        (None, false) => ("Connect Freighter".to_string(), false),
        (Some(_), true) => ("Connecting...".to_string(), true),
    };

    let on_connect = {
        let callback = props.on_connect_wallet.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <div class="wallet-section">
            <h2>{"Wallet Connection"}</h2>
            <button
                class="btn btn-primary"
                onclick={on_connect}
                disabled={connect_button_disabled}
            >
                {connect_button_text}
            </button>
        </div>
    }
}