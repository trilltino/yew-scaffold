use js_sys::{Array, Object};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

// Import the generated TypeScript bindings
#[wasm_bindgen]
extern "C" {
    // Import the Client class from your generated bindings
    #[wasm_bindgen(js_namespace = ["hello_world"], js_name = "Client")]
    type HelloClient;

    #[wasm_bindgen(constructor, js_namespace = ["hello_world"])]
    fn new(options: &JsValue) -> HelloClient;

    // The hello method returns AssembledTransaction
    #[wasm_bindgen(method)]
    async fn hello(this: &HelloClient, params: &JsValue) -> JsValue;

    // AssembledTransaction type
    #[wasm_bindgen(js_name = "AssembledTransaction")]
    type AssembledTransaction;

    #[wasm_bindgen(method, getter)]
    fn result(this: &AssembledTransaction) -> JsValue;

    #[wasm_bindgen(method, js_name = "signAndSend")]
    async fn sign_and_send(this: &AssembledTransaction) -> JsValue;

    #[wasm_bindgen(method)]
    async fn simulate(this: &AssembledTransaction) -> JsValue;
}

// Network configuration matching your generated bindings
#[derive(Serialize, Deserialize)]
pub struct NetworkConfig {
    #[serde(rename = "networkPassphrase")]
    pub network_passphrase: String,
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "rpcUrl")]
    pub rpc_url: String,
    #[serde(rename = "allowHttp")]
    pub allow_http: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network_passphrase: "Test SDF Network ; September 2015".to_string(),
            contract_id: "CAT3RDKJVYMETDDGLFDLJ6TUZNXMTPV7ZPR7UZA3LVOSDWPONNASH4TM".to_string(),
            rpc_url: "https://soroban-testnet.stellar.org".to_string(),
            allow_http: false,
        }
    }
}

// High-level Rust wrapper
#[wasm_bindgen]
pub struct HelloContract {
    client: HelloClient,
}

#[wasm_bindgen]
impl HelloContract {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Option<JsValue>) -> Result<HelloContract, JsValue> {
        let config = config
            .unwrap_or_else(|| serde_wasm_bindgen::to_value(&NetworkConfig::default()).unwrap());

        let client = HelloClient::new(&config);
        Ok(HelloContract { client })
    }

    /// Call hello and get the result directly (simulated)
    #[wasm_bindgen]
    pub async fn call_hello(&self, to: &str) -> Result<Vec<String>, JsValue> {
        // Create parameters object
        let params = js_sys::Object::new();
        js_sys::Reflect::set(&params, &JsValue::from_str("to"), &JsValue::from_str(to))?;

        // Call the contract method
        let assembled_tx = self.client.hello(&params).await;
        let assembled_tx: AssembledTransaction = assembled_tx.dyn_into()?;

        // Get the result from simulation
        let result = assembled_tx.result();

        // Convert JavaScript Array to Rust Vec<String>
        if let Ok(array) = result.dyn_into::<Array>() {
            let mut strings = Vec::new();
            for i in 0..array.length() {
                if let Some(item) = array.get(i).as_string() {
                    strings.push(item);
                }
            }
            Ok(strings)
        } else {
            Err(JsValue::from_str(
                "Failed to convert result to string array",
            ))
        }
    }

    /// Call hello and return the assembled transaction for signing
    #[wasm_bindgen]
    pub async fn prepare_hello(&self, to: &str) -> Result<JsValue, JsValue> {
        let params = js_sys::Object::new();
        js_sys::Reflect::set(&params, &JsValue::from_str("to"), &JsValue::from_str(to))?;

        let assembled_tx = self.client.hello(&params).await;
        Ok(assembled_tx)
    }

    /// Simple hello that logs to console
    #[wasm_bindgen]
    pub async fn hello_console(&self, to: &str) -> Result<String, JsValue> {
        match self.call_hello(to).await {
            Ok(result) => {
                let result_str = format!("{:?}", result);
                console::log_1(&JsValue::from_str(&format!(
                    "Contract result: {}",
                    result_str
                )));
                Ok(result_str)
            }
            Err(e) => {
                console::log_1(&JsValue::from_str(&format!("Contract error: {:?}", e)));
                Err(e)
            }
        }
    }
}
