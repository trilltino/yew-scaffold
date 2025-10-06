/// Integration tests for health check endpoints
///
/// Tests:
/// - GET /health returns 200 OK
/// - Health response contains service information
/// - Health endpoint works even without authentication
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use serde_json::Value;

use common::{TestDb, create_test_app, response_json};

#[tokio::test]
async fn test_health_check_returns_ok() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_health_check_response_structure() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response_json(response).await;

    // Verify response has expected fields
    assert!(json.get("status").is_some(), "Health response should have 'status' field");
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("healthy"),
        "Status should be 'healthy'"
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_health_check_no_auth_required() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Send request without any authentication headers
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                // No Authorization header
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should still succeed
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Health check should work without authentication"
    );

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_404_not_found() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/this-route-does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "Non-existent routes should return 404"
    );

    // Cleanup
    test_db.cleanup().await;
}
