use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use crate::middleware::auth::CurrentUser;
use tracing::{debug, warn, info};




pub async fn require_admin(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    debug!("[MIDDLEWARE] Checking admin role for: {}", path);

    let (parts, body) = request.into_parts();
    let current_user = parts.extensions.get::<CurrentUser>();

    let current_user = match current_user {
        Some(user) => user,
        None => {
            warn!("[MIDDLEWARE] CurrentUser not found in request extensions!");
            warn!("[MIDDLEWARE] Did you forget to apply auth_middleware before require_admin?");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };



    debug!("[MIDDLEWARE] User: {} (role: {})", current_user.username, current_user.role);
    if !current_user.is_admin() {
        warn!("[MIDDLEWARE]  Access DENIED: User {} is not an admin (role: {})",
              current_user.username, current_user.role);
        warn!("[MIDDLEWARE] Attempted to access admin endpoint: {}", path);
        return Err(StatusCode::FORBIDDEN);
    }
    info!("[MIDDLEWARE]  Admin access GRANTED for user: {} on {}",
          current_user.username, path);
    let request = Request::from_parts(parts, body);
    Ok(next.run(request).await)
}




pub async fn require_chapter_lead(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    debug!("[MIDDLEWARE] 📋 Checking chapter lead role for: {}", path);

    let (parts, body) = request.into_parts();
    let current_user = parts.extensions.get::<CurrentUser>();
    let current_user = match current_user {
        Some(user) => user,
        None => {
            warn!("[MIDDLEWARE] CurrentUser not found in request extensions!");
            warn!("[MIDDLEWARE] Did you forget to apply auth_middleware before require_chapter_lead?");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };



    debug!("[MIDDLEWARE] User: {} (role: {})", current_user.username, current_user.role);
    if !current_user.is_chapter_lead() {
        warn!("[MIDDLEWARE] Access DENIED: User {} is not a chapter lead (role: {})",
              current_user.username, current_user.role);
        warn!("[MIDDLEWARE] Attempted to access chapter lead endpoint: {}", path);
        return Err(StatusCode::FORBIDDEN);
    }
    info!("[MIDDLEWARE]  Chapter lead access GRANTED for user: {} (role: {}) on {}",
          current_user.username, current_user.role, path);
    let request = Request::from_parts(parts, body);
    Ok(next.run(request).await)
}



pub async fn require_role(
    request: Request,
    next: Next,
    required_role: &str,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    debug!("[MIDDLEWARE]  Checking role '{}' for: {}", required_role, path);

    let (parts, body) = request.into_parts();
    let current_user = parts.extensions.get::<CurrentUser>();

    let current_user = match current_user {
        Some(user) => user,
        None => {
            warn!("[MIDDLEWARE] CurrentUser not found in request extensions!");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };


    debug!("[MIDDLEWARE] User: {} (role: {})", current_user.username, current_user.role);
    if !current_user.has_role(required_role) {
        warn!("[MIDDLEWARE] Access DENIED: User {} does not have role '{}' (has: {})",
              current_user.username, required_role, current_user.role);
        return Err(StatusCode::FORBIDDEN);
    }



    info!("[MIDDLEWARE] Role check PASSED for user: {} (required: {}) on {}",
          current_user.username, required_role, path);
    let request = Request::from_parts(parts, body);
    Ok(next.run(request).await)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(role: &str) -> CurrentUser {
        CurrentUser {
            user_id: 1,
            username: "testuser".to_string(),
            role: role.to_string(),
        }
    }

    #[test]
    fn test_admin_role_check() {
        let admin = create_test_user("admin");
        assert!(admin.is_admin());

        let visitor = create_test_user("visitor");
        assert!(!visitor.is_admin());
    }


    #[test]
    fn test_chapter_lead_role_check() {
        let admin = create_test_user("admin");
        assert!(admin.is_chapter_lead());

        let chapter_lead = create_test_user("chapter_lead");
        assert!(chapter_lead.is_chapter_lead());

        let visitor = create_test_user("visitor");
        assert!(!visitor.is_chapter_lead());
    }



    #[test]
    fn test_has_role_check() {
        let user = create_test_user("chapter_lead");
        assert!(user.has_role("chapter_lead"));
        assert!(!user.has_role("admin"));
        assert!(!user.has_role("visitor"));
    }
}
