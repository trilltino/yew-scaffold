use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde_json;

use shared::dto::soroban::*;

const BACKEND_URL: &str = "http://127.0.0.1:3001";
const BLEND_POOL_ID: &str = "CDDG7DLOWSHRYQ2HWGZEZ4UTR7LPTKFFHN3QUCSZEXOWOPARMONX6T65";
const BLEND_BACKSTOP_ID: &str = "CBHWKF4RHIKOKSURAKXSJRIIA7RJAMJH4VHRVPYGUF4AJ5L544LYZ35X";

#[function_component(BlendProtocol)]
pub fn blend_protocol() -> Html {
    let pool_result = use_state(|| String::from("Click a button to query Blend protocol data"));
    let is_querying = use_state(|| false);

    // Query pool config
    let query_pool_config = {
        let pool_result = pool_result.clone();
        let is_querying = is_querying.clone();

        Callback::from(move |_| {
            let pool_result = pool_result.clone();
            let is_querying = is_querying.clone();

            is_querying.set(true);
            pool_result.set("🔄 Querying pool configuration...".to_string());

            spawn_local(async move {
                let request = CallContractFunctionRequest {
                    contract_id: BLEND_POOL_ID.to_string(),
                    function_name: "get_config".to_string(),
                    parameters: vec![],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);

                match Request::post(&url).json(&request).unwrap().send().await {
                    Ok(response) => {
                        if let Ok(response_text) = response.text().await {
                            if let Ok(data) = serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                                if data.success {
                                    let pretty = serde_json::to_string_pretty(&data.result)
                                        .unwrap_or_else(|_| "Error formatting result".to_string());
                                    pool_result.set(format!("✅ Pool Config:\n\n{}", pretty));
                                } else {
                                    pool_result.set(format!("❌ Error: {}", data.error.unwrap_or_default()));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        pool_result.set(format!("❌ Request failed: {}", e));
                    }
                }
                is_querying.set(false);
            });
        })
    };

    // Query pool reserves
    let query_pool_reserves = {
        let pool_result = pool_result.clone();
        let is_querying = is_querying.clone();

        Callback::from(move |_| {
            let pool_result = pool_result.clone();
            let is_querying = is_querying.clone();

            is_querying.set(true);
            pool_result.set("🔄 Querying pool reserves...".to_string());

            spawn_local(async move {
                let request = CallContractFunctionRequest {
                    contract_id: BLEND_POOL_ID.to_string(),
                    function_name: "get_reserve_list".to_string(),
                    parameters: vec![],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);

                match Request::post(&url).json(&request).unwrap().send().await {
                    Ok(response) => {
                        if let Ok(response_text) = response.text().await {
                            if let Ok(data) = serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                                if data.success {
                                    let pretty = serde_json::to_string_pretty(&data.result)
                                        .unwrap_or_else(|_| "Error formatting result".to_string());
                                    pool_result.set(format!("✅ Pool Reserves:\n\n{}", pretty));
                                } else {
                                    pool_result.set(format!("❌ Error: {}", data.error.unwrap_or_default()));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        pool_result.set(format!("❌ Request failed: {}", e));
                    }
                }
                is_querying.set(false);
            });
        })
    };

    // Query pool admin
    let query_pool_admin = {
        let pool_result = pool_result.clone();
        let is_querying = is_querying.clone();

        Callback::from(move |_| {
            let pool_result = pool_result.clone();
            let is_querying = is_querying.clone();

            is_querying.set(true);
            pool_result.set("🔄 Querying pool admin...".to_string());

            spawn_local(async move {
                let request = CallContractFunctionRequest {
                    contract_id: BLEND_POOL_ID.to_string(),
                    function_name: "get_admin".to_string(),
                    parameters: vec![],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);

                match Request::post(&url).json(&request).unwrap().send().await {
                    Ok(response) => {
                        if let Ok(response_text) = response.text().await {
                            if let Ok(data) = serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                                if data.success {
                                    let pretty = serde_json::to_string_pretty(&data.result)
                                        .unwrap_or_else(|_| "Error formatting result".to_string());
                                    pool_result.set(format!("✅ Pool Admin:\n\n{}", pretty));
                                } else {
                                    pool_result.set(format!("❌ Error: {}", data.error.unwrap_or_default()));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        pool_result.set(format!("❌ Request failed: {}", e));
                    }
                }
                is_querying.set(false);
            });
        })
    };

    // Query reward zone
    let query_reward_zone = {
        let pool_result = pool_result.clone();
        let is_querying = is_querying.clone();

        Callback::from(move |_| {
            let pool_result = pool_result.clone();
            let is_querying = is_querying.clone();

            is_querying.set(true);
            pool_result.set("🔄 Querying reward zone...".to_string());

            spawn_local(async move {
                let request = CallContractFunctionRequest {
                    contract_id: BLEND_BACKSTOP_ID.to_string(),
                    function_name: "reward_zone".to_string(),
                    parameters: vec![],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);

                match Request::post(&url).json(&request).unwrap().send().await {
                    Ok(response) => {
                        if let Ok(response_text) = response.text().await {
                            if let Ok(data) = serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                                if data.success {
                                    let pretty = serde_json::to_string_pretty(&data.result)
                                        .unwrap_or_else(|_| "Error formatting result".to_string());
                                    pool_result.set(format!("✅ Reward Zone:\n\n{}", pretty));
                                } else {
                                    pool_result.set(format!("❌ Error: {}", data.error.unwrap_or_default()));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        pool_result.set(format!("❌ Request failed: {}", e));
                    }
                }
                is_querying.set(false);
            });
        })
    };

    // Query backstop token
    let query_backstop = {
        let pool_result = pool_result.clone();
        let is_querying = is_querying.clone();

        Callback::from(move |_| {
            let pool_result = pool_result.clone();
            let is_querying = is_querying.clone();

            is_querying.set(true);
            pool_result.set("🔄 Querying backstop data...".to_string());

            spawn_local(async move {
                let request = CallContractFunctionRequest {
                    contract_id: BLEND_BACKSTOP_ID.to_string(),
                    function_name: "backstop_token".to_string(),
                    parameters: vec![],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);

                match Request::post(&url).json(&request).unwrap().send().await {
                    Ok(response) => {
                        if let Ok(response_text) = response.text().await {
                            if let Ok(data) = serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                                if data.success {
                                    let pretty = serde_json::to_string_pretty(&data.result)
                                        .unwrap_or_else(|_| "Error formatting result".to_string());
                                    pool_result.set(format!("✅ Backstop Token:\n\n{}", pretty));
                                } else {
                                    pool_result.set(format!("❌ Error: {}", data.error.unwrap_or_default()));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        pool_result.set(format!("❌ Request failed: {}", e));
                    }
                }
                is_querying.set(false);
            });
        })
    };

    html! {
        <div class="blend-protocol">
            <h2>{"💧 Blend Protocol"}</h2>
            <p class="protocol-description">
                {"Universal liquidity protocol primitive for lending and borrowing on Stellar. "}
                {"Isolated lending pools with dynamic interest rates and market-driven backstop system."}
            </p>

            <div class="protocol-info">
                <p><strong>{"Test Pool:"}</strong> {" CDDG7D...6T65"}</p>
                <p><strong>{"Backstop:"}</strong> {" CBHWKF...YZ35X"}</p>
                <p><strong>{"Network:"}</strong> {" Testnet"}</p>
            </div>

            <div class="protocol-actions">
                <h3>{"📊 Query Pool Data"}</h3>
                <div class="button-group">
                    <button
                        class="btn btn-primary"
                        onclick={query_pool_config}
                        disabled={*is_querying}
                    >
                        {"Get Pool Config"}
                    </button>
                    <button
                        class="btn btn-primary"
                        onclick={query_pool_reserves}
                        disabled={*is_querying}
                    >
                        {"Get Reserve List"}
                    </button>
                    <button
                        class="btn btn-primary"
                        onclick={query_pool_admin}
                        disabled={*is_querying}
                    >
                        {"Get Pool Admin"}
                    </button>
                </div>

                <h3>{"🔐 Query Backstop Data"}</h3>
                <div class="button-group">
                    <button
                        class="btn btn-primary"
                        onclick={query_backstop}
                        disabled={*is_querying}
                    >
                        {"Get Backstop Token"}
                    </button>
                    <button
                        class="btn btn-primary"
                        onclick={query_reward_zone}
                        disabled={*is_querying}
                    >
                        {"Get Reward Zone"}
                    </button>
                </div>
            </div>

            <div class="result-display">
                <pre>{&*pool_result}</pre>
            </div>
        </div>
    }
}
