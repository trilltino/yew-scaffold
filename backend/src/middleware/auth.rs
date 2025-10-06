use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use tower_cookies::Cookies;
use crate::{
    auth::{validate_token, Claims},
    config::AppState,
};
use tracing::{debug, info, warn, error};

/// Current authenticated user information
/// Inserted into request extensions by auth_middleware
#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

impl CurrentUser {
    /// Create CurrentUser from JWT claims
    pub fn from_claims(claims: &Claims) -> Result<Self, String> {
        debug!("[MIDDLEWARE] Creating CurrentUser from claims");

        let user_id = claims.sub.parse::<i32>()
            .map_err(|e| {
                error!("[MIDDLEWARE]  Invalid user ID in token: {}", e);
                format!("Invalid user ID in token: {e}")
            })?;

        let current_user = Self {
            user_id,
            username: claims.username.clone(),
            role: claims.role.clone(),
        };

        debug!("[MIDDLEWARE] ✅ CurrentUser created:");
        debug!("  └─ User ID: {}", current_user.user_id);
        debug!("  └─ Username: {}", current_user.username);
        debug!("  └─ Role: {}", current_user.role);

        Ok(current_user)
    }

    /// Check if user is an admin
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    /// Check if user is a chapter lead (or admin)
    pub fn is_chapter_lead(&self) -> bool {
        self.role == "chapter_lead" || self.is_admin()
    }

    /// Check if user is a visitor
    pub fn is_visitor(&self) -> bool {
        self.role == "visitor"
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.role == role
    }
}

/// Authentication middleware
/// Validates JWT token from cookie and inserts CurrentUser into request extensions
///
/// This middleware:
/// 1. Extracts JWT token from cookie
/// 2. Validates the token
/// 3. Creates CurrentUser and inserts it into request extensions
/// 4. Returns 401 if authentication fails
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    debug!("[MIDDLEWARE] 🔐 Auth middleware checking request: {}", path);

    // Extract token from cookie
    let token = crate::auth::get_token_from_cookies(&cookies, &app_state.config);

    let token = match token {
        Some(t) if !t.is_empty() => t,
        _ => {
            warn!("[MIDDLEWARE] No authentication token found in cookies for: {}", path);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    debug!("[MIDDLEWARE] Token extracted from cookie, validating...");

    // Validate token
    let claims = match validate_token(&token, &app_state.config.jwt_secret) {
        Ok(claims) => {
            debug!("[MIDDLEWARE] Token validated successfully");
            claims
        }
        Err(e) => {
            warn!("[MIDDLEWARE] Token validation failed for {}: {}", path, e);
            error!("[MIDDLEWARE] Authentication error details: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Create CurrentUser and insert into request extensions
    let current_user = match CurrentUser::from_claims(&claims) {
        Ok(user) => {
            info!("[MIDDLEWARE] User authenticated: {} (role: {})", user.username, user.role);
            user
        }
        Err(e) => {
            error!("[MIDDLEWARE]  Failed to create CurrentUser: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Insert CurrentUser into request extensions so handlers can access it
    debug!("[MIDDLEWARE] Inserting CurrentUser into request extensions");
    request.extensions_mut().insert(current_user.clone());

    info!("[MIDDLEWARE] Request authorized for user: {} on {}", current_user.username, path);

    // Continue to the next middleware/handler
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::Claims;

    #[test]
    fn test_current_user_from_claims() {
        let claims = Claims::new(
            42,
            "testuser".to_string(),
            "chapter_lead".to_string(),
            24
        );

        let user = CurrentUser::from_claims(&claims).unwrap();

        assert_eq!(user.user_id, 42);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.role, "chapter_lead");
    }

    #[test]
    fn test_current_user_role_checks() {
        let admin = CurrentUser {
            user_id: 1,
            username: "admin".to_string(),
            role: "admin".to_string(),
        };

        assert!(admin.is_admin());
        assert!(admin.is_chapter_lead()); // Admin has chapter lead permissions
        assert!(!admin.is_visitor());

        let chapter_lead = CurrentUser {
            user_id: 2,
            username: "lead".to_string(),
            role: "chapter_lead".to_string(),
        };

        assert!(!chapter_lead.is_admin());
        assert!(chapter_lead.is_chapter_lead());
        assert!(!chapter_lead.is_visitor());

        let visitor = CurrentUser {
            user_id: 3,
            username: "visitor".to_string(),
            role: "visitor".to_string(),
        };

        assert!(!visitor.is_admin());
        assert!(!visitor.is_chapter_lead());
        assert!(visitor.is_visitor());
    }

    #[test]
    fn test_has_role() {
        let user = CurrentUser {
            user_id: 1,
            username: "test".to_string(),
            role: "chapter_lead".to_string(),
        };

        assert!(user.has_role("chapter_lead"));
        assert!(!user.has_role("admin"));
        assert!(!user.has_role("visitor"));
    }
}
