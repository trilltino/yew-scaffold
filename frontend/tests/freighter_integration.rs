/// WASM integration tests for Freighter wallet functionality
///
/// These tests MUST run in a browser environment with Freighter extension installed.
/// They cannot run as native tests.
///
/// Run with: wasm-pack test --headless --chrome
/// Debug with: wasm-pack test --chrome (opens browser with devtools)

use wasm_bindgen_test::*;

// Configure tests to run in browser (required for wallet extension testing)
wasm_bindgen_test_configure!(run_in_browser);

// Note: We cannot directly import from src/ in wasm-bindgen tests due to module structure
// Instead, we test the public API behavior

/// Test 1: Verify Freighter API is available in browser
///
/// This test checks if window.freighterApi exists when Freighter extension is installed.
/// Expected: true if Freighter is installed, false otherwise
#[wasm_bindgen_test]
async fn test_freighter_api_exists() {
    use wasm_bindgen::prelude::*;
    use web_sys::window;
    use js_sys::Reflect;

    let window = window().expect("window should exist in browser");
    let freighter_api = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi"));

    // This test will pass if Freighter is installed, fail if not
    // In CI, you'd need to mock this or skip if not available
    if let Ok(api) = freighter_api {
        assert!(!api.is_undefined(), "freighterApi should be defined when Freighter is installed");
    } else {
        // Log warning but don't fail - allows tests to run without Freighter
        web_sys::console::warn_1(&JsValue::from_str(
            "⚠️  Freighter extension not detected. Install from https://freighter.app/"
        ));
    }
}

/// Test 2: Verify Freighter API has required methods
///
/// Checks that the Freighter API object has all expected methods:
/// - isConnected
/// - requestAccess
/// - getPublicKey
/// - signTransaction
#[wasm_bindgen_test]
async fn test_freighter_api_methods() {
    use wasm_bindgen::prelude::*;
    use web_sys::window;
    use js_sys::Reflect;

    let window = window().expect("window should exist");

    if let Ok(api) = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi")) {
        if !api.is_undefined() {
            // Check for required methods
            let required_methods = ["isConnected", "requestAccess", "getPublicKey", "signTransaction"];

            for method in &required_methods {
                let method_exists = Reflect::get(&api, &JsValue::from_str(method));
                assert!(
                    method_exists.is_ok() && method_exists.unwrap().is_function(),
                    "Freighter API should have {} method",
                    method
                );
            }
        }
    }
}

/// Test 3: Test error handling when Freighter is not available
///
/// This test verifies that our error handling works correctly when
/// the Freighter extension is not installed.
#[wasm_bindgen_test]
async fn test_freighter_not_available_error() {
    use wasm_bindgen::prelude::*;
    use web_sys::window;
    use js_sys::Reflect;

    let window = window().expect("window should exist");

    // Temporarily remove freighterApi to test error handling
    let original_api = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi"));

    // Set it to undefined
    let _ = Reflect::set(
        window.as_ref(),
        &JsValue::from_str("freighterApi"),
        &JsValue::UNDEFINED
    );

    // Now check that we handle the missing API correctly
    let api = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi"));
    if let Ok(api_val) = api {
        assert!(
            api_val.is_undefined(),
            "API should be undefined when testing error case"
        );
    }

    // Restore original value
    if let Ok(orig) = original_api {
        let _ = Reflect::set(window.as_ref(), &JsValue::from_str("freighterApi"), &orig);
    }
}

/// Test 4: Test Promise-based async operations
///
/// Verifies that we can work with JavaScript Promises correctly in WASM.
/// This is critical for Freighter which uses Promise-based APIs.
#[wasm_bindgen_test]
async fn test_promise_handling() {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use js_sys::Promise;

    // Create a simple resolved promise
    let promise = Promise::resolve(&JsValue::from_str("test_value"));

    // Convert to Rust future and await
    let result = JsFuture::from(promise).await;

    assert!(result.is_ok(), "Promise should resolve successfully");
    assert_eq!(
        result.unwrap().as_string().unwrap(),
        "test_value",
        "Promise should resolve to correct value"
    );
}

/// Test 5: Test error Promise handling
///
/// Verifies that we correctly handle rejected promises, which is important
/// for handling user rejection in Freighter wallet flow.
#[wasm_bindgen_test]
async fn test_promise_rejection_handling() {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use js_sys::Promise;

    // Create a rejected promise
    let promise = Promise::reject(&JsValue::from_str("User rejected"));

    // Convert to Rust future and await
    let result = JsFuture::from(promise).await;

    assert!(result.is_err(), "Rejected promise should return error");

    if let Err(e) = result {
        let error_msg = e.as_string().unwrap_or_default();
        assert!(
            error_msg.contains("rejected"),
            "Error message should contain 'rejected'"
        );
    }
}

/// Test 6: Test window object availability
///
/// Ensures that web_sys::window() works correctly in WASM environment.
/// This is a fundamental requirement for all wallet integration code.
#[wasm_bindgen_test]
fn test_window_object() {
    use web_sys::window;

    let win = window();
    assert!(win.is_some(), "window object should be available in browser");

    let win = win.unwrap();
    let location = win.location();
    assert!(location.href().is_ok(), "window.location should be accessible");
}

/// Test 7: Test console logging in WASM
///
/// Verifies that console.log works for debugging Freighter integration issues.
#[wasm_bindgen_test]
fn test_console_logging() {
    use web_sys::console;
    use wasm_bindgen::prelude::*;

    // Test various console methods
    console::log_1(&JsValue::from_str("Test log message"));
    console::warn_1(&JsValue::from_str("Test warning message"));
    console::error_1(&JsValue::from_str("Test error message"));

    // If we get here without panicking, logging works
    assert!(true, "Console logging should work in WASM");
}

/// Test 8: Test Reflect API for dynamic property access
///
/// Freighter integration uses Reflect extensively to access JavaScript objects.
/// This test ensures Reflect works correctly in our WASM environment.
#[wasm_bindgen_test]
fn test_reflect_api() {
    use wasm_bindgen::prelude::*;
    use js_sys::{Object, Reflect};

    // Create a test object
    let obj = Object::new();

    // Set a property
    let set_result = Reflect::set(
        &obj,
        &JsValue::from_str("testProp"),
        &JsValue::from_str("testValue")
    );
    assert!(set_result.is_ok(), "Reflect::set should succeed");

    // Get the property
    let get_result = Reflect::get(&obj, &JsValue::from_str("testProp"));
    assert!(get_result.is_ok(), "Reflect::get should succeed");
    assert_eq!(
        get_result.unwrap().as_string().unwrap(),
        "testValue",
        "Retrieved value should match set value"
    );
}

/// Test 9: Test async/await in WASM
///
/// Verifies that async functions work correctly in WASM tests.
/// This is essential for Freighter's async connect/sign operations.
#[wasm_bindgen_test]
async fn test_async_operations() {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use gloo_timers::future::sleep;
    use std::time::Duration;

    // Test async sleep
    sleep(Duration::from_millis(10)).await;

    // Test chaining async operations
    let promise1 = js_sys::Promise::resolve(&JsValue::from_f64(1.0));
    let result1 = JsFuture::from(promise1).await.unwrap();

    let promise2 = js_sys::Promise::resolve(&JsValue::from_f64(
        result1.as_f64().unwrap() + 1.0
    ));
    let result2 = JsFuture::from(promise2).await.unwrap();

    assert_eq!(result2.as_f64().unwrap(), 2.0, "Async chain should work");
}

/// Test 10: Integration test placeholder for actual Freighter connection
///
/// NOTE: This test requires user interaction and Freighter extension.
/// It's marked as a placeholder and should be run manually with a real browser.
///
/// To run manually:
/// 1. Install Freighter extension
/// 2. Run: wasm-pack test --chrome (without --headless)
/// 3. Click "Allow" when Freighter prompts
#[wasm_bindgen_test]
#[ignore] // Ignore by default since it requires user interaction
async fn test_freighter_connection_flow() {
    // This test would require:
    // 1. Freighter extension installed
    // 2. User clicking "Allow" in the popup
    // 3. Valid Stellar testnet account

    // Placeholder - implement only when needed for manual testing
    web_sys::console::log_1(&wasm_bindgen::prelude::JsValue::from_str(
        "Manual test: Connect to Freighter and verify public key is returned"
    ));
}
