/// Unit tests for database repositories
///
/// Tests the UserRepository CRUD operations directly
/// without going through HTTP endpoints.
///
/// These are "integration tests" at the database layer,
/// ensuring SQL queries work correctly with a real database.
mod common;

use stellar_xdr_service::database::repositories::user_repository::UserRepository;
use stellar_xdr_service::auth::hash_password;
use common::TestDb;

// ============================================================================
// CREATE OPERATIONS
// ============================================================================

#[tokio::test]
async fn test_create_guest_user() {
    // Arrange
    let test_db = TestDb::new().await;

    // Act
    let user = UserRepository::create_guest(
        &test_db.pool,
        "test_guest",
        "GATEST123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"
    )
    .await
    .expect("Failed to create guest user");

    // Assert
    assert!(user.id > 0);
    assert_eq!(user.username, "test_guest");
    assert_eq!(user.wallet_address, "GATEST123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890");
    assert_eq!(user.role, "user");
    assert_eq!(user.email_verified, false);
    assert!(user.email.is_none());
    assert!(user.password_hash.is_none());

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_create_user_with_password() {
    // Arrange
    let test_db = TestDb::new().await;
    let password_hash = hash_password("Test123!").unwrap();

    // Act
    let user = UserRepository::create_with_password(
        &test_db.pool,
        "testuser",
        "test@example.com",
        &password_hash,
    )
    .await
    .expect("Failed to create user with password");

    // Assert
    assert!(user.id > 0);
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, Some("test@example.com".to_string()));
    assert!(user.password_hash.is_some());
    assert_eq!(user.role, "user");
    assert_eq!(user.wallet_address, ""); // Empty for password users initially

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_create_duplicate_wallet_address() {
    // Arrange
    let test_db = TestDb::new().await;
    let wallet = "GADUP123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

    // Create first user
    UserRepository::create_guest(&test_db.pool, "user1", wallet)
        .await
        .expect("Failed to create first user");

    // Act - Try to create second user with same wallet
    let result = UserRepository::create_guest(&test_db.pool, "user2", wallet).await;

    // Assert - Should fail due to unique constraint
    assert!(result.is_err(), "Should not allow duplicate wallet addresses");

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_create_duplicate_email() {
    // Arrange
    let test_db = TestDb::new().await;
    let password_hash = hash_password("Test123!").unwrap();
    let email = "duplicate@example.com";

    // Create first user
    UserRepository::create_with_password(
        &test_db.pool,
        "user1",
        email,
        &password_hash,
    )
    .await
    .expect("Failed to create first user");

    // Act - Try to create second user with same email
    let result = UserRepository::create_with_password(
        &test_db.pool,
        "user2",
        email,
        &password_hash,
    )
    .await;

    // Assert - Should fail due to unique constraint
    assert!(result.is_err(), "Should not allow duplicate emails");

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// READ OPERATIONS
// ============================================================================

#[tokio::test]
async fn test_find_user_by_email() {
    // Arrange
    let test_db = TestDb::new().await;
    let password_hash = hash_password("Test123!").unwrap();

    UserRepository::create_with_password(
        &test_db.pool,
        "findme",
        "findme@example.com",
        &password_hash,
    )
    .await
    .unwrap();

    // Act
    let user = UserRepository::find_by_email(&test_db.pool, "findme@example.com")
        .await
        .expect("Query failed");

    // Assert
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.username, "findme");
    assert_eq!(user.email, Some("findme@example.com".to_string()));

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_find_user_by_email_not_found() {
    // Arrange
    let test_db = TestDb::new().await;

    // Act
    let user = UserRepository::find_by_email(&test_db.pool, "nonexistent@example.com")
        .await
        .expect("Query failed");

    // Assert
    assert!(user.is_none(), "Should return None for non-existent email");

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_find_user_by_wallet_address() {
    // Arrange
    let test_db = TestDb::new().await;
    let wallet = "GAFIND123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

    UserRepository::create_guest(&test_db.pool, "walletuser", wallet)
        .await
        .unwrap();

    // Act
    let user = UserRepository::find_by_wallet_address(&test_db.pool, wallet)
        .await
        .expect("Query failed");

    // Assert
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.wallet_address, wallet);
    assert_eq!(user.username, "walletuser");

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_find_user_by_id() {
    // Arrange
    let test_db = TestDb::new().await;
    let password_hash = hash_password("Test123!").unwrap();

    let created_user = UserRepository::create_with_password(
        &test_db.pool,
        "iduser",
        "iduser@example.com",
        &password_hash,
    )
    .await
    .unwrap();

    // Act
    let user = UserRepository::find_by_id(&test_db.pool, created_user.id)
        .await
        .expect("Query failed");

    // Assert
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.id, created_user.id);
    assert_eq!(user.username, "iduser");

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// UPDATE OPERATIONS
// ============================================================================

#[tokio::test]
async fn test_update_username() {
    // Arrange
    let test_db = TestDb::new().await;
    let wallet = "GAUPDATE1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";

    UserRepository::create_guest(&test_db.pool, "oldname", wallet)
        .await
        .unwrap();

    // Act
    let updated_user = UserRepository::update_username(&test_db.pool, wallet, "newname")
        .await
        .expect("Failed to update username");

    // Assert
    assert_eq!(updated_user.username, "newname");
    assert_eq!(updated_user.wallet_address, wallet);

    // Verify persistence
    let user = UserRepository::find_by_wallet_address(&test_db.pool, wallet)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.username, "newname");

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_update_last_login() {
    // Arrange
    let test_db = TestDb::new().await;
    let password_hash = hash_password("Test123!").unwrap();

    let user = UserRepository::create_with_password(
        &test_db.pool,
        "loginuser",
        "login@example.com",
        &password_hash,
    )
    .await
    .unwrap();

    assert!(user.last_login.is_none(), "last_login should initially be None");

    // Act
    UserRepository::update_last_login(&test_db.pool, user.id)
        .await
        .expect("Failed to update last_login");

    // Assert - Verify last_login was set
    let updated_user = UserRepository::find_by_id(&test_db.pool, user.id)
        .await
        .unwrap()
        .unwrap();

    assert!(updated_user.last_login.is_some(), "last_login should be set after update");

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_link_wallet() {
    // Arrange
    let test_db = TestDb::new().await;
    let password_hash = hash_password("Test123!").unwrap();

    let user = UserRepository::create_with_password(
        &test_db.pool,
        "linkuser",
        "link@example.com",
        &password_hash,
    )
    .await
    .unwrap();

    let new_wallet = "GALINK123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";

    // Act
    let updated_user = UserRepository::link_wallet(&test_db.pool, user.id, new_wallet)
        .await
        .expect("Failed to link wallet");

    // Assert
    assert_eq!(updated_user.wallet_address, new_wallet);
    assert_eq!(updated_user.id, user.id);

    // Verify we can now find user by wallet address
    let found = UserRepository::find_by_wallet_address(&test_db.pool, new_wallet)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found.id, user.id);

    // Cleanup
    test_db.cleanup().await;
}

#[tokio::test]
async fn test_link_wallet_already_taken() {
    // Arrange
    let test_db = TestDb::new().await;
    let wallet = "GATAKEN123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567";

    // Create user1 with this wallet
    UserRepository::create_guest(&test_db.pool, "user1", wallet)
        .await
        .unwrap();

    // Create user2 without wallet
    let password_hash = hash_password("Test123!").unwrap();
    let user2 = UserRepository::create_with_password(
        &test_db.pool,
        "user2",
        "user2@example.com",
        &password_hash,
    )
    .await
    .unwrap();

    // Act - Try to link user1's wallet to user2
    let result = UserRepository::link_wallet(&test_db.pool, user2.id, wallet).await;

    // Assert - Should fail because wallet is already taken
    assert!(result.is_err(), "Should not allow linking wallet that's already in use");

    // Cleanup
    test_db.cleanup().await;
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[tokio::test]
async fn test_concurrent_user_creation() {
    // Arrange
    let test_db = TestDb::new().await;
    let password_hash = hash_password("Test123!").unwrap();

    // Act - Create multiple users concurrently
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let pool = test_db.pool.clone();
            let hash = password_hash.clone();
            tokio::spawn(async move {
                UserRepository::create_with_password(
                    &pool,
                    &format!("concurrent_user_{}", i),
                    &format!("concurrent_{}@example.com", i),
                    &hash,
                )
                .await
            })
        })
        .collect();

    // Wait for all to complete
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    // Assert - All should succeed
    for result in results {
        assert!(result.is_ok(), "Concurrent user creation should succeed");
    }

    // Cleanup
    test_db.cleanup().await;
}
