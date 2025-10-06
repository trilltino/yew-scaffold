/// Integration tests for middleware functionality
///
/// Tests:
/// - Auth middleware on protected routes
/// - CORS headers are correctly set
/// - JWT token validation in middleware
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use tower::{ServiceExt, Service};
use serde_json::Value;

use common::{TestDb, TestUser, create_test_app, response_json};

// ============================================================================
// AUTH MIDDLEWARE TESTS
// ============================================================================

#[tokio::test]
async fn test_protected_endpoint_requires_auth() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Access protected endpoint without auth
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
        "Protected routes should require authentication"
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_protected_endpoint_with_valid_auth() {
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

    // Act - Access protected endpoint with valid token
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
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Valid auth should grant access to protected routes"
    );

    let json: Value = response_json(response).await;
    assert_eq!(
        json.get("data").and_then(|d| d.get("id")).and_then(|v| v.as_i64()),
        Some(test_user.id as i64)
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_expired_token_rejected() {
    // Arrange
    let test_db = TestDb::new().await;
    let test_user = TestUser::new().create_in_db(&test_db.pool).await;

    // Create token with -1 hour expiration (already expired)
    let config = stellar_xdr_service::AppConfig {
        jwt_secret: "test-secret-key-with-minimum-32-characters-for-testing!".to_string(),
        jwt_expiration_hours: -1, // Expired immediately
        ..stellar_xdr_service::AppConfig::default()
    };

    let expired_token = stellar_xdr_service::auth::encode_jwt(
        test_user.id,
        test_user.username.clone(),
        test_user.role.clone(),
        &config.jwt_secret,
        config.jwt_expiration_hours,
    )
    .unwrap();

    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Try to access with expired token
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header(header::COOKIE, format!("test_auth={}", expired_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Expired tokens should be rejected"
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_malformed_token_rejected() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Try to access with malformed token
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header(header::COOKIE, "test_auth=this.is.not.a.valid.jwt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Malformed tokens should be rejected"
    );

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// CORS MIDDLEWARE TESTS
// ============================================================================

#[tokio::test]
async fn test_cors_headers_present() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Make a request to see CORS headers
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header(header::ORIGIN, "http://localhost:8080")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - CORS headers should be present
    assert!(
        response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_some(),
        "CORS headers should be present"
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_public_endpoints_no_auth() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let public_endpoints = vec![
        "/health",
        "/api/auth/login",
        "/api/auth/signup",
        "/api/auth/logout",
        "/api/auth/register-guest",
    ];

    // Act & Assert - All public endpoints should work without auth
    for endpoint in public_endpoints {
        let method = if endpoint.contains("login") || endpoint.contains("signup") || endpoint.contains("register") {
            "POST"
        } else if endpoint.contains("logout") {
            "POST"
        } else {
            "GET"
        };

        let request_builder = Request::builder()
            .method(method)
            .uri(endpoint);

        let request = if method == "POST" {
            request_builder
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap()
        } else {
            request_builder
                .body(Body::empty())
                .unwrap()
        };

        let mut app_clone = create_test_app(test_db.pool.clone()).await;
        let response = ServiceExt::<Request<Body>>::ready(&mut app_clone)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        // Should NOT return 401 Unauthorized (might return 400 Bad Request for invalid JSON, that's ok)
        assert_ne!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "Endpoint {} should be accessible without auth",
            endpoint
        );
    }

    // Cleanup
    test_db.cleanup().await;
}
