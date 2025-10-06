pub mod config;
pub mod database;
pub mod error;
pub mod handlers;
pub mod services;
pub mod types;
pub mod utils;

// Authentication modules
pub mod auth;
pub mod middleware;
pub mod extractors;

use axum::{routing::{get, post}, Router, middleware as axum_middleware};
use tower_cookies::CookieManagerLayer;
use tracing::info;

pub use config::{AppConfig, AppState};
pub use error::{AppError, Result};
pub use handlers::{generate_xdr_handler, submit_transaction_handler, health_handler};
pub use handlers::soroban::{metrics_handler, contract_info_handler, soroban_health_handler, list_contracts_handler};
pub use types::{XdrRequest, XdrResponse, SubmitRequest, SubmitResponse, HealthResponse};
pub use utils::create_cors_layer;

// Auth re-exports
pub use auth::{encode_jwt, decode_jwt, hash_password, verify_password, create_auth_cookie, create_logout_cookie};
pub use middleware::{auth_middleware, require_admin, require_chapter_lead};
pub use extractors::CurrentUser;

pub async fn create_app(config: AppConfig, db_pool: sqlx::PgPool) -> Result<Router> {
    // Try to create state with Soroban manager and database pool
    let state = match AppState::with_soroban_manager(config.clone(), db_pool.clone()).await {
        Ok(s) => {
            info!("✅ AppState initialized with Soroban manager and database");
            s
        }
        Err(e) => {
            info!("⚠️  Soroban manager initialization failed, using basic state: {}", e);
            AppState::new(config.clone(), db_pool.clone())?
        }
    };

    // Public routes (no authentication required)
    let mut app = Router::new()
        .route("/generate-xdr", get(generate_xdr_handler))
        .route("/submit-transaction", post(submit_transaction_handler))
        .route("/health", get(health_handler));

    // Add Soroban routes if manager is available
    if state.soroban_manager.is_some() {
        info!("Registering Soroban advanced routes");
        app = app
            .route("/api/soroban/metrics", get(metrics_handler))
            .route("/api/soroban/health", get(soroban_health_handler))
            .route("/api/soroban/contracts", get(list_contracts_handler))
            .route("/api/soroban/contract/{id}", get(contract_info_handler))
            .route("/api/soroban/events", post(handlers::soroban::query_events_handler))
            .route("/api/soroban/simulate", post(handlers::soroban::simulate_transaction_handler))
            .route("/api/soroban/contract-data", post(handlers::soroban::get_contract_data_handler))
            .route("/api/soroban/call-function", post(handlers::soroban::call_contract_function_handler));
        info!("Soroban routes registered successfully (events, simulation, state querying, function calls)");
    } else {
        info!("WARNING: Soroban routes NOT registered (manager not initialized)");
    }

    // Add public auth routes (signup, login, logout)
    info!("Registering public auth routes");
    app = app
        .route("/api/auth/register-guest", post(handlers::auth::register_guest))
        .route("/api/auth/signup", post(handlers::auth::signup_with_password))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/logout", post(handlers::auth::logout));
    info!("Public auth routes registered: /api/auth/{{register-guest,signup,login,logout}}");

    // Protected routes (require authentication)
    info!("Registering protected auth routes");
    let protected_routes = Router::new()
        .route("/api/auth/me", get(handlers::auth::me))
        .route("/api/auth/link-wallet", post(handlers::auth::link_wallet))
        .layer(axum_middleware::from_fn_with_state(state.clone(), auth_middleware));
    info!("Protected auth routes registered: /api/auth/{{me,link-wallet}}");

    // Merge protected routes with main app
    let mut app = app
        .merge(protected_routes)
        .with_state(state);

    // CRITICAL: Add CookieManagerLayer AFTER routes, BEFORE CORS
    info!("Adding CookieManagerLayer");
    app = app.layer(CookieManagerLayer::new());

    // Add CORS layer (must be last)
    let app = app.layer(create_cors_layer(config.allowed_origins.clone()));

    Ok(app)
}

pub async fn run_server(config: AppConfig) -> Result<()> {
    info!("Service configuration loaded");
    info!("  Port: {}", config.port);
    info!("  Contract ID: {}", config.contract_id);
    info!("  Allowed origins: {:?}", config.allowed_origins);

    // Connect to database (REQUIRED for auth features)
    info!("Connecting to database...");
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::Config(
            "DATABASE_URL not set. Please set DATABASE_URL in backend/.env file".to_string()
        ))?;

    info!("DATABASE_URL found: {}...", &database_url[..50.min(database_url.len())]);

    let db_pool = database::connection::create_pool(&database_url).await
        .map_err(|e| AppError::Database(format!("Failed to connect to database: {}", e)))?;

    info!("✅ Database connected successfully");

    let app = create_app(config.clone(), db_pool).await?;

    let bind_address = format!("127.0.0.1:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .map_err(|e| AppError::Config(format!("Failed to bind to address {}: {}", bind_address, e)))?;

    info!("Stellar XDR Service running on http://{}", bind_address);
    info!("Health check: http://{}/health", bind_address);
    info!("Auth endpoints: http://{}/api/auth/{{signup,login,logout,me}}", bind_address);

    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::Internal(format!("Server error: {}", e)))?;

    Ok(())
}