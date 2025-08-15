use js_sys::{Function, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

fn freighter_api() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let api = Reflect::get(&win, &JsValue::from_str("freighterApi"))?;
    if api.is_undefined() || api.is_null() {
        return Err(JsValue::from_str(
            "Freighter extension not installed or API not exposed",
        ));
    }
    Ok(api)
}

async fn call0(name: &str) -> Result<JsValue, JsValue> {
    let api = freighter_api()?;
    let f = Reflect::get(&api, &JsValue::from_str(name))?.dyn_into::<Function>()?;
    let p = f.call0(&api)?.dyn_into::<Promise>()?;
    JsFuture::from(p).await
}

pub async fn connect_and_get_address() -> Result<String, JsValue> {
    let is_conn = call0("isConnected").await?;
    let ok = Reflect::get(&is_conn, &JsValue::from_str("isConnected"))?
        .as_bool()
        .unwrap_or(false);
    if !ok {
        return Err(JsValue::from_str("Freighter not detected"));
    }

    let addr_res = call0("getAddress").await?;
    let address = Reflect::get(&addr_res, &JsValue::from_str("address"))?
        .as_string()
        .unwrap_or_default();
    if !address.is_empty() {
        return Ok(address);
    }

    let access_res = call0("requestAccess").await?;
    let addr = Reflect::get(&access_res, &JsValue::from_str("address"))?
        .as_string()
        .unwrap_or_default();
    if addr.is_empty() {
        return Err(JsValue::from_str("user rejected or no address"));
    }
    Ok(addr)
}
