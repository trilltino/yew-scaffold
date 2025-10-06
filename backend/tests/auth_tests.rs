/// Integration tests for authentication endpoints
///
/// Tests cover:
/// - User signup with email/password
/// - Login with valid/invalid credentials
/// - Logout functionality
/// - Protected endpoint access with/without auth
/// - Guest user registration (wallet-only)
/// - Wallet linking to existing account
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use tower::ServiceExt; // for `oneshot`
use serde_json::{json, Value};

use common::{TestDb, TestUser, create_test_app, response_json, assertions::*};

// ============================================================================
// SIGNUP TESTS
// ============================================================================

#[tokio::test]
async fn test_signup_success() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let test_user = TestUser::new();
    let payload = json!({
        "username": test_user.username,
        "email": test_user.email,
        "password": test_user.password,
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/signup")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);

    let json: Value = response_json(response).await;
    assert_success(&json);
    assert!(json.get("data").is_some());

    let data = json.get("data").unwrap();
    assert_eq!(
        data.get("username").and_then(|v| v.as_str()),
        Some(test_user.username.as_str())
    );
    assert_eq!(
        data.get("email").and_then(|v| v.as_str()),
        Some(test_user.email.as_str())
    );
    assert!(data.get("user_id").is_some());

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_signup_duplicate_email() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let test_user = TestUser::new().create_in_db(&test_db.pool).await;

    // Try to signup with same email
    let payload = json!({
        "username": "different_username",
        "email": test_user.email,
        "password": "DifferentPass123!",
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/signup")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::CONFLICT);

    let json: Value = response_json(response).await;
    assert_error(&json);

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_signup_invalid_email() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let payload = json!({
        "username": "testuser",
        "email": "not-an-email",  // Invalid email format
        "password": "Test123!",
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/signup")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// LOGIN TESTS
// ============================================================================

#[tokio::test]
async fn test_login_success() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let test_user = TestUser::new().create_in_db(&test_db.pool).await;

    let payload = json!({
        "email": test_user.email,
        "password": test_user.password,
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response_json(response).await;
    assert_success(&json);

    let data = json.get("data").unwrap();
    assert!(data.get("token").is_some(), "Login should return JWT token");
    assert_eq!(
        data.get("username").and_then(|v| v.as_str()),
        Some(test_user.username.as_str())
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_login_wrong_password() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let test_user = TestUser::new().create_in_db(&test_db.pool).await;

    let payload = json!({
        "email": test_user.email,
        "password": "WrongPassword123!",
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let json: Value = response_json(response).await;
    assert_error(&json);

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let payload = json!({
        "email": "nonexistent@example.com",
        "password": "Password123!",
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// LOGOUT TESTS
// ============================================================================

#[tokio::test]
async fn test_logout_success() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/logout")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    // Check that Set-Cookie header is present (clearing the cookie)
    assert!(
        response.headers().get(header::SET_COOKIE).is_some(),
        "Logout should set a cookie to clear authentication"
    );

    let json: Value = response_json(response).await;
    assert_success(&json);

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// PROTECTED ENDPOINT TESTS (GET /api/auth/me)
// ============================================================================

#[tokio::test]
async fn test_me_endpoint_with_valid_token() {
    // Arrange
    let test_db = TestDb::new().await;
    let test_user = TestUser::new().create_in_db(&test_db.pool).await;

    let config = stellar_xdr_service::AppConfig {
        jwt_secret: "test-secret-key-with-minimum-32-characters-for-testing!".to_string(),
        jwt_expiration_hours: 24,
        ..stellar_xdr_service::AppConfig::default()
    };

    let token = test_user.get_token(&config);
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Send request with valid JWT token in cookie
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header(header::COOKIE, format!("test_auth={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response_json(response).await;
    assert_success(&json);

    let data = json.get("data").unwrap();
    assert_eq!(
        data.get("username").and_then(|v| v.as_str()),
        Some(test_user.username.as_str())
    );
    assert_eq!(
        data.get("id").and_then(|v| v.as_i64()),
        Some(test_user.id as i64)
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_me_endpoint_without_token() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Send request without authentication
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Protected endpoint should return 401 without auth"
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_me_endpoint_with_invalid_token() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Send request with invalid token
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header(header::COOKIE, "test_auth=invalid.jwt.token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Protected endpoint should reject invalid tokens"
    );

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// GUEST REGISTRATION TESTS (wallet-only users)
// ============================================================================

#[tokio::test]
async fn test_guest_registration() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let wallet_address = "GBTEST123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890AB";
    let payload = json!({
        "username": "wallet_user",
        "wallet_address": wallet_address,
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register-guest")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response_json(response).await;
    assert_success(&json);

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// WALLET LINKING TESTS
// ============================================================================

#[tokio::test]
async fn test_link_wallet_to_existing_account() {
    // Arrange
    let test_db = TestDb::new().await;
    let test_user = TestUser::new().create_in_db(&test_db.pool).await;

    let config = stellar_xdr_service::AppConfig {
        jwt_secret: "test-secret-key-with-minimum-32-characters-for-testing!".to_string(),
        jwt_expiration_hours: 24,
        ..stellar_xdr_service::AppConfig::default()
    };

    let token = test_user.get_token(&config);
    let app = create_test_app(test_db.pool.clone()).await;

    let new_wallet = "GBNEWWALLET123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ12345";
    let payload = json!({
        "wallet_address": new_wallet,
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/link-wallet")
                .header(header::COOKIE, format!("test_auth={}", token))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response_json(response).await;
    assert_success(&json);

    // Verify wallet was linked
    let data = json.get("data").unwrap();
    assert_eq!(
        data.get("wallet_address").and_then(|v| v.as_str()),
        Some(new_wallet)
    );

    // Cleanup
    test_db.cleanup().await;
}
