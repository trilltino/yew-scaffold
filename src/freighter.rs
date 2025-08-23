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
    UserRejected,
}

impl fmt::Display for FreighterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreighterError::FreighterExtNotFound => {
                write!(f, "Freighter wallet extension not found. Install from https://freighter.app/")
            }
            FreighterError::NotAFunction(method) => {
                write!(f, "Property {method} exists but is not a callable function")
            }
            FreighterError::JsExecutionError(msg) => write!(f, "JavaScript error: {msg}"),
            FreighterError::NoWindow => write!(f, "Window object not available"),
            FreighterError::UserRejected => write!(f, "User rejected the connection request"),
        }
    }
}

impl std::error::Error for FreighterError {}

impl From<JsValue> for FreighterError {
    fn from(js_val: JsValue) -> Self {
        if let Some(err) = js_val.dyn_ref::<js_sys::Error>() {
            // Works on both older and newer js-sys:
            // If name()/message() return JsString, the Into<String> impl handles it.
            // If they already return String, this is a no-op move.
            let name: String = err.name().into();
            let msg: String  = err.message().into();

            let name_l = name.to_lowercase();
            let msg_l  = msg.to_lowercase();

            if name_l.contains("referenceerror")
                || (name_l.contains("typeerror") && (msg_l.contains("freighter") || msg_l.contains("undefined")))
            {
                return FreighterError::FreighterExtNotFound;
            }
            if msg_l.contains("user") && msg_l.contains("reject") {
                return FreighterError::UserRejected;
            }
            return FreighterError::JsExecutionError(msg);
        }

        if let Some(s) = js_val.as_string() {
            let l = s.to_lowercase();
            if l.contains("user") && l.contains("reject") {
                FreighterError::UserRejected
            } else if l.contains("freighter") || l.contains("not found") || l.contains("undefined") {
                FreighterError::FreighterExtNotFound
            } else {
                FreighterError::JsExecutionError(s)
            }
        } else {
            FreighterError::JsExecutionError("Unknown JavaScript error".to_string())
        }
    }
}

fn get_freighter_api() -> Result<JsValue, FreighterError> {
    let window = window().ok_or(FreighterError::NoWindow)?;
    let api = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi"))
        .map_err(FreighterError::from)?;
    if api.is_undefined() || api.is_null() {
        return Err(FreighterError::FreighterExtNotFound);
    }
    Ok(api)
}

pub fn is_freighter_available() -> bool {
    get_freighter_api().is_ok()
}

// IMPORTANT: extern now uses `catch` so JS exceptions become Result<_, JsValue>.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "freighterApi"], catch)]
    fn getPublicKey() -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "freighterApi"], catch)]
    fn requestAccess() -> Result<js_sys::Promise, JsValue>;
}

pub async fn get_public_key() -> Result<String, FreighterError> {
    if !is_freighter_available() {
        return Err(FreighterError::FreighterExtNotFound);
    }
    let promise = getPublicKey().map_err(FreighterError::from)?;
    let value = JsFuture::from(promise).await.map_err(FreighterError::from)?;
    value
        .as_string()
        .ok_or_else(|| FreighterError::JsExecutionError("getPublicKey returned non string".to_string()))
}

pub async fn connect_wallet() -> Result<String, FreighterError> {
    web_sys::console::log_1(&"Starting Freighter connection...".into());

    // 1) Verify API presence up front
    let api = get_freighter_api()?;
    web_sys::console::log_1(&"Requesting access...".into());

    // 2) Try freighterApi.requestAccess if present, otherwise skip gracefully
    let maybe_req = Reflect::get(&api, &JsValue::from_str("requestAccess"))
        .map_err(FreighterError::from)?;
    if maybe_req.is_function() {
        let req_promise = requestAccess().map_err(FreighterError::from)?;
        JsFuture::from(req_promise).await.map_err(FreighterError::from)?;
        web_sys::console::log_1(&"Access granted!".into());
    } else {
        web_sys::console::log_1(&"requestAccess not available, proceeding...".into());
    }

    // 3) Resolve a public key using any of the known methods
    web_sys::console::log_1(&"Getting public key...".into());
    let method_names = ["getPublicKey", "getUserInfo", "getAddress"];
    let mut callable = JsValue::UNDEFINED;

    for name in &method_names {
        let m = Reflect::get(&api, &JsValue::from_str(name)).map_err(FreighterError::from)?;
        if m.is_function() {
            web_sys::console::log_1(&format!("Found working method: {}", name).into());
            callable = m;
            break;
        }
    }
    if callable.is_undefined() {
        // Fall back to extern getPublicKey anyway, which will error clearly if missing
        let pk = get_public_key().await?;
        return Ok(pk);
    }

    let f = callable.dyn_into::<Function>().map_err(FreighterError::from)?;
    let p = f.call0(&api).map_err(FreighterError::from)?;
    let p = p.dyn_into::<Promise>().map_err(FreighterError::from)?;
    let v = JsFuture::from(p).await.map_err(FreighterError::from)?;

    if let Some(s) = v.as_string() {
        web_sys::console::log_1(&format!("Got key: {}", s).into());
        return Ok(s);
    }

    if let Ok(obj) = v.clone().dyn_into::<js_sys::Object>() {
        if let Ok(address) = Reflect::get(&obj, &JsValue::from_str("address")) {
            if let Some(s) = address.as_string() {
                web_sys::console::log_1(&format!("Got address: {}", s).into());
                return Ok(s);
            } else {
                return Err(FreighterError::JsExecutionError("Address property is not a string".to_string()));
            }
        }
        return Err(FreighterError::JsExecutionError("No address property in result".to_string()));
    }

    Err(FreighterError::JsExecutionError("Public key result not understood".to_string()))
}
