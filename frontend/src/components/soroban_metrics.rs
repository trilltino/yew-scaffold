use yew::prelude::*;
use gloo_timers::callback::Interval;
use crate::services::SorobanApiClient;
use shared::dto::soroban::{MetricsResponse, SorobanHealthResponse};

#[function_component(SorobanMetrics)]
pub fn soroban_metrics() -> Html {
    let metrics = use_state(|| Option::<MetricsResponse>::None);
    let health = use_state(|| Option::<SorobanHealthResponse>::None);
    let error = use_state(|| Option::<String>::None);

    // Fetch initial data
    {
        let metrics = metrics.clone();
        let health = health.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = SorobanApiClient::new();

                match client.get_metrics().await {
                    Ok(m) => metrics.set(Some(m)),
                    Err(e) => error.set(Some(format!("Metrics error: {}", e))),
                }

                match client.get_health().await {
                    Ok(h) => health.set(Some(h)),
                    Err(e) => error.set(Some(format!("Health error: {}", e))),
                }
            });
        });
    }

    // Auto-refresh every 10 seconds
    {
        let metrics = metrics.clone();
        let health = health.clone();

        use_effect_with((), move |_| {
            let interval = Interval::new(10_000, move || {
                let metrics = metrics.clone();
                let health = health.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let client = SorobanApiClient::new();

                    if let Ok(m) = client.get_metrics().await {
                        metrics.set(Some(m));
                    }

                    if let Ok(h) = client.get_health().await {
                        health.set(Some(h));
                    }
                });
            });

            move || drop(interval)
        });
    }

    html! {
        <div class="soroban-metrics">
            <h2>{"Soroban Service Metrics"}</h2>

            if let Some(err) = (*error).clone() {
                <div class="error-message">
                    <p>{err}</p>
                </div>
            }

            if let Some(h) = (*health).clone() {
                <div class="health-status">
                    <h3>{"Health Status"}</h3>
                    <div class="metric-grid">
                        <div class="metric-item">
                            <span class="metric-label">{"Status:"}</span>
                            <span class={if h.health.healthy { "status-healthy" } else { "status-unhealthy" }}>
                                {if h.health.healthy { "✅ Healthy" } else { "❌ Unhealthy" }}
                            </span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Total Contracts:"}</span>
                            <span>{h.health.total_contracts}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Enabled Contracts:"}</span>
                            <span>{h.health.enabled_contracts}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Cache Hit Rate:"}</span>
                            <span>{format!("{:.2}%", h.health.cache_hit_rate)}</span>
                        </div>
                    </div>
                </div>
            }

            if let Some(m) = (*metrics).clone() {
                <div class="metrics-display">
                    <h3>{"Performance Metrics"}</h3>
                    <div class="metric-grid">
                        <div class="metric-item">
                            <span class="metric-label">{"Total Operations:"}</span>
                            <span>{m.metrics.total_operations}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Successful:"}</span>
                            <span class="metric-success">{m.metrics.successful_operations}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Failed:"}</span>
                            <span class="metric-error">{m.metrics.failed_operations}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Retried:"}</span>
                            <span>{m.metrics.retried_operations}</span>
                        </div>
                    </div>

                    <h3>{"Cache Performance"}</h3>
                    <div class="metric-grid">
                        <div class="metric-item">
                            <span class="metric-label">{"Cache Hits:"}</span>
                            <span class="metric-success">{m.metrics.cache_hits}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Cache Misses:"}</span>
                            <span>{m.metrics.cache_misses}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"XDR Generated:"}</span>
                            <span>{m.metrics.xdr_generated}</span>
                        </div>
                        <div class="metric-item">
                            <span class="metric-label">{"Transactions Submitted:"}</span>
                            <span>{m.metrics.transactions_submitted}</span>
                        </div>
                    </div>
                </div>
            } else {
                <p>{"Loading metrics..."}</p>
            }
        </div>
    }
}
