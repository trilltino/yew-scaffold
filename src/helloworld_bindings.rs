use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde::{Deserialize, Serialize};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "StellarClient", js_namespace = window)]
    type StellarClient;
    
    #[wasm_bindgen(constructor)]
    pub fn new(options: &JsValue) -> StellarClient;
    
    #[wasm_bindgen(method)]
    pub fn hello(this: &StellarClient, params: &JsValue) -> js_sys::Promise;
}

#[derive(Serialize)]
pub struct ClientOptions {
    #[serde(rename = "contractId")]
    contract_id: String,
    #[serde(rename = "networkPassphrase")]
    network_passphrase: String,
    #[serde(rename = "rpcUrl")]
    rpc_url: String,
}

#[derive(Serialize)]
struct HelloParams {
    to: String,
}

pub async fn call_hello_contract() -> Result<String, JsValue> {
    let options = ClientOptions {
        contract_id: "CAJHY2JSOGE7JMTBFZ4H3QL5GK2ZGPJBGII7W5GZ5LT4HGTAVP5IVDYE".to_string(),
        network_passphrase: "Test SDF Network ; September 2015".to_string(),
        rpc_url: "https://soroban-testnet.stellar.org".to_string(),
    };
    
    let options_js = serde_wasm_bindgen::to_value(&options)?;
    let client = StellarClient::new(&options_js);
    

    let params = HelloParams {
        to: "GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54".to_string(),
    };
    let params_js = serde_wasm_bindgen::to_value(&params)?;
    
    let promise = client.hello(&params_js);
    let assembled_tx = wasm_bindgen_futures::JsFuture::from(promise).await?;
    
    web_sys::console::log_1(&JsValue::from_str("📦 Full AssembledTransaction:"));
    web_sys::console::log_1(&assembled_tx);
    

    let result_field = js_sys::Reflect::get(&assembled_tx, &JsValue::from_str("result"))?;
    

    web_sys::console::log_1(&JsValue::from_str("📦 Raw contract result:"));
    web_sys::console::log_1(&result_field);

    let result_field_for_array = result_field.clone();
    if let Ok(array) = result_field_for_array.dyn_into::<js_sys::Array>() {
        let mut response_parts = Vec::new();
        for i in 0..array.length() {
            if let Some(item) = array.get(i).as_string() {
                response_parts.push(item);
            }
        } 
        if response_parts.len() >= 2 {
            Ok(format!("Contract says: '{}' to '{}'", response_parts[0], response_parts[1]))
        } else {
            Ok(format!("Contract returned array: {:?}", response_parts))
        }
    } else {
        Ok(format!("Contract result (not array): {:?}", result_field))
    }
}