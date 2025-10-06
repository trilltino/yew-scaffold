use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde_json;

use shared::dto::soroban::*;

const BACKEND_URL: &str = "http://127.0.0.1:3001";
const CONTRACT_ID: &str = "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF";

#[function_component(SorobanTestSection)]
pub fn soroban_test_section() -> Html {
    let events_result = use_state(|| String::from("Click 'Query Events' to test"));
    let simulation_result = use_state(|| String::from("Click 'Simulate Transaction' to test"));
    let state_result = use_state(|| String::from("Click 'Get Contract State' to test"));

    let is_querying_events = use_state(|| false);
    let is_simulating = use_state(|| false);
    let is_querying_state = use_state(|| false);

    // Query Events Handler
    let on_query_events = {
        let events_result = events_result.clone();
        let is_querying_events = is_querying_events.clone();

        Callback::from(move |_| {
            let events_result = events_result.clone();
            let is_querying_events = is_querying_events.clone();

            is_querying_events.set(true);
            events_result.set("🔄 Querying events...".to_string());

            spawn_local(async move {
                let request = QueryEventsRequest {
                    contract_id: CONTRACT_ID.to_string(),
                    pagination: EventPagination::From { ledger: 1 },
                    filters: vec![EventFilterDto {
                        event_type: EventType::Contract,
                        contract_ids: vec![CONTRACT_ID.to_string()],
                        topics: vec![],
                    }],
                    limit: Some(10),
                };

                match Request::post(&format!("{}/api/soroban/events", BACKEND_URL))
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<QueryEventsResponse>().await {
                            Ok(data) => {
                                let pretty = serde_json::to_string_pretty(&data)
                                    .unwrap_or_else(|_| "Error formatting response".to_string());
                                events_result.set(format!("✅ Success:\n\n{}", pretty));
                            }
                            Err(e) => {
                                events_result.set(format!("❌ Parse Error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        events_result.set(format!("❌ Request Error: {}", e));
                    }
                }
                is_querying_events.set(false);
            });
        })
    };

    // Simulate Transaction Handler
    let on_simulate_transaction = {
        let simulation_result = simulation_result.clone();
        let is_simulating = is_simulating.clone();

        Callback::from(move |_| {
            let simulation_result = simulation_result.clone();
            let is_simulating = is_simulating.clone();

            is_simulating.set(true);
            simulation_result.set("🔄 Simulating transaction...".to_string());

            spawn_local(async move {
                // Sample XDR for simulation (this is a valid hello_yew transaction)
                let sample_xdr = "AAAAAgAAAACiEzg0JkWS9MhlpE+yk8a0w2KMLnH25bV9CDD5kkoeBAAQCR8AB96tAAAAAQAAAAAAAAAAAAAAAQAAAAAAAAAYAAAAAAAAAAGKXpAaEqPqzPSZFN+toNeI+Ml59moclRqcWzmPotcC5QAAAAloZWxsb195ZXcAAAAAAAABAAAADgAAAANZZXcAAAAAAAAAAAEAAAAAAAAAAgAAAAYAAAABil6QGhKj6sz0mRTfraDXiPjJefZqHJUanFs5j6LXAuUAAAAUAAAAAQAAAAfwMzbiOi0F4TdwiXKmAyuJPm3COAWDdS4NjHuXsx7M6wAAAAAABb7NAAAAAAAAAAAAAAAAAADG3wAAAAA=";

                let request = SimulateTransactionRequest {
                    contract_id: CONTRACT_ID.to_string(),
                    transaction_xdr: sample_xdr.to_string(),
                    options: Some(SimulationOptionsDto {
                        cpu_instructions: 100000,
                        auth_mode: Some(AuthModeDto::Record),
                    }),
                };

                match Request::post(&format!("{}/api/soroban/simulate", BACKEND_URL))
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<SimulateTransactionResponseDto>().await {
                            Ok(data) => {
                                let pretty = serde_json::to_string_pretty(&data)
                                    .unwrap_or_else(|_| "Error formatting response".to_string());
                                simulation_result.set(format!("✅ Success:\n\n{}", pretty));
                            }
                            Err(e) => {
                                simulation_result.set(format!("❌ Parse Error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        simulation_result.set(format!("❌ Request Error: {}", e));
                    }
                }
                is_simulating.set(false);
            });
        })
    };

    // Get Contract State Handler
    let on_get_contract_state = {
        let state_result = state_result.clone();
        let is_querying_state = is_querying_state.clone();

        Callback::from(move |_| {
            let state_result = state_result.clone();
            let is_querying_state = is_querying_state.clone();

            is_querying_state.set(true);
            state_result.set("🔄 Querying contract state...".to_string());

            spawn_local(async move {
                // Sample base64 XDR key for "COUNTER" or similar
                let sample_key = "AAAADwAAAAdDT1VOVEVSAA=="; // "COUNTER" in base64 XDR

                let request = GetContractDataRequest {
                    contract_id: CONTRACT_ID.to_string(),
                    key: sample_key.to_string(),
                    durability: DurabilityDto::Persistent,
                };

                match Request::post(&format!("{}/api/soroban/contract-data", BACKEND_URL))
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<GetContractDataResponse>().await {
                            Ok(data) => {
                                let pretty = serde_json::to_string_pretty(&data)
                                    .unwrap_or_else(|_| "Error formatting response".to_string());
                                state_result.set(format!("✅ Success:\n\n{}", pretty));
                            }
                            Err(e) => {
                                state_result.set(format!("❌ Parse Error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        state_result.set(format!("❌ Request Error: {}", e));
                    }
                }
                is_querying_state.set(false);
            });
        })
    };

    html! {
        <div class="soroban-test-section">
            <h2>{"🔬 Soroban Advanced Features Testing"}</h2>
            <p class="test-description">
                {"Test the advanced Soroban features: Event Querying, Transaction Simulation, and Contract State Queries"}
            </p>

            <div class="test-grid">
                // Events Query Test
                <div class="test-card">
                    <h3>{"📊 Event Querying"}</h3>
                    <p class="test-info">
                        {"Query contract events from the Stellar network. This retrieves historical events emitted by the contract."}
                    </p>
                    <button
                        class="btn btn-test"
                        onclick={on_query_events}
                        disabled={*is_querying_events}
                    >
                        {if *is_querying_events { "Querying..." } else { "Query Events" }}
                    </button>
                    <textarea
                        class="result-textarea"
                        readonly=true
                        value={(*events_result).clone()}
                        rows="12"
                    />
                </div>

                // Transaction Simulation Test
                <div class="test-card">
                    <h3>{"🎭 Transaction Simulation"}</h3>
                    <p class="test-info">
                        {"Simulate a transaction without submitting it to the network. Shows resource usage, return values, and potential errors."}
                    </p>
                    <button
                        class="btn btn-test"
                        onclick={on_simulate_transaction}
                        disabled={*is_simulating}
                    >
                        {if *is_simulating { "Simulating..." } else { "Simulate Transaction" }}
                    </button>
                    <textarea
                        class="result-textarea"
                        readonly=true
                        value={(*simulation_result).clone()}
                        rows="12"
                    />
                </div>

                // Contract State Query Test
                <div class="test-card">
                    <h3>{"💾 Contract State Query"}</h3>
                    <p class="test-info">
                        {"Query contract storage data directly from the ledger. Retrieves persistent or temporary contract state."}
                    </p>
                    <button
                        class="btn btn-test"
                        onclick={on_get_contract_state}
                        disabled={*is_querying_state}
                    >
                        {if *is_querying_state { "Querying..." } else { "Get Contract State" }}
                    </button>
                    <textarea
                        class="result-textarea"
                        readonly=true
                        value={(*state_result).clone()}
                        rows="12"
                    />
                </div>
            </div>

            <div class="test-info-footer">
                <p>
                    <strong>{"Contract ID: "}</strong>
                    <code>{CONTRACT_ID}</code>
                </p>
                <p class="test-note">
                    {"💡 These tests demonstrate the production-hardened Soroban service with circuit breaker protection, connection pooling, and multi-layer caching."}
                </p>
            </div>
        </div>
    }
}
