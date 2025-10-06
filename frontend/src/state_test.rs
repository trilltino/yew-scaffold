/// Unit tests for application state reducer
///
/// Tests the state transitions and reducer logic

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::wallet::ConnectedWallet;
    use std::rc::Rc;

    #[test]
    fn test_default_state() {
        let state = AppState::default();

        assert!(state.connected_wallet.is_none());
        assert_eq!(state.is_connecting, false);
        assert_eq!(state.is_processing, false);
        assert_eq!(state.dark_mode, false);
        assert!(state.selected_function.is_none());
        assert_eq!(state.result_message, "Ready to connect with Freighter wallet");
    }

    #[test]
    fn test_wallet_connected_message() {
        let state = Rc::new(AppState::default());

        let wallet = ConnectedWallet {
            address: "GABC123456789DEFGHIJKLMNOPQRSTUVWXYZ1234567890ABCD".to_string(),
            public_key: "ABC123".to_string(),
            wallet_type: "freighter".to_string(),
        };

        let new_state = state.reduce(AppMessage::WalletConnected(wallet.clone()));

        assert!(new_state.connected_wallet.is_some());
        assert_eq!(new_state.is_connecting, false);
        assert!(new_state.result_message.contains("GABC12"));
        assert!(new_state.result_message.contains("90ABCD"));
    }

    #[test]
    fn test_wallet_connection_failed() {
        let state = Rc::new(AppState::default());

        let new_state = state.reduce(AppMessage::WalletConnectionFailed("User rejected".to_string()));

        assert!(new_state.connected_wallet.is_none());
        assert_eq!(new_state.is_connecting, false);
        assert_eq!(new_state.result_message, "Connection failed: User rejected");
    }

    #[test]
    fn test_select_function() {
        let state = Rc::new(AppState::default());

        let function = ContractFunction::Hello { to: "World".to_string() };
        let new_state = state.reduce(AppMessage::SelectFunction(function.clone()));

        assert!(new_state.selected_function.is_some());
        assert_eq!(new_state.selected_function.as_ref().unwrap().name(), "hello");
        assert!(new_state.result_message.contains("Hello"));
    }

    #[test]
    fn test_sign_transaction() {
        let state = Rc::new(AppState::default());

        let new_state = state.reduce(AppMessage::SignTransaction);

        assert_eq!(new_state.is_processing, true);
        assert!(new_state.result_message.contains("Generating XDR"));
    }

    #[test]
    fn test_transaction_result() {
        let mut state = AppState::default();
        state.is_processing = true;
        let state = Rc::new(state);

        let new_state = state.reduce(AppMessage::TransactionResult("Transaction successful".to_string()));

        assert_eq!(new_state.is_processing, false);
        assert_eq!(new_state.result_message, "Transaction successful");
    }

    #[test]
    fn test_toggle_dark_mode() {
        let state = Rc::new(AppState::default());

        // Start with dark mode off
        assert_eq!(state.dark_mode, false);

        // Toggle on
        let new_state = state.reduce(AppMessage::ToggleDarkMode);
        assert_eq!(new_state.dark_mode, true);

        // Toggle off
        let new_state = new_state.reduce(AppMessage::ToggleDarkMode);
        assert_eq!(new_state.dark_mode, false);
    }

    #[test]
    fn test_state_equality() {
        let state1 = AppState::default();
        let state2 = AppState::default();

        assert_eq!(state1, state2);

        let mut state3 = AppState::default();
        state3.dark_mode = true;

        assert_ne!(state1, state3);
    }

    #[test]
    fn test_state_clone() {
        let state = AppState {
            connected_wallet: Some(ConnectedWallet {
                address: "GTEST123".to_string(),
                public_key: "TEST".to_string(),
                wallet_type: "freighter".to_string(),
            }),
            is_connecting: true,
            result_message: "Test message".to_string(),
            is_processing: false,
            selected_function: Some(ContractFunction::Simple),
            dark_mode: true,
        };

        let cloned = state.clone();

        assert_eq!(state, cloned);
    }

    #[test]
    fn test_state_transitions_preserve_wallet() {
        let wallet = ConnectedWallet {
            address: "GTEST123".to_string(),
            public_key: "TEST".to_string(),
            wallet_type: "freighter".to_string(),
        };

        let mut state = AppState::default();
        state.connected_wallet = Some(wallet.clone());
        let state = Rc::new(state);

        // Toggle dark mode shouldn't affect wallet
        let new_state = state.reduce(AppMessage::ToggleDarkMode);
        assert!(new_state.connected_wallet.is_some());
        assert_eq!(new_state.connected_wallet.as_ref().unwrap().address, wallet.address);

        // Select function shouldn't affect wallet
        let new_state = new_state.reduce(AppMessage::SelectFunction(ContractFunction::Simple));
        assert!(new_state.connected_wallet.is_some());
    }

    #[test]
    fn test_multiple_state_transitions() {
        let state = Rc::new(AppState::default());

        // Connect wallet
        let wallet = ConnectedWallet {
            address: "GTEST123".to_string(),
            public_key: "TEST".to_string(),
            wallet_type: "freighter".to_string(),
        };
        let state = state.reduce(AppMessage::WalletConnected(wallet));

        // Select function
        let state = state.reduce(AppMessage::SelectFunction(ContractFunction::HelloYew {
            to: "Yew".to_string(),
        }));

        // Start transaction
        let state = state.reduce(AppMessage::SignTransaction);
        assert_eq!(state.is_processing, true);

        // Complete transaction
        let state = state.reduce(AppMessage::TransactionResult("Success".to_string()));
        assert_eq!(state.is_processing, false);
        assert_eq!(state.result_message, "Success");

        // Wallet and function should still be set
        assert!(state.connected_wallet.is_some());
        assert!(state.selected_function.is_some());
    }
}
