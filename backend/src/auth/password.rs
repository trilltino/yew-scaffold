use bcrypt::{hash, verify, BcryptError};
use tracing::{debug, info, warn, error};

/// Bcrypt cost factor (higher = more secure but slower)
/// 12 is a good balance between security and performance
const BCRYPT_COST: u32 = 12;

/// Hash a password using bcrypt
/// Returns Err if password doesn't meet strength requirements or hashing fails
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    debug!("[PASSWORD] Hashing password (length: {} chars)", password.len());

    // Validate password strength BEFORE hashing (save CPU if invalid)
    if let Err(e) = validate_password_strength(password) {
        warn!("[PASSWORD] ❌ Password validation failed");
        return Err(e);
    }

    debug!("[PASSWORD] Password meets strength requirements, proceeding with hash");
    debug!("[PASSWORD] Using bcrypt cost factor: {}", BCRYPT_COST);

    let start = std::time::Instant::now();
    let hash_result = hash(password, BCRYPT_COST);
    let duration = start.elapsed();

    match &hash_result {
        Ok(hash) => {
            info!("[PASSWORD] ✅ Password hashed successfully in {:?}", duration);
            debug!("[PASSWORD] Hash length: {} chars", hash.len());
            debug!("[PASSWORD] Hash preview: {}...", &hash[..20]);
        }
        Err(e) => {
            error!("[PASSWORD] ❌ Hashing failed: {}", e);
        }
    }

    hash_result
}

/// Verify a password against its bcrypt hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    debug!("[PASSWORD] Verifying password");
    debug!("[PASSWORD] Hash preview: {}...", &hash[..20.min(hash.len())]);

    let start = std::time::Instant::now();
    let verify_result = verify(password, hash);
    let duration = start.elapsed();

    match &verify_result {
        Ok(true) => {
            info!("[PASSWORD] ✅ Password verification SUCCESSFUL in {:?}", duration);
        }
        Ok(false) => {
            warn!("[PASSWORD] ❌ Password verification FAILED (wrong password) in {:?}", duration);
        }
        Err(e) => {
            error!("[PASSWORD] ❌ Password verification ERROR: {} in {:?}", e, duration);
        }
    }

    verify_result
}

/// Validate password strength requirements
/// Requirements:
/// - At least 8 characters
/// - Contains uppercase letter
/// - Contains lowercase letter
/// - Contains digit
/// - Optionally: special character (commented out for now)
fn validate_password_strength(password: &str) -> Result<(), BcryptError> {
    debug!("[PASSWORD] Validating password strength");

    // Check minimum length
    if password.len() < 8 {
        warn!("[PASSWORD] Password too short: {} chars (minimum 8)", password.len());
        return Err(BcryptError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Password must be at least 8 characters"
        )));
    }

    // Check maximum length (bcrypt has a 72 character limit)
    if password.len() > 72 {
        warn!("[PASSWORD] Password too long: {} chars (maximum 72 for bcrypt)", password.len());
        return Err(BcryptError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Password must be at most 72 characters"
        )));
    }

    // Check for lowercase
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    if !has_lowercase {
        warn!("[PASSWORD] Password missing lowercase letter");
        return Err(BcryptError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Password must contain at least one lowercase letter"
        )));
    }

    // Check for uppercase
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    if !has_uppercase {
        warn!("[PASSWORD] Password missing uppercase letter");
        return Err(BcryptError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Password must contain at least one uppercase letter"
        )));
    }

    // Check for digit
    let has_digit = password.chars().any(|c| c.is_numeric());
    if !has_digit {
        warn!("[PASSWORD] Password missing digit");
        return Err(BcryptError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Password must contain at least one number"
        )));
    }

    debug!("[PASSWORD] ✅ Password meets all strength requirements");
    Ok(())
}

/// Check if a password needs rehashing (e.g., cost factor changed)
/// Returns true if the current hash uses a different cost factor
pub fn needs_rehash(hash: &str, new_cost: u32) -> bool {
    // Extract cost from hash (bcrypt hash format: $2a$cost$...)
    if let Some(cost_str) = hash.get(4..6) {
        if let Ok(current_cost) = cost_str.parse::<u32>() {
            let needs = current_cost != new_cost;
            if needs {
                debug!("[PASSWORD] Hash needs rehashing: current cost {} != new cost {}", current_cost, new_cost);
            }
            return needs;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "SecurePass123";
        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("WrongPassword", &hash).unwrap());
        assert!(!verify_password("securepass123", &hash).unwrap());
    }

    #[test]
    fn test_weak_password_too_short() {
        let result = hash_password("Short1");
        assert!(result.is_err());
    }

    #[test]
    fn test_weak_password_no_uppercase() {
        let result = hash_password("nouppercase123");
        assert!(result.is_err());
    }

    #[test]
    fn test_weak_password_no_lowercase() {
        let result = hash_password("NOLOWERCASE123");
        assert!(result.is_err());
    }

    #[test]
    fn test_weak_password_no_digit() {
        let result = hash_password("NoDigitPassword");
        assert!(result.is_err());
    }

    #[test]
    fn test_strong_password_accepted() {
        let result = hash_password("StrongPass123!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_too_long() {
        let long_password = "A".repeat(73) + "bc123";
        let result = hash_password(&long_password);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_password_strength_valid() {
        assert!(validate_password_strength("Valid123Pass").is_ok());
        assert!(validate_password_strength("Another1Valid").is_ok());
        assert!(validate_password_strength("Test1234").is_ok());
    }

    #[test]
    fn test_validate_password_strength_invalid() {
        // These should all fail validation
        assert!(validate_password_strength("short").is_err());
        assert!(validate_password_strength("nouppercase123").is_err());
        assert!(validate_password_strength("NOLOWERCASE123").is_err());
        assert!(validate_password_strength("NoDigitsHere").is_err());
    }

    #[test]
    fn test_needs_rehash() {
        // Create a hash with default cost
        let password = "TestPass123";
        let hash = hash(password, 10).unwrap();

        // Should need rehash if we want cost 12 (our constant)
        assert!(needs_rehash(&hash, 12));

        // Should not need rehash if cost matches
        assert!(!needs_rehash(&hash, 10));
    }
}
