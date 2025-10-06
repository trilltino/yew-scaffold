# WASM Testing Guide for yew-scaffold

> **Official Documentation Source**: This guide is based on the official wasm-bindgen documentation at https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/

## Table of Contents

1. [Overview](#overview)
2. [Setup](#setup)
3. [Writing Basic Tests](#writing-basic-tests)
4. [Writing Asynchronous Tests](#writing-asynchronous-tests)
5. [Testing in Headless Browsers](#testing-in-headless-browsers)
6. [Continuous Integration](#continuous-integration)
7. [Project Integration](#project-integration)

---

## Overview

### Two Types of Rust Tests

**Native Tests** (`cargo test`):
- Run on your host machine (not in WASM environment)
- Fast and suitable for 95% of logic testing
- Cannot test browser APIs
- **This is what we use in yew-scaffold**: See [frontend/src/types_test.rs](../frontend/src/types_test.rs) and [frontend/src/state_test.rs](../frontend/src/state_test.rs)

**WASM Tests** (`wasm-bindgen-test`):
- Run in WASM environment (Node.js or browsers)
- Required only for testing browser-specific APIs (DOM, WebGL, WebAudio, etc.)
- Slower due to WASM compilation
- Uses `#[wasm_bindgen_test]` attribute instead of `#[test]`

### When to Use Each Type

| Test Type | Use For | Examples |
|-----------|---------|----------|
| **Native** | Business logic, state management, types, pure functions | `ContractFunction::name()`, `AppState::reduce()`, validators |
| **WASM** | Browser APIs, DOM interactions, web-sys calls | `document.query_selector()`, `window.fetch()`, canvas rendering |

**For yew-scaffold**: We use **native tests** for all our current tests because we're testing logic, not browser APIs.

---

## Setup

### 1. Add wasm-bindgen-test Dependency

Add to `frontend/Cargo.toml`:

```toml
[dev-dependencies]
wasm-bindgen-test = "0.3.0"
```

### 2. Install wasm-pack (Required for Browser Testing)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Or on Windows:
```powershell
cargo install wasm-pack
```

---

## Writing Basic Tests

### Native Test (Current Approach)

```rust
// frontend/src/types_test.rs
#[test]
fn test_contract_function_name() {
    let func = ContractFunction::Hello { to: "World".to_string() };
    assert_eq!(func.name(), "hello");
}
```

**Run with**: `cargo test`

### WASM Test (Only When Needed)

```rust
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
```

**Run with**:
```bash
# Node.js environment (default)
wasm-pack test --node

# Browser environment
wasm-pack test --headless --chrome
wasm-pack test --headless --firefox
```

---

## Writing Asynchronous Tests

WASM tests support async/await out of the box.

### Basic Async Test

```rust
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen_test]
async fn my_async_test() {
    // Create a JavaScript Promise
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    // Await it
    let result = JsFuture::from(promise).await.unwrap();

    assert_eq!(result, 42);
}
```

### Real-World Example: Testing Fetch API

```rust
use wasm_bindgen_test::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[wasm_bindgen_test]
async fn test_fetch_api() {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(
        "https://api.github.com/repos/rustwasm/wasm-bindgen",
        &opts,
    ).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    let resp: Response = resp_value.dyn_into().unwrap();
    assert!(resp.ok());
}
```

---

## Testing in Headless Browsers

### Environment Variables

By default, tests run on Node.js. To run in browsers:

```bash
# Browser (headless)
WASM_BINDGEN_USE_BROWSER=1 cargo test --target wasm32-unknown-unknown

# Dedicated Worker
WASM_BINDGEN_USE_DEDICATED_WORKER=1 cargo test --target wasm32-unknown-unknown

# Shared Worker
WASM_BINDGEN_USE_SHARED_WORKER=1 cargo test --target wasm32-unknown-unknown

# Service Worker
WASM_BINDGEN_USE_SERVICE_WORKER=1 cargo test --target wasm32-unknown-unknown

# Deno
WASM_BINDGEN_USE_DENO=1 cargo test --target wasm32-unknown-unknown

# Node.js ES module
WASM_BINDGEN_USE_NODE_EXPERIMENTAL=1 cargo test --target wasm32-unknown-unknown
```

### Force Configuration in Code

You can force tests to run in a specific environment:

```rust
use wasm_bindgen_test::wasm_bindgen_test_configure;

// Run in a browser
wasm_bindgen_test_configure!(run_in_browser);

// Or run in a dedicated worker
wasm_bindgen_test_configure!(run_in_dedicated_worker);

// Or run in a shared worker
wasm_bindgen_test_configure!(run_in_shared_worker);

// Or run in a service worker
wasm_bindgen_test_configure!(run_in_service_worker);

// Or run in Node.js but as an ES module
wasm_bindgen_test_configure!(run_in_node_experimental);
```

**Note**: This ignores environment variables.

### Using wasm-pack

```bash
# Chrome (headless)
wasm-pack test --headless --chrome

# Firefox (headless)
wasm-pack test --headless --firefox

# Safari (headless)
wasm-pack test --headless --safari

# Multiple browsers
wasm-pack test --headless --chrome --firefox --safari

# Without headless (opens browser with devtools)
wasm-pack test --chrome
```

### Configuring Browser Capabilities

Create `webdriver.json` in your crate root:

```json
{
  "moz:firefoxOptions": {
    "prefs": {
      "media.navigator.streams.fake": true,
      "media.navigator.permission.disabled": true
    },
    "args": []
  },
  "goog:chromeOptions": {
    "args": [
      "--use-fake-device-for-media-stream",
      "--use-fake-ui-for-media-stream"
    ]
  }
}
```

**Note**: The `headless` argument is always enabled automatically when using `--headless`.

### Debugging Browser Tests

Omit the `--headless` flag to open a real browser with devtools:

```bash
wasm-pack test --chrome
```

This allows you to:
- Set breakpoints in browser devtools
- Inspect DOM state
- View console logs
- Debug failing tests interactively

---

## Continuous Integration

### GitHub Actions

```yaml
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - run: cargo test
      - run: wasm-pack test --headless --chrome
      - run: wasm-pack test --headless --firefox
```

### Travis CI

```yaml
language: rust
rust: nightly

addons:
  firefox: latest
  chrome: stable

install:
  - curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

script:
  # Native tests
  - cargo test

  # WASM tests in browsers
  - wasm-pack test --firefox --headless
  - wasm-pack test --chrome --headless
```

### AppVeyor

```yaml
install:
  - ps: Install-Product node 10
  - appveyor-retry appveyor DownloadFile https://win.rustup.rs/ -FileName rustup-init.exe
  - rustup-init.exe -y --default-host x86_64-pc-windows-msvc --default-toolchain nightly
  - set PATH=%PATH%;C:\Users\appveyor\.cargo\bin
  - rustc -V
  - cargo -V
  - rustup target add wasm32-unknown-unknown
  - cargo install wasm-bindgen-cli

build: false

test_script:
  # Test in Chrome
  - set CHROMEDRIVER=C:\Tools\WebDriver\chromedriver.exe
  - cargo test --target wasm32-unknown-unknown
  - set CHROMEDRIVER=

  # Test in Firefox
  - set GECKODRIVER=C:\Tools\WebDriver\geckodriver.exe
  - cargo test --target wasm32-unknown-unknown
```

---

## Project Integration

### Current Setup (Native Tests Only)

Our frontend tests in `yew-scaffold` use **native tests** because we're testing pure Rust logic:

**frontend/src/types_test.rs** (9 tests):
```rust
#[test]
fn test_contract_function_name() {
    let func = ContractFunction::Hello { to: "World".to_string() };
    assert_eq!(func.name(), "hello");
}

#[test]
fn test_contract_function_all_functions() {
    let functions = ContractFunction::all_functions();
    assert_eq!(functions.len(), 6);
}
```

**frontend/src/state_test.rs** (11 tests):
```rust
#[test]
fn test_default_state() {
    let state = AppState::default();
    assert!(state.connected_wallet.is_none());
    assert_eq!(state.is_connecting, false);
}

#[test]
fn test_wallet_connected_message() {
    let state = Rc::new(AppState::default());
    let wallet = ConnectedWallet {
        address: "GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54".to_string(),
        wallet_type: "freighter".to_string(),
    };
    let new_state = state.reduce(AppMessage::WalletConnected(wallet));
    assert!(new_state.connected_wallet.is_some());
}
```

**Run with**:
```bash
cd frontend
cargo test
```

### When You'd Need WASM Tests

You would need WASM tests if you were testing:

1. **DOM Interactions**:
```rust
use wasm_bindgen_test::*;
use web_sys::window;

#[wasm_bindgen_test]
fn test_dom_manipulation() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let div = document.create_element("div").unwrap();
    div.set_inner_html("Hello");
    body.append_child(&div).unwrap();

    assert_eq!(div.inner_html(), "Hello");
}
```

2. **Freighter Wallet API** (browser extension):
```rust
#[wasm_bindgen_test]
async fn test_freighter_connection() {
    // This would require actual browser environment
    let result = check_freighter_available().await;
    assert!(result.is_ok());
}
```

3. **Browser Storage**:
```rust
#[wasm_bindgen_test]
fn test_local_storage() {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();

    storage.set_item("test_key", "test_value").unwrap();
    let value = storage.get_item("test_key").unwrap().unwrap();

    assert_eq!(value, "test_value");
}
```

### Adding WASM Tests to yew-scaffold

If you want to add WASM tests:

1. **Update frontend/Cargo.toml**:
```toml
[dev-dependencies]
wasm-bindgen-test = "0.3.0"
```

2. **Create frontend/tests/browser_tests.rs**:
```rust
use wasm_bindgen_test::*;

// Configure to run in browser
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_browser_api() {
    let window = web_sys::window().unwrap();
    assert!(window.document().is_some());
}
```

3. **Run with wasm-pack**:
```bash
cd frontend
wasm-pack test --headless --chrome
```

---

## Summary

### Current Approach (Recommended)

- ✅ **Native tests** for business logic (types, state, pure functions)
- ✅ Fast, simple, works with `cargo test`
- ✅ 20 tests already implemented
- ✅ No WASM compilation overhead

### When to Use WASM Tests

- ⚠️ Only when testing **browser-specific APIs**
- ⚠️ Requires wasm-pack installation
- ⚠️ Slower due to WASM compilation
- ⚠️ More complex CI setup

### Running Tests

```bash
# Native tests (current approach)
cd frontend && cargo test

# WASM tests (if you add them later)
cd frontend && wasm-pack test --headless --chrome
```

### Best Practice

Start with native tests (like we have), add WASM tests only when you need to test browser APIs.

---

## References

- **Official wasm-bindgen-test docs**: https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/
- **wasm-pack**: https://rustwasm.github.io/wasm-pack/
- **yew-scaffold testing guide**: [../TESTING.md](../TESTING.md)
- **Frontend unit tests**: [../frontend/src/types_test.rs](../frontend/src/types_test.rs), [../frontend/src/state_test.rs](../frontend/src/state_test.rs)
