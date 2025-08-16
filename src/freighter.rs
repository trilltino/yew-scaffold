use js_sys::{Function, Promise, Reflect};
use std::fmt;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[derive(Debug, Clone)]
pub enum FreighterError {
    FreighterExtNotFound,
    NotAFunction(String),
    JsExecutionError(String),
    NoWindow,
}

impl fmt::Display for FreighterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreighterError::FreighterExtNotFound => {
                write!(
                    f,
                    "Freighter wallet extension not found. Install from https://freighter.app/"
                )
            }

            FreighterError::NotAFunction(method) => {
                write!(
                    f,
                    "Property '{}' exists but is not a callable function",
                    method
                )
            }

            FreighterError::JsExecutionError(method) => {
                write!(f, "Javascript '{}' failed", method)
            }

            FreighterError::NoWindow => write!(f, "Window object not available"),
        }
    }
}

impl std::error::Error for FreighterError {}

impl From<JsValue> for FreighterError {
    fn from(js_val: JsValue) -> Self {
        if let Some(error_msg) = js_val.as_string() {
            if error_msg.contains("Freighter") && error_msg.contains("not found") {
                FreighterError::FreighterExtNotFound
            } else if error_msg.contains("Javascript not injected") {
                FreighterError::JsExecutionError(error_msg)
            } else if error_msg.contains("Not a valid Function") {
                FreighterError::NotAFunction(error_msg)
            } else {
                FreighterError::JsExecutionError("Uknown Javascript error".to_string())
            }
        } else {
            FreighterError::JsExecutionError("JS couldnt be converted".to_string())
        }
    }
}

impl From<FreighterError> for JsValue {
    fn from(error: FreighterError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

impl From<String> for FreighterError {
    fn from(error_msg: String) -> Self {
        FreighterError::JsExecutionError(error_msg)
    }
}

impl FreighterError {
    pub fn from_str(error_msg: &str) -> Self {
        FreighterError::JsExecutionError(error_msg.to_string())
    }
    pub fn is_extension_missing(&self) -> bool {
        matches!(
            self,
            FreighterError::FreighterExtNotFound | FreighterError::NoWindow
        )
    }

    pub fn user_message(&self) -> &str {
        match self {
            FreighterError::FreighterExtNotFound => {
                "Please install the Freighter wallet extension to continue"
            }
            FreighterError::JsExecutionError(_) => "Javascript failed to inject",
            FreighterError::NotAFunction(_) => "This is not a function",
            FreighterError::NoWindow => "Window object not available",
        }
    }
}

fn get_freighter_api() -> Result<JsValue, FreighterError> {
    let window = window().ok_or(FreighterError::NoWindow)?;

    let api = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi"))?;

    if api.is_undefined() || api.is_null() {
        return Err(FreighterError::FreighterExtNotFound);
    }

    Ok(api)
}

pub async fn is_connected() -> Result<bool, FreighterError> {
    let api = get_freighter_api()?;
    let method = Reflect::get(&api, &JsValue::from_str("isConnected"))?;

    if !method.is_function() {
        return Err(FreighterError::NotAFunction("isConnected".to_string()));
    }

    let function = method
        .dyn_into::<Function>()
        .map_err(|_| FreighterError::NotAFunction("isConnected".to_string()))?;

    let promise = function.call0(&api)?;
    let promise = promise.dyn_into::<Promise>().map_err(|_| {
        FreighterError::JsExecutionError("isConnected didn't return Promise".to_string())
    })?;

    let result = JsFuture::from(promise).await?;
    Ok(result.as_bool().unwrap_or(false))
}

pub async fn request_access() -> Result<JsValue, FreighterError> {
    let api = get_freighter_api()?;
    let method = Reflect::get(&api, &JsValue::from_str("requestAccess"))?;

    if !method.is_function() {
        return Err(FreighterError::NotAFunction("requestAccess".to_string()));
    }

    let function = method
        .dyn_into::<Function>()
        .map_err(|_| FreighterError::NotAFunction("requestAccess".to_string()))?;

    let promise = function.call0(&api)?;
    let promise = promise.dyn_into::<Promise>().map_err(|_| {
        FreighterError::JsExecutionError("requestAccess didn't return Promise".to_string())
    })?;

    let result = JsFuture::from(promise).await?;
    Ok(result)
}

pub async fn get_public_key() -> Result<String, FreighterError> {
    let api = get_freighter_api()?;
    let method = Reflect::get(&api, &JsValue::from_str("getPublicKey"))?;

    if !method.is_function() {
        return Err(FreighterError::NotAFunction("getPublicKey".to_string()));
    }

    let function = method
        .dyn_into::<Function>()
        .map_err(|_| FreighterError::NotAFunction("getPublicKey".to_string()))?;

    let promise = function.call0(&api)?;
    let promise = promise.dyn_into::<Promise>().map_err(|_| {
        FreighterError::JsExecutionError("getPublicKey didn't return Promise".to_string())
    })?;

    let result = JsFuture::from(promise).await?;
    result.as_string().ok_or_else(|| {
        FreighterError::JsExecutionError("getPublicKey didn't return string".to_string())
    })
}

// Main connect function that combines the above
pub async fn connect_wallet() -> Result<String, FreighterError> {
    // First check if already connected
    if !is_connected().await? {
        // Request access if not connected
        request_access().await?;
    }

    // Get the public key
    get_public_key().await
}

// Utility function to check if Freighter is available
pub fn is_freighter_available() -> bool {
    get_freighter_api().is_ok()
}
