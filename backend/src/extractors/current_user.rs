use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use tracing::{debug, warn};

/// Extractor for CurrentUser
/// Automatically extracts the authenticated user from request extensions
///
/// Usage in handlers:
/// ```
/// async fn my_handler(CurrentUser(user): CurrentUser) -> impl IntoResponse {
///     format!("Hello, {}! You are a {}", user.username, user.role)
/// }
/// ```
pub struct CurrentUser(pub crate::middleware::auth::CurrentUser);

// Axum 0.8 uses native async traits, no #[async_trait] macro needed
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("[EXTRACTOR] Extracting CurrentUser from request");

        let current_user = parts
            .extensions
            .get::<crate::middleware::auth::CurrentUser>()
            .cloned();

        match current_user {
            Some(user) => {
                debug!("[EXTRACTOR] ✅ CurrentUser extracted: {} (role: {})", user.username, user.role);
                Ok(CurrentUser(user))
            }
            None => {
                warn!("[EXTRACTOR] ❌ CurrentUser not found in request extensions");
                warn!("[EXTRACTOR] Did you apply auth_middleware to this route?");
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

// Implement Deref for convenient access to inner CurrentUser
impl std::ops::Deref for CurrentUser {
    type Target = crate::middleware::auth::CurrentUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_user_deref() {
        let inner_user = crate::middleware::auth::CurrentUser {
            user_id: 42,
            username: "testuser".to_string(),
            role: "visitor".to_string(),
        };

        let extractor = CurrentUser(inner_user);

        // Test deref
        assert_eq!(extractor.user_id, 42);
        assert_eq!(extractor.username, "testuser");
        assert_eq!(extractor.role, "visitor");
    }
}
