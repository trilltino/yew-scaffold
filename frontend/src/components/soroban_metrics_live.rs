use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::callback::Interval;

use shared::dto::soroban::*;

const BACKEND_URL: &str = "http://127.0.0.1:3001";
const CONTRACT_ID: &str = "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF";

#[function_component(SorobanMetricsLive)]
pub fn soroban_metrics_live() -> Html {
    let metrics = use_state(|| None::<ContractMetrics>);
    let health = use_state(|| None::<HealthStatus>);
    let circuit_stats = use_state(|| None::<CircuitBreakerStats>);
    let pool_stats = use_state(|| None::<PoolStats>);
    let cache_stats = use_state(|| None::<CacheStats>);
    let last_update = use_state(|| "Never".to_string());

    // Fetch all metrics
    let fetch_metrics = {
        let metrics = metrics.clone();
        let health = health.clone();
        let circuit_stats = circuit_stats.clone();
        let pool_stats = pool_stats.clone();
        let cache_stats = cache_stats.clone();
        let last_update = last_update.clone();

        Callback::from(move |_| {
            let metrics = metrics.clone();
            let health = health.clone();
            let circuit_stats = circuit_stats.clone();
            let pool_stats = pool_stats.clone();
            let cache_stats = cache_stats.clone();
            let last_update = last_update.clone();

            spawn_local(async move {
                // Fetch metrics
                if let Ok(response) = Request::get(&format!("{}/api/soroban/metrics", BACKEND_URL)).send().await {
                    if let Ok(data) = response.json::<MetricsResponse>().await {
                        metrics.set(Some(data.metrics));
                    }
                }

                // Fetch health
                if let Ok(response) = Request::get(&format!("{}/api/soroban/health", BACKEND_URL)).send().await {
                    if let Ok(data) = response.json::<SorobanHealthResponse>().await {
                        health.set(Some(data.health));
                    }
                }

                // Fetch contract info (includes circuit breaker, pool, cache)
                if let Ok(response) = Request::get(&format!("{}/api/soroban/contract/{}", BACKEND_URL, CONTRACT_ID)).send().await {
                    if let Ok(data) = response.json::<ContractInfoResponse>().await {
                        circuit_stats.set(Some(data.info.circuit_breaker_stats));
                        pool_stats.set(Some(data.info.pool_stats));
                        cache_stats.set(Some(data.info.cache_stats));
                    }
                }

                // Update timestamp
                let now = js_sys::Date::new_0();
                let hours = now.get_hours();
                let minutes = now.get_minutes();
                let seconds = now.get_seconds();
                let time_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
                last_update.set(time_str);
            });
        })
    };

    // Auto-refresh every 2 seconds
    {
        let fetch_metrics = fetch_metrics.clone();
        use_effect_with((), move |_| {
            // Initial fetch
            fetch_metrics.emit(());

            // Set up interval for auto-refresh
            let interval = Interval::new(2000, move || {
                fetch_metrics.emit(());
            });

            // Cleanup function
            move || drop(interval)
        });
    }

    // Calculate cache hit rate
    let cache_hit_rate = if let Some(m) = metrics.as_ref() {
        if m.cache_hits + m.cache_misses > 0 {
            (m.cache_hits as f64 / (m.cache_hits + m.cache_misses) as f64) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };

    // Calculate success rate
    let success_rate = if let Some(m) = metrics.as_ref() {
        if m.total_operations > 0 {
            (m.successful_operations as f64 / m.total_operations as f64) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };

    html! {
        <div class="soroban-metrics-live">
            <div class="metrics-header">
                <h2>{"📊 Live Backend Performance Metrics"}</h2>
                <div class="last-update">
                    {"Last updated: "}{&*last_update}
                    <span class="auto-refresh">{"🔄 Auto-refresh: 2s"}</span>
                </div>
            </div>

            <div class="metrics-grid">
                // Health Status Card
                <div class="metric-card health-card">
                    <div class="metric-icon">{"💚"}</div>
                    <h3>{"System Health"}</h3>
                    {if let Some(h) = health.as_ref() {
                        html! {
                            <>
                                <div class={classes!("health-status", if h.healthy { "healthy" } else { "unhealthy" })}>
                                    {if h.healthy { "✅ HEALTHY" } else { "❌ DEGRADED" }}
                                </div>
                                <div class="metric-details">
                                    <div class="detail-row">
                                        <span>{"Contracts:"}</span>
                                        <span class="detail-value">{format!("{}/{}", h.enabled_contracts, h.total_contracts)}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Operations:"}</span>
                                        <span class="detail-value">{h.total_operations}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Failed:"}</span>
                                        <span class="detail-value">{h.failed_operations}</span>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! { <div class="loading">{"Loading..."}</div> }
                    }}
                </div>

                // Cache Performance Card
                <div class="metric-card cache-card">
                    <div class="metric-icon">{"⚡"}</div>
                    <h3>{"Cache Performance"}</h3>
                    {if let Some(m) = metrics.as_ref() {
                        html! {
                            <>
                                <div class="big-stat">
                                    {format!("{:.1}%", cache_hit_rate)}
                                </div>
                                <div class="stat-label">{"Hit Rate"}</div>
                                <div class="metric-details">
                                    <div class="detail-row">
                                        <span>{"Hits:"}</span>
                                        <span class="detail-value success">{m.cache_hits}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Misses:"}</span>
                                        <span class="detail-value">{m.cache_misses}</span>
                                    </div>
                                    {if let Some(c) = cache_stats.as_ref() {
                                        html! {
                                            <div class="detail-row">
                                                <span>{"Active Entries:"}</span>
                                                <span class="detail-value">{c.active_entries}</span>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            </>
                        }
                    } else {
                        html! { <div class="loading">{"Loading..."}</div> }
                    }}
                </div>

                // Success Rate Card
                <div class="metric-card success-card">
                    <div class="metric-icon">{"🎯"}</div>
                    <h3>{"Success Rate"}</h3>
                    {if let Some(m) = metrics.as_ref() {
                        html! {
                            <>
                                <div class="big-stat">
                                    {format!("{:.1}%", success_rate)}
                                </div>
                                <div class="stat-label">{"Reliability"}</div>
                                <div class="metric-details">
                                    <div class="detail-row">
                                        <span>{"Successful:"}</span>
                                        <span class="detail-value success">{m.successful_operations}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Failed:"}</span>
                                        <span class="detail-value error">{m.failed_operations}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Retried:"}</span>
                                        <span class="detail-value warning">{m.retried_operations}</span>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! { <div class="loading">{"Loading..."}</div> }
                    }}
                </div>

                // Circuit Breaker Card
                <div class="metric-card circuit-card">
                    <div class="metric-icon">{"🔌"}</div>
                    <h3>{"Circuit Breaker"}</h3>
                    {if let Some(c) = circuit_stats.as_ref() {
                        html! {
                            <>
                                <div class={classes!("circuit-status",
                                    match c.state {
                                        CircuitState::Closed => "closed",
                                        CircuitState::Open => "open",
                                        CircuitState::HalfOpen => "half-open",
                                    }
                                )}>
                                    {match c.state {
                                        CircuitState::Closed => "✅ CLOSED",
                                        CircuitState::Open => "🔴 OPEN",
                                        CircuitState::HalfOpen => "🟡 HALF-OPEN",
                                    }}
                                </div>
                                <div class="metric-details">
                                    <div class="detail-row">
                                        <span>{"Failures:"}</span>
                                        <span class="detail-value">{c.failure_count}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Successes:"}</span>
                                        <span class="detail-value">{c.success_count}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Status:"}</span>
                                        <span class="detail-value">{if c.is_open { "Protecting" } else { "Normal" }}</span>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! { <div class="loading">{"Loading..."}</div> }
                    }}
                </div>

                // RPC Pool Card
                <div class="metric-card pool-card">
                    <div class="metric-icon">{"🏊"}</div>
                    <h3>{"Connection Pool"}</h3>
                    {if let Some(p) = pool_stats.as_ref() {
                        let usage_pct = (p.total_connections as f64 / p.max_connections as f64) * 100.0;
                        html! {
                            <>
                                <div class="big-stat">
                                    {format!("{}/{}", p.total_connections, p.max_connections)}
                                </div>
                                <div class="stat-label">{"Connections"}</div>
                                <div class="progress-bar">
                                    <div class="progress-fill" style={format!("width: {}%", usage_pct)}></div>
                                </div>
                                <div class="metric-details">
                                    <div class="detail-row">
                                        <span>{"Available:"}</span>
                                        <span class="detail-value success">{p.available}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"In Use:"}</span>
                                        <span class="detail-value">{p.total_connections - p.available}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Utilization:"}</span>
                                        <span class="detail-value">{format!("{:.1}%", usage_pct)}</span>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! { <div class="loading">{"Loading..."}</div> }
                    }}
                </div>

                // Operations Breakdown Card
                <div class="metric-card operations-card">
                    <div class="metric-icon">{"📈"}</div>
                    <h3>{"Operations Breakdown"}</h3>
                    {if let Some(m) = metrics.as_ref() {
                        html! {
                            <>
                                <div class="operations-stats">
                                    <div class="operation-stat">
                                        <div class="operation-count">{m.xdr_generated}</div>
                                        <div class="operation-label">{"XDR Generated"}</div>
                                    </div>
                                    <div class="operation-stat">
                                        <div class="operation-count">{m.transactions_submitted}</div>
                                        <div class="operation-label">{"Transactions"}</div>
                                    </div>
                                </div>
                                <div class="metric-details">
                                    <div class="detail-row">
                                        <span>{"Total Ops:"}</span>
                                        <span class="detail-value">{m.total_operations}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Success:"}</span>
                                        <span class="detail-value success">{m.successful_operations}</span>
                                    </div>
                                    <div class="detail-row">
                                        <span>{"Failed:"}</span>
                                        <span class="detail-value error">{m.failed_operations}</span>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! { <div class="loading">{"Loading..."}</div> }
                    }}
                </div>
            </div>

            <div class="metrics-footer">
                <p class="performance-note">
                    {"💡 These metrics show the real-time performance of the production-hardened Soroban backend with circuit breaker protection, 50-connection RPC pool, and multi-layer caching (XDR: 1min, Events: 30sec, Simulation: 60sec, State: 5min)."}
                </p>
            </div>
        </div>
    }
}
