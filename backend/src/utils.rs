use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};

pub fn create_cors_layer(allowed_origins: Vec<String>) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    if allowed_origins.is_empty() || allowed_origins.contains(&"*".to_string()) {
        cors = cors.allow_origin(Any);
    } else {
        for origin in allowed_origins {
            if let Ok(origin_header) = origin.parse::<axum::http::HeaderValue>() {
                cors = cors.allow_origin(origin_header);
            }
        }
    }

    cors
}

pub fn validate_stellar_address(address: &str) -> bool {
    address.starts_with('G') && address.len() == 56
}

pub fn validate_contract_id(contract_id: &str) -> bool {
    contract_id.starts_with('C') && contract_id.len() == 56
}

pub fn truncate_address(address: &str) -> String {
    if address.len() >= 12 {
        format!("{}...{}", &address[..6], &address[address.len()-6..])
    } else {
        address.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_stellar_address() {
        assert!(validate_stellar_address("GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54"));
        assert!(!validate_stellar_address("CDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54"));
        assert!(!validate_stellar_address("GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG5"));
    }

    #[test]
    fn test_validate_contract_id() {
        assert!(validate_contract_id("CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF"));
        assert!(!validate_contract_id("GCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF"));
        assert!(!validate_contract_id("CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUI"));
    }

    #[test]
    fn test_truncate_address() {
        let address = "GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54";
        assert_eq!(truncate_address(address), "GDAT5H...2CIG54");
        assert_eq!(truncate_address("short"), "short");
    }
}