use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Deserialize, Serialize};
use chrono::{Utc, TimeDelta};
use uuid::Uuid;
use tracing::{debug, warn, error, info};

/// JWT Claims structure containing user identity and metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // Subject (User ID)
    pub username: String,   // Username for convenience
    pub role: String,       // User role (visitor, chapter_lead, admin)
    pub exp: i64,          // Expiration time (Unix timestamp)
    pub iat: i64,          // Issued at (Unix timestamp)
    pub jti: String,       // JWT ID (unique identifier for this token, used for revocation)
}

impl Claims {
    /// Create new JWT claims for a user
    pub fn new(user_id: i32, username: String, role: String, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + TimeDelta::hours(expiration_hours);
        let jti = Uuid::new_v4().to_string();

        debug!("[JWT] Creating new claims:");
        debug!("  └─ User ID: {}", user_id);
        debug!("  └─ Username: {}", username);
        debug!("  └─ Role: {}", role);
        debug!("  └─ Issued at: {}", now);
        debug!("  └─ Expires at: {}", exp);
        debug!("  └─ JWT ID: {}", jti);

        Self {
            sub: user_id.to_string(),
            username,
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti,
        }
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        let expired = self.exp < now;

        if expired {
            let diff = now - self.exp;
            warn!("[JWT] Token expired {} seconds ago (user: {})", diff, self.username);
        }

        expired
    }

    /// Get time remaining until expiration (in seconds)
    pub fn time_until_expiry(&self) -> i64 {
        let now = Utc::now().timestamp();
        (self.exp - now).max(0)
    }
}

/// Encode a JWT token for a user
pub fn encode_jwt(
    user_id: i32,
    username: String,
    role: String,
    secret: &str,
    expiration_hours: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    info!("[JWT] Encoding JWT for user: {} (ID: {}, role: {})", username, user_id, role);

    let claims = Claims::new(user_id, username.clone(), role.clone(), expiration_hours);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    let token_preview = if token.len() > 40 {
        format!("{}...{}", &token[..20], &token[token.len()-20..])
    } else {
        token.clone()
    };

    info!("[JWT] ✅ Token encoded successfully for user: {}", username);
    debug!("[JWT] Token preview: {}", token_preview);
    debug!("[JWT] Token length: {} characters", token.len());
    debug!("[JWT] Valid for: {} hours", expiration_hours);

    Ok(token)
}

/// Decode a JWT token and extract claims
pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    debug!("[JWT] Decoding JWT token (length: {} chars)", token.len());

    let token_preview = if token.len() > 40 {
        format!("{}...{}", &token[..20], &token[token.len()-20..])
    } else {
        token.to_string()
    };
    debug!("[JWT] Token preview: {}", token_preview);

    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    let claims = token_data.claims;

    debug!("[JWT] ✅ Token decoded successfully");
    debug!("[JWT] User: {} (ID: {})", claims.username, claims.sub);
    debug!("[JWT] Role: {}", claims.role);
    debug!("[JWT] Issued at: {}", chrono::DateTime::from_timestamp(claims.iat, 0).map_or("Invalid".to_string(), |dt| dt.to_rfc3339()));
    debug!("[JWT] Expires at: {}", chrono::DateTime::from_timestamp(claims.exp, 0).map_or("Invalid".to_string(), |dt| dt.to_rfc3339()));
    debug!("[JWT] Time until expiry: {} seconds", claims.time_until_expiry());

    Ok(claims)
}

/// Validate a JWT token and return claims if valid
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, String> {
    debug!("[JWT] Validating token");

    decode_jwt(token, secret).map_err(|e| {
        let error_msg = match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                warn!("[JWT] ❌ Token validation failed: Expired signature");
                "Token has expired - please login again".to_string()
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                warn!("[JWT] ❌ Token validation failed: Invalid token format");
                "Invalid token format".to_string()
            }
            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                error!("[JWT] ❌ SECURITY: Invalid token signature detected!");
                "Invalid token signature - possible tampering detected".to_string()
            }
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                warn!("[JWT] ❌ Token validation failed: Invalid issuer");
                "Token from invalid issuer".to_string()
            }
            jsonwebtoken::errors::ErrorKind::InvalidAudience => {
                warn!("[JWT] ❌ Token validation failed: Invalid audience");
                "Token for invalid audience".to_string()
            }
            jsonwebtoken::errors::ErrorKind::ImmatureSignature => {
                warn!("[JWT] ❌ Token validation failed: Token not yet valid");
                "Token not yet valid".to_string()
            }
            _ => {
                error!("[JWT] ❌ Token validation failed: {}", e);
                format!("Token validation failed: {e}")
            }
        };

        error_msg
    }).and_then(|claims| {
        // Additional validation: check if token is expired
        if claims.is_expired() {
            warn!("[JWT] ❌ Token is expired");
            return Err("Token has expired - please login again".to_string());
        }

        info!("[JWT] ✅ Token validation successful for user: {}", claims.username);
        Ok(claims)
    })
}

/// Extract JWT ID (jti) from token for session tracking
pub fn extract_jti(token: &str, secret: &str) -> Result<String, String> {
    validate_token(token, secret).map(|claims| claims.jti)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "test-secret-key-at-least-32-characters-long-for-security";

    #[test]
    fn test_encode_decode_jwt() {
        let token = encode_jwt(
            1,
            "testuser".to_string(),
            "visitor".to_string(),
            TEST_SECRET,
            24
        ).unwrap();

        let claims = decode_jwt(&token, TEST_SECRET).unwrap();

        assert_eq!(claims.sub, "1");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, "visitor");
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_validate_token_success() {
        let token = encode_jwt(
            42,
            "alice".to_string(),
            "chapter_lead".to_string(),
            TEST_SECRET,
            1
        ).unwrap();

        let result = validate_token(&token, TEST_SECRET);
        assert!(result.is_ok());

        let claims = result.unwrap();
        assert_eq!(claims.username, "alice");
        assert_eq!(claims.role, "chapter_lead");
    }

    #[test]
    fn test_validate_token_wrong_secret() {
        let token = encode_jwt(
            1,
            "testuser".to_string(),
            "visitor".to_string(),
            TEST_SECRET,
            24
        ).unwrap();

        let result = validate_token(&token, "wrong-secret-key-that-is-also-32-chars");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("signature"));
    }

    #[test]
    fn test_claims_time_until_expiry() {
        let claims = Claims::new(1, "test".to_string(), "visitor".to_string(), 2);
        let time = claims.time_until_expiry();

        // Should be approximately 2 hours in seconds (with small tolerance)
        assert!(time > 7100 && time <= 7200);
    }

    #[test]
    fn test_extract_jti() {
        let token = encode_jwt(
            1,
            "testuser".to_string(),
            "visitor".to_string(),
            TEST_SECRET,
            24
        ).unwrap();

        let jti = extract_jti(&token, TEST_SECRET).unwrap();
        assert!(!jti.is_empty());

        // JTI should be a valid UUID v4
        assert!(Uuid::parse_str(&jti).is_ok());
    }
}
