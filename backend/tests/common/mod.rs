/// Common test utilities and fixtures for integration tests
///
/// This module provides:
/// - Test database setup and teardown
/// - Test user creation helpers
/// - HTTP client utilities
/// - Assertion helpers

use sqlx::{PgPool, Row};
use stellar_xdr_service::{AppConfig, create_app};
use axum::Router;

/// Test database configuration
pub struct TestDb {
    pub pool: PgPool,
    pub database_name: String,
}

impl TestDb {
    /// Create a new test database with a unique name
    /// Each test gets its own isolated database
    pub async fn new() -> Self {
        // Connect to the postgres database to create test db
        let postgres_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for tests");

        // Extract base URL without database name
        let base_url = if let Some(pos) = postgres_url.rfind('/') {
            &postgres_url[..pos]
        } else {
            &postgres_url
        };

        // Generate unique database name using timestamp and random suffix
        let database_name = format!(
            "test_db_{}_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            rand::random::<u32>()
        );

        // Connect to postgres database
        let postgres_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!("{}/postgres", base_url))
            .await
            .expect("Failed to connect to postgres database");

        // Create test database
        sqlx::query(&format!("CREATE DATABASE {}", database_name))
            .execute(&postgres_pool)
            .await
            .expect("Failed to create test database");

        postgres_pool.close().await;

        // Connect to the new test database
        let test_db_url = format!("{}/{}", base_url, database_name);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&test_db_url)
            .await
            .expect("Failed to connect to test database");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Self {
            pool,
            database_name,
        }
    }

    /// Clean up: drop the test database
    pub async fn cleanup(self) {
        let database_name = self.database_name.clone();

        // Close all connections
        self.pool.close().await;

        // Get base URL
        let postgres_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let base_url = if let Some(pos) = postgres_url.rfind('/') {
            &postgres_url[..pos]
        } else {
            &postgres_url
        };

        // Connect to postgres to drop the test database
        let postgres_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!("{}/postgres", base_url))
            .await
            .expect("Failed to connect to postgres");

        // Terminate existing connections to the test database
        sqlx::query(&format!(
            "SELECT pg_terminate_backend(pg_stat_activity.pid) \
             FROM pg_stat_activity \
             WHERE pg_stat_activity.datname = '{}' \
             AND pid <> pg_backend_pid()",
            database_name
        ))
        .execute(&postgres_pool)
        .await
        .ok();

        // Drop the test database
        sqlx::query(&format!("DROP DATABASE IF EXISTS {}", database_name))
            .execute(&postgres_pool)
            .await
            .expect("Failed to drop test database");

        postgres_pool.close().await;
    }
}

/// Create a test app router with the given database pool
pub async fn create_test_app(pool: PgPool) -> Router {
    let config = AppConfig {
        port: 3001,
        contract_id: "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF".to_string(),
        network_passphrase: "Test SDF Network ; September 2015".to_string(),
        rpc_url: "https://soroban-testnet.stellar.org".to_string(),
        allowed_origins: vec!["http://localhost:8080".to_string()],
        jwt_secret: "test-secret-key-with-minimum-32-characters-for-testing!".to_string(),
        jwt_expiration_hours: 24,
        cookie_name: "test_auth".to_string(),
        cookie_domain: "localhost".to_string(),
        cookie_secure: false,
        cookie_http_only: true,
        cookie_same_site: "Lax".to_string(),
    };

    create_app(config, pool)
        .await
        .expect("Failed to create test app")
}

/// Test user fixture
#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub wallet_address: String,
    pub role: String,
}

impl TestUser {
    /// Create a new test user with default values
    pub fn new() -> Self {
        let random_id = rand::random::<u32>();
        Self {
            id: 0, // Will be set after insertion
            username: format!("testuser_{}", random_id),
            email: format!("test_{}@example.com", random_id),
            password: "Test123!@#".to_string(),
            wallet_address: format!("GTEST{:0<51}", random_id),
            role: "user".to_string(),
        }
    }

    /// Create a test user with admin role
    pub fn admin() -> Self {
        let mut user = Self::new();
        user.role = "admin".to_string();
        user.username = format!("admin_{}", rand::random::<u32>());
        user
    }

    /// Create the user in the database and return the created user with ID
    pub async fn create_in_db(&self, pool: &PgPool) -> Self {
        use stellar_xdr_service::auth::hash_password;

        let password_hash = hash_password(&self.password)
            .expect("Failed to hash password");

        let row = sqlx::query(
            r#"
            INSERT INTO users (username, wallet_address, email, password_hash, role, email_verified, created_at)
            VALUES ($1, $2, $3, $4, $5, false, NOW())
            RETURNING id
            "#
        )
        .bind(&self.username)
        .bind(&self.wallet_address)
        .bind(&self.email)
        .bind(&password_hash)
        .bind(&self.role)
        .fetch_one(pool)
        .await
        .expect("Failed to create test user");

        let id: i32 = row.try_get("id").expect("Failed to get user ID");

        Self {
            id,
            ..self.clone()
        }
    }

    /// Get JWT token for this user
    pub fn get_token(&self, config: &AppConfig) -> String {
        use stellar_xdr_service::auth::encode_jwt;

        encode_jwt(
            self.id,
            self.username.clone(),
            self.role.clone(),
            &config.jwt_secret,
            config.jwt_expiration_hours,
        )
        .expect("Failed to generate test token")
    }
}

impl Default for TestUser {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to extract JSON response body
pub async fn response_json<T>(response: axum::http::Response<axum::body::Body>) -> T
where
    T: serde::de::DeserializeOwned,
{
    use http_body_util::BodyExt;

    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).expect("Failed to parse JSON response")
}

/// Helper to extract response body as string
pub async fn response_text(response: axum::http::Response<axum::body::Body>) -> String {
    use http_body_util::BodyExt;

    let body = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(body.to_vec()).expect("Failed to parse response as UTF-8")
}

/// Custom assertions for API responses
pub mod assertions {
    use axum::http::StatusCode;
    use serde_json::Value;

    /// Assert that a response has the expected status code
    pub fn assert_status(response: &axum::http::Response<axum::body::Body>, expected: StatusCode) {
        assert_eq!(
            response.status(),
            expected,
            "Expected status {}, got {}",
            expected,
            response.status()
        );
    }

    /// Assert that a JSON response has a specific field with expected value
    pub fn assert_json_field(json: &Value, field: &str, expected: &Value) {
        let actual = json.get(field).unwrap_or_else(|| {
            panic!("Field '{}' not found in JSON response: {}", field, json)
        });

        assert_eq!(
            actual, expected,
            "Field '{}': expected {:?}, got {:?}",
            field, expected, actual
        );
    }

    /// Assert that a JSON response contains a success message
    pub fn assert_success(json: &Value) {
        assert_eq!(
            json.get("success").and_then(|v| v.as_bool()),
            Some(true),
            "Expected success=true, got: {}",
            json
        );
    }

    /// Assert that a JSON response contains an error
    pub fn assert_error(json: &Value) {
        assert_eq!(
            json.get("success").and_then(|v| v.as_bool()),
            Some(false),
            "Expected success=false, got: {}",
            json
        );
    }
}
