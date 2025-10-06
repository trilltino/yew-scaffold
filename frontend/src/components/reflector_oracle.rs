use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde_json;

use shared::dto::soroban::*;
use crate::components::LivePriceFeed;

/// Format oracle price with 14 decimals to human-readable USD value
fn format_oracle_price(price_str: &str, _asset: &str) -> String {
    if let Ok(price) = price_str.parse::<f64>() {
        let actual_price = price / 100_000_000_000_000.0; // Divide by 10^14

        // Use appropriate precision based on price magnitude
        if actual_price >= 1000.0 {
            // Add thousands separator manually for large values
            let formatted = format!("{:.2}", actual_price);
            add_thousands_separator(&formatted)
        } else if actual_price >= 1.0 {
            format!("${:.4}", actual_price) // Medium values: $227.2790
        } else if actual_price >= 0.01 {
            format!("${:.6}", actual_price) // Small values: $0.393352
        } else {
            format!("${:.8}", actual_price) // Very small values
        }
    } else {
        format!("Invalid price: {}", price_str)
    }
}

/// Add thousands separator to a formatted number string
fn add_thousands_separator(num_str: &str) -> String {
    let parts: Vec<&str> = num_str.split('.').collect();
    let whole = parts[0];
    let decimal = if parts.len() > 1 { parts[1] } else { "00" };

    let mut result = String::new();
    let chars: Vec<char> = whole.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }

    format!("${}.{}", result, decimal)
}

const BACKEND_URL: &str = "http://127.0.0.1:3001";
// Note: On testnet, the FX oracle handles both crypto and fiat prices
// The "crypto" oracle (CAVLP...) only supports Stellar DEX assets (contract addresses)
const REFLECTOR_ORACLE_ID: &str = "CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63"; // Using FX oracle for both
const REFLECTOR_FX_ID: &str = "CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63";

#[function_component(ReflectorOracleSection)]
pub fn reflector_oracle_section() -> Html {
    let price_result = use_state(|| String::from("Select an asset to query its price"));
    let fx_result = use_state(|| String::from("Select a currency pair to query FX rate"));
    let info_result = use_state(|| String::from("Click 'Get Oracle Info' to view contract details"));
    let events_result = use_state(|| String::from("Click 'Query Events' to view oracle contract events"));

    let is_querying_price = use_state(|| false);
    let is_querying_fx = use_state(|| false);
    let is_querying_info = use_state(|| false);
    let is_querying_events = use_state(|| false);

    // Generic price query handler using the new call-function endpoint
    let create_price_query = |asset: &'static str| {
        let price_result = price_result.clone();
        let is_querying_price = is_querying_price.clone();

        Callback::from(move |_| {
            let price_result = price_result.clone();
            let is_querying_price = is_querying_price.clone();
            let asset_name = asset;

            is_querying_price.set(true);
            price_result.set(format!("🔄 Querying {} price from Reflector Oracle...", asset_name));

            web_sys::console::log_1(&format!("🔮 [REFLECTOR CRYPTO] Starting {} price query", asset_name).into());
            web_sys::console::log_1(&format!("📍 [REFLECTOR CRYPTO] Oracle Contract: {}", REFLECTOR_ORACLE_ID).into());

            spawn_local(async move {
                // Call the lastprice() function with the asset symbol
                web_sys::console::log_1(&format!("🔑 [REFLECTOR CRYPTO] Calling lastprice({:?})", asset_name).into());

                let request = CallContractFunctionRequest {
                    contract_id: REFLECTOR_ORACLE_ID.to_string(),
                    function_name: "lastprice".to_string(),
                    parameters: vec![
                        FunctionParameter::Enum(
                            "Other".to_string(),
                            Some(Box::new(FunctionParameter::Symbol(asset_name.to_string())))
                        )
                    ],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);
                web_sys::console::log_1(&format!("📤 [REFLECTOR CRYPTO] POST {}", url).into());
                web_sys::console::log_1(&format!("📦 [REFLECTOR CRYPTO] Calling lastprice({}) on {}", asset_name, REFLECTOR_ORACLE_ID).into());

                match Request::post(&url)
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        let status = response.status();
                        web_sys::console::log_1(&format!("📥 [REFLECTOR CRYPTO] Response status: {}", status).into());

                        let response_text = response.text().await.unwrap_or_else(|_| "Failed to read response".to_string());
                        web_sys::console::log_1(&format!("📄 [REFLECTOR CRYPTO] Response body ({} bytes): {}", response_text.len(), response_text).into());

                        match serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                            Ok(data) => {
                                web_sys::console::log_1(&"✅ [REFLECTOR CRYPTO] Successfully parsed response".into());
                                if data.success {
                                    // Parse the price from the result
                                    if let Some(result_obj) = data.result.as_ref().and_then(|r| r.as_object()) {
                                        if let (Some(price), Some(timestamp)) = (
                                            result_obj.get("price").and_then(|p| p.as_str()),
                                            result_obj.get("timestamp").and_then(|t| t.as_u64())
                                        ) {
                                            let formatted_price = format_oracle_price(price, asset_name);
                                            let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                                                .unwrap_or_else(|| timestamp.to_string());

                                            price_result.set(format!(
                                                "✅ {} Price: {}\n\nRaw Value: {}\nTimestamp: {}\n\n💡 Prices use 14 decimals",
                                                asset_name, formatted_price, price, datetime
                                            ));
                                        } else {
                                            let pretty = serde_json::to_string_pretty(&data.result)
                                                .unwrap_or_else(|_| "Error formatting result".to_string());
                                            price_result.set(format!("✅ {} Response:\n\n{}", asset_name, pretty));
                                        }
                                    } else {
                                        price_result.set(format!("❌ No price data available for {}", asset_name));
                                    }
                                } else {
                                    let error = data.error.unwrap_or_else(|| "Unknown error".to_string());
                                    web_sys::console::error_1(&format!("❌ [REFLECTOR CRYPTO] Function call failed: {}", error).into());
                                    price_result.set(format!("❌ Error: {}", error));
                                }
                            }
                            Err(e) => {
                                web_sys::console::error_1(&format!("❌ [REFLECTOR CRYPTO] Parse Error: {}", e).into());
                                web_sys::console::log_1(&format!("Raw response was: {}", response_text).into());
                                price_result.set(format!("❌ Parse Error: {}\n\nRaw Response:\n{}", e, response_text));
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("❌ [REFLECTOR CRYPTO] Request Error: {}", e).into());
                        price_result.set(format!("❌ Request Error: {}", e));
                    }
                }
                web_sys::console::log_1(&format!("🏁 [REFLECTOR CRYPTO] {} query completed", asset_name).into());
                is_querying_price.set(false);
            });
        })
    };

    // FX Rate query handler using the new call-function endpoint
    let create_fx_query = |pair: &'static str| {
        let fx_result = fx_result.clone();
        let is_querying_fx = is_querying_fx.clone();

        Callback::from(move |_| {
            let fx_result = fx_result.clone();
            let is_querying_fx = is_querying_fx.clone();
            let pair_name = pair;

            is_querying_fx.set(true);
            fx_result.set(format!("🔄 Querying {} rate from Reflector FX Oracle...", pair_name));

            web_sys::console::log_1(&format!("💱 [REFLECTOR FX] Starting {} rate query", pair_name).into());
            web_sys::console::log_1(&format!("📍 [REFLECTOR FX] Oracle Contract: {}", REFLECTOR_FX_ID).into());

            spawn_local(async move {
                // Call the lastprice() function with the asset symbol
                web_sys::console::log_1(&format!("🔑 [REFLECTOR FX] Calling lastprice({:?})", pair_name).into());

                let request = CallContractFunctionRequest {
                    contract_id: REFLECTOR_FX_ID.to_string(),
                    function_name: "lastprice".to_string(),
                    parameters: vec![
                        FunctionParameter::Enum(
                            "Other".to_string(),
                            Some(Box::new(FunctionParameter::Symbol(pair_name.to_string())))
                        )
                    ],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);
                web_sys::console::log_1(&format!("📤 [REFLECTOR FX] POST {}", url).into());
                web_sys::console::log_1(&format!("📦 [REFLECTOR FX] Calling lastprice({}) on {}", pair_name, REFLECTOR_FX_ID).into());

                match Request::post(&url)
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        let status = response.status();
                        web_sys::console::log_1(&format!("📥 [REFLECTOR FX] Response status: {}", status).into());

                        let response_text = response.text().await.unwrap_or_else(|_| "Failed to read response".to_string());
                        web_sys::console::log_1(&format!("📄 [REFLECTOR FX] Response body ({} bytes): {}", response_text.len(), response_text).into());

                        match serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                            Ok(data) => {
                                web_sys::console::log_1(&"✅ [REFLECTOR FX] Successfully parsed response".into());
                                if data.success {
                                    // Parse the price from the result
                                    if let Some(result_obj) = data.result.as_ref().and_then(|r| r.as_object()) {
                                        if let (Some(price), Some(timestamp)) = (
                                            result_obj.get("price").and_then(|p| p.as_str()),
                                            result_obj.get("timestamp").and_then(|t| t.as_u64())
                                        ) {
                                            let formatted_price = format_oracle_price(price, pair_name);
                                            let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                                                .unwrap_or_else(|| timestamp.to_string());

                                            fx_result.set(format!(
                                                "✅ {}/USD: {}\n\nRaw Value: {}\nTimestamp: {}\n\n💡 Prices use 14 decimals",
                                                pair_name, formatted_price, price, datetime
                                            ));
                                        } else {
                                            let pretty = serde_json::to_string_pretty(&data.result)
                                                .unwrap_or_else(|_| "Error formatting result".to_string());
                                            fx_result.set(format!("✅ {} Response:\n\n{}", pair_name, pretty));
                                        }
                                    } else {
                                        fx_result.set(format!("❌ No price data available for {}", pair_name));
                                    }
                                } else {
                                    let error = data.error.unwrap_or_else(|| "Unknown error".to_string());
                                    web_sys::console::error_1(&format!("❌ [REFLECTOR FX] Function call failed: {}", error).into());
                                    fx_result.set(format!("❌ Error: {}", error));
                                }
                            }
                            Err(e) => {
                                web_sys::console::error_1(&format!("❌ [REFLECTOR FX] Parse Error: {}", e).into());
                                web_sys::console::log_1(&format!("Raw response was: {}", response_text).into());
                                fx_result.set(format!("❌ Parse Error: {}\n\nRaw Response:\n{}", e, response_text));
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("❌ [REFLECTOR FX] Request Error: {}", e).into());
                        fx_result.set(format!("❌ Request Error: {}", e));
                    }
                }
                web_sys::console::log_1(&format!("🏁 [REFLECTOR FX] {} query completed", pair_name).into());
                is_querying_fx.set(false);
            });
        })
    };

    // Get Oracle Info Handler
    let on_get_oracle_info = {
        let info_result = info_result.clone();
        let is_querying_info = is_querying_info.clone();

        Callback::from(move |_| {
            let info_result = info_result.clone();
            let is_querying_info = is_querying_info.clone();

            is_querying_info.set(true);
            info_result.set("🔄 Fetching oracle contract information...".to_string());

            web_sys::console::log_1(&"📊 [REFLECTOR INFO] Starting contract info query".into());
            web_sys::console::log_1(&format!("📍 [REFLECTOR INFO] Oracle Contract: {}", REFLECTOR_ORACLE_ID).into());

            spawn_local(async move {
                let url = format!("{}/api/soroban/contract/{}", BACKEND_URL, REFLECTOR_ORACLE_ID);
                web_sys::console::log_1(&format!("📤 [REFLECTOR INFO] GET {}", url).into());

                match Request::get(&url)
                    .send()
                    .await
                {
                    Ok(response) => {
                        let status = response.status();
                        web_sys::console::log_1(&format!("📥 [REFLECTOR INFO] Response status: {}", status).into());

                        let response_text = response.text().await.unwrap_or_else(|_| "Failed to read response".to_string());
                        web_sys::console::log_1(&format!("📄 [REFLECTOR INFO] Response body ({} bytes): {}", response_text.len(), response_text).into());

                        match serde_json::from_str::<serde_json::Value>(&response_text) {
                            Ok(data) => {
                                web_sys::console::log_1(&"✅ [REFLECTOR INFO] Successfully parsed response".into());
                                let pretty = serde_json::to_string_pretty(&data)
                                    .unwrap_or_else(|_| "Error formatting response".to_string());
                                info_result.set(format!("✅ Oracle Contract Info:\n\n{}", pretty));
                            }
                            Err(e) => {
                                web_sys::console::error_1(&format!("❌ [REFLECTOR INFO] Parse Error: {}", e).into());
                                web_sys::console::log_1(&format!("Raw response was: {}", response_text).into());
                                info_result.set(format!("❌ Parse Error: {}\n\nRaw Response:\n{}", e, response_text));
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("❌ [REFLECTOR INFO] Request Error: {}", e).into());
                        info_result.set(format!("❌ Request Error: {}", e));
                    }
                }
                web_sys::console::log_1(&"🏁 [REFLECTOR INFO] Query completed".into());
                is_querying_info.set(false);
            });
        })
    };

    // Query Oracle Events Handler
    let on_query_events = {
        let events_result = events_result.clone();
        let is_querying_events = is_querying_events.clone();

        Callback::from(move |_| {
            let events_result = events_result.clone();
            let is_querying_events = is_querying_events.clone();

            is_querying_events.set(true);
            events_result.set("🔄 Querying oracle contract events...".to_string());

            web_sys::console::log_1(&"📡 [REFLECTOR EVENTS] Starting events query".into());
            web_sys::console::log_1(&format!("📍 [REFLECTOR EVENTS] Oracle Contract: {}", REFLECTOR_ORACLE_ID).into());

            spawn_local(async move {
                let request = QueryEventsRequest {
                    contract_id: REFLECTOR_ORACLE_ID.to_string(),
                    filters: vec![EventFilterDto {
                        event_type: EventType::All,
                        contract_ids: vec![],
                        topics: vec![],
                    }],
                    pagination: EventPagination::From { ledger: 0 },
                    limit: Some(10),
                };

                let url = format!("{}/api/soroban/events", BACKEND_URL);
                web_sys::console::log_1(&format!("📤 [REFLECTOR EVENTS] POST {}", url).into());

                match Request::post(&url)
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        let status = response.status();
                        web_sys::console::log_1(&format!("📥 [REFLECTOR EVENTS] Response status: {}", status).into());

                        let response_text = response.text().await.unwrap_or_else(|_| "Failed to read response".to_string());
                        web_sys::console::log_1(&format!("📄 [REFLECTOR EVENTS] Response body ({} bytes): {}", response_text.len(), response_text).into());

                        match serde_json::from_str::<QueryEventsResponse>(&response_text) {
                            Ok(data) => {
                                web_sys::console::log_1(&"✅ [REFLECTOR EVENTS] Successfully parsed response".into());
                                if data.success {
                                    let events_count = data.events.events.len();
                                    let mut result = format!("✅ Found {} events\n\n", events_count);

                                    for (i, event) in data.events.events.iter().enumerate() {
                                        result.push_str(&format!(
                                            "Event #{}\n",
                                            i + 1
                                        ));
                                        result.push_str(&format!(
                                            "  Type: {}\n",
                                            event.event_type
                                        ));
                                        result.push_str(&format!(
                                            "  Ledger: {}\n",
                                            event.ledger
                                        ));
                                        result.push_str(&format!(
                                            "  Time: {}\n",
                                            event.ledger_closed_at
                                        ));
                                        if let Some(tx_hash) = &event.transaction_hash {
                                            result.push_str(&format!(
                                                "  TX: {}...\n",
                                                &tx_hash[..16.min(tx_hash.len())]
                                            ));
                                        }
                                        result.push('\n');
                                    }

                                    events_result.set(result);
                                } else {
                                    events_result.set("❌ Failed to query events".to_string());
                                }
                            }
                            Err(e) => {
                                web_sys::console::error_1(&format!("❌ [REFLECTOR EVENTS] Parse Error: {}", e).into());
                                web_sys::console::log_1(&format!("Raw response was: {}", response_text).into());
                                events_result.set(format!("❌ Parse Error: {}\n\nRaw Response:\n{}", e, response_text));
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("❌ [REFLECTOR EVENTS] Request Error: {}", e).into());
                        events_result.set(format!("❌ Request Error: {}", e));
                    }
                }
                web_sys::console::log_1(&"🏁 [REFLECTOR EVENTS] Query completed".into());
                is_querying_events.set(false);
            });
        })
    };

    html! {
        <div class="reflector-oracle-section">
            <h2>{"🔮 Reflector Oracle - Price Feeds"}</h2>
            <p class="oracle-description">
                {"Reflector Network provides decentralized price oracles for Stellar (SEP-40 compatible). Query real-time crypto and FX prices."}
            </p>

            // Live Price Feed - Auto-updating prices for all assets
            <LivePriceFeed />

            <div class="oracle-grid">
                // Crypto Price Queries
                <div class="oracle-card">
                    <h3>{"💰 Crypto Prices"}</h3>
                    <p class="oracle-info">
                        {"Query cryptocurrency prices from the Reflector Oracle. Supports major assets like BTC, ETH, XLM."}
                    </p>

                    <div class="button-group">
                        <button
                            class="btn btn-oracle"
                            onclick={create_price_query("BTC")}
                            disabled={*is_querying_price}
                        >
                            {"BTC/USD"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_price_query("ETH")}
                            disabled={*is_querying_price}
                        >
                            {"ETH/USD"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_price_query("XLM")}
                            disabled={*is_querying_price}
                        >
                            {"XLM/USD"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_price_query("SOL")}
                            disabled={*is_querying_price}
                        >
                            {"SOL/USD"}
                        </button>
                    </div>

                    <textarea
                        class="result-textarea"
                        readonly=true
                        value={(*price_result).clone()}
                        rows="14"
                    />
                </div>

                // Additional Crypto & Stablecoin Prices
                <div class="oracle-card">
                    <h3>{"💰 More Assets"}</h3>
                    <p class="oracle-info">
                        {"Additional crypto assets and stablecoins supported by the Reflector Oracle on testnet."}
                    </p>

                    <div class="button-group">
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("USDT")}
                            disabled={*is_querying_fx}
                        >
                            {"USDT"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("XRP")}
                            disabled={*is_querying_fx}
                        >
                            {"XRP"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("USDC")}
                            disabled={*is_querying_fx}
                        >
                            {"USDC"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("ADA")}
                            disabled={*is_querying_fx}
                        >
                            {"ADA"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("AVAX")}
                            disabled={*is_querying_fx}
                        >
                            {"AVAX"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("DOT")}
                            disabled={*is_querying_fx}
                        >
                            {"DOT"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("MATIC")}
                            disabled={*is_querying_fx}
                        >
                            {"MATIC"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("LINK")}
                            disabled={*is_querying_fx}
                        >
                            {"LINK"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("DAI")}
                            disabled={*is_querying_fx}
                        >
                            {"DAI"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("ATOM")}
                            disabled={*is_querying_fx}
                        >
                            {"ATOM"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("UNI")}
                            disabled={*is_querying_fx}
                        >
                            {"UNI"}
                        </button>
                        <button
                            class="btn btn-oracle"
                            onclick={create_fx_query("EURC")}
                            disabled={*is_querying_fx}
                        >
                            {"EURC"}
                        </button>
                    </div>

                    <textarea
                        class="result-textarea"
                        readonly=true
                        value={(*fx_result).clone()}
                        rows="14"
                    />
                </div>

                // Oracle Contract Info
                <div class="oracle-card oracle-card-full">
                    <h3>{"📊 Oracle Contract Information"}</h3>
                    <p class="oracle-info">
                        {"View detailed information about the Reflector Oracle contracts including metadata, pool stats, and circuit breaker status."}
                    </p>
                    <button
                        class="btn btn-oracle"
                        onclick={on_get_oracle_info}
                        disabled={*is_querying_info}
                    >
                        {if *is_querying_info { "Fetching..." } else { "Get Oracle Info" }}
                    </button>
                    <textarea
                        class="result-textarea"
                        readonly=true
                        value={(*info_result).clone()}
                        rows="14"
                    />
                </div>

                // Oracle Events
                <div class="oracle-card oracle-card-full">
                    <h3>{"📡 Oracle Contract Events"}</h3>
                    <p class="oracle-info">
                        {"Query historical events emitted by the Reflector Oracle contract. Shows recent price updates, admin actions, and contract interactions."}
                    </p>
                    <button
                        class="btn btn-oracle"
                        onclick={on_query_events}
                        disabled={*is_querying_events}
                    >
                        {if *is_querying_events { "Querying..." } else { "Query Events" }}
                    </button>
                    <textarea
                        class="result-textarea"
                        readonly=true
                        value={(*events_result).clone()}
                        rows="14"
                    />
                </div>
            </div>

            <div class="oracle-info-footer">
                <div class="oracle-contracts">
                    <div>
                        <strong>{"Crypto Oracle: "}</strong>
                        <code class="contract-id">{REFLECTOR_ORACLE_ID}</code>
                    </div>
                    <div>
                        <strong>{"FX Oracle: "}</strong>
                        <code class="contract-id">{REFLECTOR_FX_ID}</code>
                    </div>
                </div>
                <p class="oracle-note">
                    {"💡 Reflector Network provides decentralized, tamper-proof price feeds used by major Stellar DeFi protocols like Blend, YieldBlox, and Phoenix."}
                </p>
                <p class="oracle-warning">
                    {"⚠️ Note: Direct storage queries shown here are for demonstration. Production apps should call oracle-specific functions for accurate price data."}
                </p>
            </div>
        </div>
    }
}
