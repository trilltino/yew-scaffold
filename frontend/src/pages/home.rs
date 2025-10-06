use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::components::{ContractSection, SorobanTestSection, SorobanMetricsLive, ReflectorOracleSection};
use crate::wallet::{WalletType, ConnectedWallet, WalletStatus, connect_wallet};
use crate::types::ContractFunction;
use crate::state::{AppState, AppMessage};
use crate::services::sign_hello_transaction;

/// Home page component with wallet and contract functionality
#[derive(Properties, PartialEq)]
pub struct HomePageProps {
    pub state: yew::UseReducerHandle<AppState>,
}

#[function_component(HomePage)]
pub fn home_page(props: &HomePageProps) -> Html {
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

            <SorobanMetricsLive />

            <ReflectorOracleSection />

            <SorobanTestSection />
        </main>
    }
}