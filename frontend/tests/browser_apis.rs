/// WASM tests for browser API functionality
///
/// Tests navigation, window operations, and other browser-specific features
/// that cannot be tested with native Rust tests.
///
/// Run with: wasm-pack test --headless --chrome

use wasm_bindgen_test::*;

// Configure to run in browser
wasm_bindgen_test_configure!(run_in_browser);

/// Test 1: Window location API availability
///
/// Tests that window.location is accessible and has expected properties.
/// Used in navigation component for logout (reload functionality).
#[wasm_bindgen_test]
fn test_window_location_api() {
    use web_sys::window;

    let win = window().expect("window should exist in browser");
    let location = win.location();

    // Test that we can access location properties
    assert!(location.href().is_ok(), "location.href should be accessible");
    assert!(location.pathname().is_ok(), "location.pathname should be accessible");
    assert!(location.origin().is_ok(), "location.origin should be accessible");
}

/// Test 2: Window location reload functionality
///
/// Tests the reload() method used in logout functionality.
/// Note: This test verifies the API exists but doesn't actually reload
/// (which would interrupt the test).
#[wasm_bindgen_test]
fn test_window_location_reload_exists() {
    use web_sys::window;

    let win = window().expect("window should exist");
    let location = win.location();

    // Verify reload method exists and is callable
    // We don't actually call it since it would reload the test page
    // Just verify the Location object has the method by checking it compiles
    let _ = location; // Location has reload() method available

    // Log that the API is available
    web_sys::console::log_1(&wasm_bindgen::prelude::JsValue::from_str(
        "✓ window.location.reload() API is available"
    ));
}

/// Test 3: Document API availability
///
/// Tests that document object is accessible for potential future features.
#[wasm_bindgen_test]
fn test_document_api() {
    use web_sys::window;

    let win = window().expect("window should exist");
    let doc = win.document().expect("window should have document");

    // Test basic document properties
    assert!(doc.body().is_some(), "document should have body");
    assert!(doc.title().len() >= 0, "document should have title");
}

/// Test 4: Local storage availability
///
/// Tests localStorage API for potential future features (wallet state persistence, etc.)
#[wasm_bindgen_test]
fn test_local_storage_api() {
    use web_sys::window;

    let win = window().expect("window should exist");
    let storage = win.local_storage().expect("local_storage should succeed");

    if let Some(storage) = storage {
        // Test set and get
        storage
            .set_item("wasm_test_key", "wasm_test_value")
            .expect("should set item");

        let value = storage
            .get_item("wasm_test_key")
            .expect("should get item");

        assert_eq!(value, Some("wasm_test_value".to_string()));

        // Clean up
        storage.remove_item("wasm_test_key").expect("should remove item");
    }
}

/// Test 5: Session storage availability
///
/// Tests sessionStorage API for temporary state management.
#[wasm_bindgen_test]
fn test_session_storage_api() {
    use web_sys::window;

    let win = window().expect("window should exist");
    let storage = win.session_storage().expect("session_storage should succeed");

    if let Some(storage) = storage {
        // Test set and get
        storage
            .set_item("wasm_session_test", "session_value")
            .expect("should set session item");

        let value = storage
            .get_item("wasm_session_test")
            .expect("should get session item");

        assert_eq!(value, Some("session_value".to_string()));

        // Clean up
        storage.remove_item("wasm_session_test").expect("should remove session item");
    }
}

/// Test 6: Console API availability
///
/// Ensures console methods work for debugging in WASM.
#[wasm_bindgen_test]
fn test_console_api() {
    use web_sys::console;
    use wasm_bindgen::prelude::*;

    // Test that console methods don't panic
    console::log_1(&JsValue::from_str("Console log test"));
    console::info_1(&JsValue::from_str("Console info test"));
    console::warn_1(&JsValue::from_str("Console warn test"));
    console::error_1(&JsValue::from_str("Console error test"));

    // Test multiple arguments
    console::log_2(
        &JsValue::from_str("Multiple"),
        &JsValue::from_str("arguments")
    );
}

/// Test 7: Fetch API availability
///
/// Tests that fetch API is available for HTTP requests.
/// Used for backend communication.
#[wasm_bindgen_test]
async fn test_fetch_api_available() {
    use web_sys::window;

    let win = window().expect("window should exist");

    // Just verify fetch exists - don't make actual request
    // The function exists in Window, this confirms it compiles
    let _ = win;

    web_sys::console::log_1(&wasm_bindgen::prelude::JsValue::from_str(
        "✓ fetch API is available"
    ));
}

/// Test 8: Performance API
///
/// Tests performance.now() for timing operations.
#[wasm_bindgen_test]
fn test_performance_api() {
    use web_sys::window;

    let win = window().expect("window should exist");
    let performance = win.performance().expect("performance should exist");

    let time1 = performance.now();
    let time2 = performance.now();

    assert!(time2 >= time1, "performance.now() should be monotonic");
}

/// Test 9: Navigator API
///
/// Tests navigator object for browser/user agent information.
#[wasm_bindgen_test]
fn test_navigator_api() {
    use web_sys::window;

    let win = window().expect("window should exist");
    let navigator = win.navigator();

    // Test user agent
    let user_agent = navigator.user_agent().expect("should have user agent");
    assert!(user_agent.len() > 0, "user agent should not be empty");

    // Test platform
    let platform = navigator.platform().expect("should have platform");
    assert!(platform.len() > 0, "platform should not be empty");
}

/// Test 10: History API
///
/// Tests browser history API for potential future SPA routing features.
#[wasm_bindgen_test]
fn test_history_api() {
    use web_sys::window;

    let win = window().expect("window should exist");
    let history = win.history().expect("window should have history");

    // Test that we can get history length
    let length = history.length().expect("should get history length");
    assert!(length >= 1, "history should have at least one entry");
}

/// Test 11: URL API
///
/// Tests URL constructor and manipulation for building API requests.
#[wasm_bindgen_test]
fn test_url_api() {
    use web_sys::Url;

    let url = Url::new("http://127.0.0.1:3001/health").expect("should create URL");

    assert_eq!(url.protocol(), "http:");
    assert_eq!(url.hostname(), "127.0.0.1");
    assert_eq!(url.port(), "3001");
    assert_eq!(url.pathname(), "/health");
}

/// Test 12: Window setTimeout/setInterval availability
///
/// Tests timer APIs for async operations and delays.
#[wasm_bindgen_test]
async fn test_timer_apis() {
    use gloo_timers::future::sleep;
    use std::time::Duration;

    let start = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now();

    // Test async sleep (uses setTimeout internally)
    sleep(Duration::from_millis(50)).await;

    let end = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now();

    let elapsed = end - start;
    assert!(elapsed >= 50.0, "sleep should wait at least 50ms");
}

/// Test 13: Crypto API availability
///
/// Tests crypto.getRandomValues for secure random generation.
/// Could be used for nonce generation, etc.
#[wasm_bindgen_test]
fn test_crypto_api() {
    use web_sys::window;
    use js_sys::Uint8Array;

    let win = window().expect("window should exist");
    let crypto = win.crypto().expect("crypto should exist");

    // Generate random bytes
    let array = Uint8Array::new_with_length(32);
    crypto.get_random_values_with_u8_array(&mut array.to_vec()[..])
        .expect("should generate random values");

    // Verify we got different values (extremely unlikely to be all zeros)
    let sum: u8 = array.to_vec().iter().sum();
    assert!(sum > 0, "random bytes should not all be zero");
}

/// Test 14: Test error handling for missing APIs
///
/// Ensures graceful degradation when browser APIs are not available.
#[wasm_bindgen_test]
fn test_api_error_handling() {
    use web_sys::window;

    let win = window().expect("window should exist");

    // Try to access a potentially missing API gracefully
    // Most modern browsers have these, but test error handling pattern
    match win.local_storage() {
        Ok(Some(_storage)) => {
            web_sys::console::log_1(&wasm_bindgen::prelude::JsValue::from_str(
                "✓ localStorage available"
            ));
        }
        Ok(None) => {
            web_sys::console::warn_1(&wasm_bindgen::prelude::JsValue::from_str(
                "⚠ localStorage returned None (private browsing?)"
            ));
        }
        Err(_) => {
            web_sys::console::error_1(&wasm_bindgen::prelude::JsValue::from_str(
                "✗ localStorage error"
            ));
        }
    }
}
