/// Authentication module
/// Handles JWT tokens, password hashing, and cookie management
pub mod jwt;
pub mod password;
pub mod cookies;

// Re-export commonly used types and functions
pub use jwt::{Claims, encode_jwt, decode_jwt, validate_token};
pub use password::{hash_password, verify_password};
pub use cookies::{create_auth_cookie, create_logout_cookie, get_token_from_cookies};
