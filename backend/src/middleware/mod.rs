/// Middleware module for authentication and authorization
pub mod auth;
pub mod require_role;

// Re-export for convenience
pub use auth::{CurrentUser, auth_middleware};
pub use require_role::{require_admin, require_chapter_lead};
