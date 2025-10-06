/// Extractors for Axum handlers
/// Provides convenient extraction of authenticated user information
pub mod current_user;

// Re-export for convenience
pub use current_user::CurrentUser;
