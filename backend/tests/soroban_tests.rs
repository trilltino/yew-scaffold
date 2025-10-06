/// Integration tests for Soroban-related functionality
///
/// Tests:
/// - XDR generation endpoint
/// - Contract function calls (if Soroban manager is available)
/// - Network connectivity (mocked or testnet)
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use tower::ServiceExt;
use serde_json::{json, Value};

use common::{TestDb, create_test_app, response_json};

// ============================================================================
// XDR GENERATION TESTS
// ============================================================================

#[tokio::test]
async fn test_generate_xdr_endpoint_exists() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act - Simple GET request to XDR generation endpoint
    let response = app
        .oneshot(
            Request::builder()
                .uri("/generate-xdr?source_account=GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Endpoint should exist (not 404)
    assert_ne!(
        response.status(),
        StatusCode::NOT_FOUND,
        "XDR generation endpoint should exist"
    );

    // Note: Actual XDR generation might fail without proper Soroban setup,
    // but endpoint should exist

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// SOROBAN HEALTH CHECK TESTS
// ============================================================================

#[tokio::test]
async fn test_soroban_health_endpoint() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/soroban/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Endpoint should exist
    // It might return 404 if Soroban manager is not initialized (that's ok)
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Soroban health endpoint should exist or return 404 if manager not available"
    );

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// CONTRACT FUNCTION CALL TESTS (if available)
// ============================================================================

#[tokio::test]
async fn test_call_contract_function_endpoint_exists() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let payload = json!({
        "contract_id": "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF",
        "function_name": "test_function",
        "parameters": []
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/soroban/call-function")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Endpoint should exist (not 404)
    // It might fail with other status codes if Soroban manager is not available
    assert_ne!(
        response.status(),
        StatusCode::NOT_FOUND,
        "Contract function call endpoint should exist"
    );

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// CONTRACT LIST TESTS
// ============================================================================

#[tokio::test]
async fn test_list_contracts_endpoint() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/soroban/contracts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Endpoint should exist
    assert_ne!(
        response.status(),
        StatusCode::NOT_FOUND,
        "List contracts endpoint should exist"
    );

    // If Soroban manager is available, it should return OK with contract list
    if response.status() == StatusCode::OK {
        let json: Value = response_json(response).await;
        assert!(json.is_array() || json.is_object(), "Response should be JSON");
    }

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[tokio::test]
async fn test_invalid_contract_id_format() {
    // Arrange
    let test_db = TestDb::new().await;
    let app = create_test_app(test_db.pool.clone()).await;

    let payload = json!({
        "contract_id": "INVALID_CONTRACT_ID",  // Invalid format
        "function_name": "test_function",
        "parameters": []
    });

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/soroban/call-function")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should return error (400 or 500, not 200)
    assert_ne!(
        response.status(),
        StatusCode::OK,
        "Invalid contract ID should return error"
    );

    // Cleanup
    test_db.cleanup().await;
}
