/// Unit tests for frontend types
///
/// Tests pure Rust logic that doesn't require WASM runtime
/// These tests run with `cargo test` (not wasm-bindgen-test)

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_contract_function_name() {
        let func = ContractFunction::Hello { to: "World".to_string() };
        assert_eq!(func.name(), "hello");

        let func = ContractFunction::HelloYew { to: "Yew".to_string() };
        assert_eq!(func.name(), "hello_yew");

        let func = ContractFunction::Simple;
        assert_eq!(func.name(), "simple");
    }

    #[test]
    fn test_contract_function_signature() {
        let func = ContractFunction::Hello { to: "Test".to_string() };
        assert_eq!(func.signature(), "hello(to: string) -> vec<string>");

        let func = ContractFunction::TestFunc123 {
            param_1: "test".to_string(),
            param_2: 42,
        };
        assert_eq!(func.signature(), "test_func_123(param_1: string, param_2: u32) -> string");
    }

    #[test]
    fn test_contract_function_display_name() {
        let func = ContractFunction::HelloYew { to: "Yew".to_string() };
        assert_eq!(func.display_name(), "Hello Yew");

        let func = ContractFunction::EdgeCaseTestYew123End {
            edge_input: "test".to_string(),
        };
        assert_eq!(func.display_name(), "Edge Case Test");
    }

    #[test]
    fn test_contract_function_description() {
        let func = ContractFunction::Simple;
        assert_eq!(func.description(), "Simple function for baseline testing");

        let func = ContractFunction::X { y: "test".to_string() };
        assert_eq!(func.description(), "Function with single character name");
    }

    #[test]
    fn test_all_functions_returns_all_variants() {
        let all_funcs = ContractFunction::all_functions();

        // Should have all 6 function variants
        assert_eq!(all_funcs.len(), 6);

        // Verify each type is present
        let has_hello = all_funcs.iter().any(|f| matches!(f, ContractFunction::Hello { .. }));
        let has_hello_yew = all_funcs.iter().any(|f| matches!(f, ContractFunction::HelloYew { .. }));
        let has_simple = all_funcs.iter().any(|f| matches!(f, ContractFunction::Simple));
        let has_test_func = all_funcs.iter().any(|f| matches!(f, ContractFunction::TestFunc123 { .. }));
        let has_x = all_funcs.iter().any(|f| matches!(f, ContractFunction::X { .. }));
        let has_edge = all_funcs.iter().any(|f| matches!(f, ContractFunction::EdgeCaseTestYew123End { .. }));

        assert!(has_hello, "Should include Hello function");
        assert!(has_hello_yew, "Should include HelloYew function");
        assert!(has_simple, "Should include Simple function");
        assert!(has_test_func, "Should include TestFunc123 function");
        assert!(has_x, "Should include X function");
        assert!(has_edge, "Should include EdgeCaseTestYew123End function");
    }

    #[test]
    fn test_contract_function_equality() {
        let func1 = ContractFunction::Hello { to: "World".to_string() };
        let func2 = ContractFunction::Hello { to: "World".to_string() };
        let func3 = ContractFunction::Hello { to: "Different".to_string() };

        assert_eq!(func1, func2);
        assert_ne!(func1, func3);
    }

    #[test]
    fn test_contract_function_clone() {
        let func = ContractFunction::TestFunc123 {
            param_1: "test".to_string(),
            param_2: 123,
        };

        let cloned = func.clone();

        assert_eq!(func, cloned);
        assert_eq!(func.name(), cloned.name());
    }

    #[test]
    fn test_xdr_response_deserialization() {
        let json = r#"{
            "success": true,
            "xdr": "AAAA...base64...",
            "message": "XDR generated successfully"
        }"#;

        let response: Result<XdrResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.success, true);
        assert_eq!(response.xdr, "AAAA...base64...");
        assert_eq!(response.message, "XDR generated successfully");
    }

    #[test]
    fn test_submit_response_deserialization() {
        let json = r#"{
            "success": true,
            "result": "Transaction successful",
            "transaction_hash": "abc123",
            "message": "Transaction submitted"
        }"#;

        let response: Result<SubmitResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.success, true);
        assert_eq!(response.transaction_hash, "abc123");
    }
}
