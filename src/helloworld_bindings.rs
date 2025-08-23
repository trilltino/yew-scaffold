use wasm_bindgen::prelude::*;
use serde::Serialize;

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
    const CONTRACT_ID: &str = "CAJHY2JSOGE7JMTBFZ4H3QL5GK2ZGPJBGII7W5GZ5LT4HGTAVP5IVDYE";
    const TO_ADDRESS: &str = "GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54";
    const NETWORK: &str = "Test SDF Network ; September 2015";
    const RPC_URL: &str = "https://soroban-testnet.stellar.org";

    let options = ClientOptions {
        contract_id: CONTRACT_ID.to_string(),
        network_passphrase: NETWORK.to_string(),
        rpc_url: RPC_URL.to_string(),
    };

    let client = StellarClient::new(&serde_wasm_bindgen::to_value(&options)?);
    let params = HelloParams { to: TO_ADDRESS.to_string() };
    
    let assembled_tx = wasm_bindgen_futures::JsFuture::from(client.hello(&serde_wasm_bindgen::to_value(&params)?)).await?;
    web_sys::console::log_2(&"AssembledTransaction:".into(), &assembled_tx);
    


    let result = js_sys::Reflect::get(&assembled_tx, &"result".into())?;
    web_sys::console::log_2(&" Contract result:".into(), &result);
    Ok(format!("Contract result: {}", 
        result.as_string().unwrap_or_else(|| format!("{:?}", result))
    ))
}

