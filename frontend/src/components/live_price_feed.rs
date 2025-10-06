use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::callback::Interval;
use std::collections::HashMap;
use serde_json;

use shared::dto::soroban::*;

const BACKEND_URL: &str = "http://127.0.0.1:3001";
const REFLECTOR_ORACLE_ID: &str = "CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63";

// All supported assets on testnet
const ASSETS: &[&str] = &[
    "BTC", "ETH", "XLM", "SOL", "USDT", "XRP", "USDC",
    "ADA", "AVAX", "DOT", "MATIC", "LINK", "DAI", "ATOM", "UNI", "EURC"
];

#[derive(Clone, PartialEq)]
struct AssetPrice {
    symbol: String,
    price: Option<String>,
    formatted_price: Option<String>,
    timestamp: Option<u64>,
    loading: bool,
}

#[function_component(LivePriceFeed)]
pub fn live_price_feed() -> Html {
    let prices = use_state(|| {
        let mut map = HashMap::new();
        for &asset in ASSETS {
            map.insert(
                asset.to_string(),
                AssetPrice {
                    symbol: asset.to_string(),
                    price: None,
                    formatted_price: None,
                    timestamp: None,
                    loading: true,
                },
            );
        }
        map
    });

    let is_paused = use_state(|| false);

    // Format price with 14 decimals
    let format_price = |price_str: &str| -> String {
        if let Ok(price) = price_str.parse::<f64>() {
            let actual_price = price / 100_000_000_000_000.0;

            if actual_price >= 1000.0 {
                let formatted = format!("{:.2}", actual_price);
                let parts: Vec<&str> = formatted.split('.').collect();
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
            } else if actual_price >= 1.0 {
                format!("${:.4}", actual_price)
            } else if actual_price >= 0.01 {
                format!("${:.6}", actual_price)
            } else {
                format!("${:.8}", actual_price)
            }
        } else {
            "—".to_string()
        }
    };

    // Fetch price for a single asset
    let fetch_price = {
        let prices = prices.clone();

        move |asset: String| {
            let prices = prices.clone();
            let format_price_inner = format_price;

            spawn_local(async move {
                let request = CallContractFunctionRequest {
                    contract_id: REFLECTOR_ORACLE_ID.to_string(),
                    function_name: "lastprice".to_string(),
                    parameters: vec![
                        FunctionParameter::Enum(
                            "Other".to_string(),
                            Some(Box::new(FunctionParameter::Symbol(asset.clone())))
                        )
                    ],
                    source_account: None,
                };

                let url = format!("{}/api/soroban/call-function", BACKEND_URL);

                match Request::post(&url).json(&request).unwrap().send().await {
                    Ok(response) => {
                        if let Ok(response_text) = response.text().await {
                            if let Ok(data) = serde_json::from_str::<CallContractFunctionResponse>(&response_text) {
                                let mut new_prices = (*prices).clone();

                                if data.success {
                                    if let Some(result_obj) = data.result.as_ref().and_then(|r| r.as_object()) {
                                        let price_opt = result_obj.get("price").and_then(|p| p.as_str()).map(|s| s.to_string());
                                        let formatted = price_opt.as_ref().map(|p| format_price_inner(p));
                                        let timestamp = result_obj.get("timestamp").and_then(|t| t.as_u64());

                                        new_prices.insert(
                                            asset.clone(),
                                            AssetPrice {
                                                symbol: asset.clone(),
                                                price: price_opt,
                                                formatted_price: formatted,
                                                timestamp,
                                                loading: false,
                                            },
                                        );
                                    } else {
                                        // Null result or non-object - asset not available on this oracle
                                        new_prices.insert(
                                            asset.clone(),
                                            AssetPrice {
                                                symbol: asset.clone(),
                                                price: None,
                                                formatted_price: None,
                                                timestamp: None,
                                                loading: false,
                                            },
                                        );
                                    }
                                } else {
                                    // Request failed
                                    if let Some(asset_price) = new_prices.get_mut(&asset) {
                                        asset_price.loading = false;
                                    }
                                }
                                prices.set(new_prices);
                            }
                        }
                    }
                    Err(_) => {
                        let mut new_prices = (*prices).clone();
                        if let Some(asset_price) = new_prices.get_mut(&asset) {
                            asset_price.loading = false;
                        }
                        prices.set(new_prices);
                    }
                }
            });
        }
    };

    // Initial load and auto-refresh
    {
        let fetch_price = fetch_price.clone();
        let is_paused = is_paused.clone();

        use_effect_with((), move |_| {
            // Initial fetch for all assets
            for &asset in ASSETS {
                fetch_price(asset.to_string());
            }

            // Auto-refresh every 30 seconds
            let interval = Interval::new(30_000, move || {
                if !*is_paused {
                    for &asset in ASSETS {
                        fetch_price(asset.to_string());
                    }
                }
            });

            move || drop(interval)
        });
    }

    let toggle_pause = {
        let is_paused = is_paused.clone();
        Callback::from(move |_| {
            is_paused.set(!*is_paused);
        })
    };

    html! {
        <div class="live-price-feed">
            <div class="feed-header">
                <h2>{"📊 Live Price Feed"}</h2>
                <button class="btn btn-toggle" onclick={toggle_pause}>
                    {if *is_paused { "▶ Resume" } else { "⏸ Pause" }}
                </button>
            </div>
            <p class="feed-description">
                {"Real-time cryptocurrency and stablecoin prices from Reflector Oracle (updates every 30s)"}
            </p>

            <div class="price-grid">
                {ASSETS.iter().map(|&asset| {
                    let asset_data = prices.get(asset).cloned().unwrap_or_else(|| AssetPrice {
                        symbol: asset.to_string(),
                        price: None,
                        formatted_price: None,
                        timestamp: None,
                        loading: true,
                    });

                    html! {
                        <div class="price-card" key={asset}>
                            <div class="price-symbol">{asset}</div>
                            <div class="price-value">
                                {if asset_data.loading {
                                    html! { <span class="loading">{"Loading..."}</span> }
                                } else if let Some(formatted) = &asset_data.formatted_price {
                                    html! {
                                        <>
                                            <span class="price">{formatted}</span>
                                            {if let Some(ts) = asset_data.timestamp {
                                                let now = js_sys::Date::now() as u64 / 1000;
                                                let age_seconds = now.saturating_sub(ts);
                                                let age_display = if age_seconds < 60 {
                                                    "Just now".to_string()
                                                } else if age_seconds < 300 {
                                                    format!("{}m ago", age_seconds / 60)
                                                } else {
                                                    "5m+ ago".to_string()
                                                };
                                                html! { <span class="price-age">{age_display}</span> }
                                            } else {
                                                html! {}
                                            }}
                                        </>
                                    }
                                } else {
                                    html! { <span class="no-data">{"No data"}</span> }
                                }}
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
