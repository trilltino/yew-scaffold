use tower_cookies::Cookie;
use time::{Duration, OffsetDateTime};
use crate::config::AppConfig;
use tracing::{debug, info, warn};

/// Create an authentication cookie with JWT token
pub fn create_auth_cookie(token: String, config: &AppConfig) -> Cookie<'static> {
    debug!("[COOKIE] Creating authentication cookie");

    let expiration = OffsetDateTime::now_utc() + Duration::hours(config.jwt_expiration_hours);

    debug!("[COOKIE] Cookie config:");
    debug!("  └─ Name: {}", config.cookie_name);
    debug!("  └─ Domain: {}", config.cookie_domain);
    debug!("  └─ Path: /");
    debug!("  └─ Secure: {}", config.cookie_secure);
    debug!("  └─ HttpOnly: {}", config.cookie_http_only);
    debug!("  └─ SameSite: {}", config.cookie_same_site);
    debug!("  └─ Expires: {}", expiration);

    // Token preview for logging (never log full token!)
    let token_preview = if token.len() > 40 {
        format!("{}...{}", &token[..15], &token[token.len()-15..])
    } else {
        "***".to_string()
    };
    debug!("[COOKIE] Token preview: {}", token_preview);

    let same_site = match config.cookie_same_site.as_str() {
        "Strict" => tower_cookies::cookie::SameSite::Strict,
        "Lax" => tower_cookies::cookie::SameSite::Lax,
        "None" => tower_cookies::cookie::SameSite::None,
        _ => {
            warn!("[COOKIE] Invalid SameSite value '{}', defaulting to Strict", config.cookie_same_site);
            tower_cookies::cookie::SameSite::Strict
        }
    };

    let cookie = Cookie::build((config.cookie_name.clone(), token))
        .path("/")
        .domain(config.cookie_domain.clone())
        .secure(config.cookie_secure)
        .http_only(config.cookie_http_only)
        .same_site(same_site)
        .expires(expiration)
        .build();

    info!("[COOKIE]  Authentication cookie created successfully");

    if !config.cookie_secure && config.cookie_domain != "localhost" {
        warn!("[COOKIE]  SECURITY WARNING: Cookie is not secure on non-localhost domain!");
    }

    cookie
}

/// Create a logout cookie (expires immediately to clear auth)
pub fn create_logout_cookie(config: &AppConfig) -> Cookie<'static> {
    debug!("[COOKIE] Creating logout cookie (clearing authentication)");

    // Set expiration to the past to delete the cookie
    let past = OffsetDateTime::now_utc() - Duration::days(1);

    let cookie = Cookie::build((config.cookie_name.clone(), ""))
        .path("/")
        .domain(config.cookie_domain.clone())
        .expires(past)
        .build();

    info!("[COOKIE] Logout cookie created (will clear existing auth cookie)");

    cookie
}

/// Extract JWT token from cookies
/// Returns None if cookie is not found
pub fn get_token_from_cookies(cookies: &tower_cookies::Cookies, config: &AppConfig) -> Option<String> {
    debug!("[COOKIE] Extracting token from cookies");
    debug!("[COOKIE] Looking for cookie named: {}", config.cookie_name);

    let token_option = cookies
        .get(&config.cookie_name)
        .map(|cookie| {
            let token = cookie.value().to_string();

            let token_preview = if token.len() > 40 {
                format!("{}...{}", &token[..15], &token[token.len()-15..])
            } else if !token.is_empty() {
                "***".to_string()
            } else {
                "(empty)".to_string()
            };

            debug!("[COOKIE] Token found in cookie");
            debug!("[COOKIE] Token preview: {}", token_preview);
            debug!("[COOKIE] Token length: {} characters", token.len());

            token
        });

    if token_option.is_none() {
        debug!("[COOKIE]  No authentication cookie found");
    }

    token_option
}

/// Extract all cookies for debugging purposes
/// WARNING: Only use in development, never log in production!
#[cfg(debug_assertions)]
pub fn debug_all_cookies(cookies: &tower_cookies::Cookies) {
    debug!("[COOKIE] [DEBUG] Listing all cookies:");

    let cookie_list = cookies.list();
    let cookie_count = cookie_list.len();
    debug!("[COOKIE] [DEBUG] Total cookies: {}", cookie_count);

    for (index, cookie) in cookie_list.iter().enumerate() {
        debug!("[COOKIE] [DEBUG] Cookie #{}: {}", index + 1, cookie.name());
        debug!("[COOKIE] [DEBUG]   └─ Path: {:?}", cookie.path());
        debug!("[COOKIE] [DEBUG]   └─ Domain: {:?}", cookie.domain());
        debug!("[COOKIE] [DEBUG]   └─ Secure: {}", cookie.secure().unwrap_or(false));
        debug!("[COOKIE] [DEBUG]   └─ HttpOnly: {}", cookie.http_only().unwrap_or(false));
        debug!("[COOKIE] [DEBUG]   └─ SameSite: {:?}", cookie.same_site());

        // Never log actual cookie values!
        if cookie.value().is_empty() {
            debug!("[COOKIE] [DEBUG]   └─ Value: (empty)");
        } else {
            debug!("[COOKIE] [DEBUG]   └─ Value: *** (hidden)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> AppConfig {
        AppConfig {
            port: 3001,
            allowed_origins: vec!["http://localhost:8080".to_string()],
            contract_id: "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF".to_string(),
            network_passphrase: "Test SDF Network ; September 2015".to_string(),
            rpc_url: "https://soroban-testnet.stellar.org".to_string(),
            jwt_secret: "test-secret-key-at-least-32-characters-long".to_string(),
            jwt_expiration_hours: 24,
            cookie_name: "test_auth".to_string(),
            cookie_domain: "localhost".to_string(),
            cookie_secure: false,
            cookie_http_only: true,
            cookie_same_site: "Strict".to_string(),
        }
    }

    #[test]
    fn test_create_auth_cookie() {
        let config = test_config();
        let token = "test.jwt.token".to_string();

        let cookie = create_auth_cookie(token.clone(), &config);

        assert_eq!(cookie.name(), "test_auth");
        assert_eq!(cookie.value(), "test.jwt.token");
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.domain(), Some("localhost"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.secure(), Some(false));
    }

    #[test]
    fn test_create_logout_cookie() {
        let config = test_config();

        let cookie = create_logout_cookie(&config);

        assert_eq!(cookie.name(), "test_auth");
        assert_eq!(cookie.value(), "");
        assert!(cookie.expires().is_some());

        // Check that expiration is in the past
        if let Some(expires) = cookie.expires() {
            let now = OffsetDateTime::now_utc();
            match expires {
                tower_cookies::cookie::Expiration::DateTime(dt) => {
                    assert!(dt < now);
                }
                _ => panic!("Expected DateTime expiration"),
            }
        }
    }

    #[test]
    fn test_same_site_parsing() {
        let mut config = test_config();

        // Test Strict
        config.cookie_same_site = "Strict".to_string();
        let cookie = create_auth_cookie("token".to_string(), &config);
        assert_eq!(cookie.same_site(), Some(tower_cookies::cookie::SameSite::Strict));

        // Test Lax
        config.cookie_same_site = "Lax".to_string();
        let cookie = create_auth_cookie("token".to_string(), &config);
        assert_eq!(cookie.same_site(), Some(tower_cookies::cookie::SameSite::Lax));

        // Test None
        config.cookie_same_site = "None".to_string();
        let cookie = create_auth_cookie("token".to_string(), &config);
        assert_eq!(cookie.same_site(), Some(tower_cookies::cookie::SameSite::None));
    }
}
