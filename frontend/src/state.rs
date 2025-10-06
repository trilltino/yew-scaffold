use std::rc::Rc;
use yew::Reducible;
use crate::wallet::ConnectedWallet;
use crate::types::ContractFunction;

// Include tests module
#[cfg(test)]
#[path = "state_test.rs"]
mod state_test;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub connected_wallet: Option<ConnectedWallet>,
    pub is_connecting: bool,
    pub result_message: String,
    pub is_processing: bool,
    pub selected_function: Option<ContractFunction>,
    pub dark_mode: bool,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.connected_wallet == other.connected_wallet
            && self.is_connecting == other.is_connecting
            && self.result_message == other.result_message
            && self.is_processing == other.is_processing
            && self.selected_function == other.selected_function
            && self.dark_mode == other.dark_mode
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
            dark_mode: false,
        }
    }
}

/// Application messages
#[derive(Debug, Clone)]
pub enum AppMessage {
    WalletConnected(ConnectedWallet),
    WalletConnectionFailed(String),
    SelectFunction(ContractFunction),
    SignTransaction,
    TransactionResult(String),
    ToggleDarkMode,
}

impl Reducible for AppState {
    type Action = AppMessage;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppMessage::WalletConnected(connected_wallet) => Self {
                connected_wallet: Some(connected_wallet.clone()),
                is_connecting: false,
                result_message: format!("Connected to {}...{}",
                                      &connected_wallet.address[..6],
                                      &connected_wallet.address[connected_wallet.address.len()-6..]),
                ..(*self).clone()
            }.into(),

            AppMessage::WalletConnectionFailed(error) => Self {
                is_connecting: false,
                result_message: format!("Connection failed: {}", error),
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

            AppMessage::ToggleDarkMode => Self {
                dark_mode: !self.dark_mode,
                ..(*self).clone()
            }.into(),
        }
    }
}