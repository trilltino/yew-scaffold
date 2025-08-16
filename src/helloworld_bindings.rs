use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "HelloClient")]
    type HelloClient;

    #[wasm_bindgen(constructor)]
    fn new(options: &JsValue) -> HelloClient;

    #[wasm_bindgen(method)]
    async fn hello(this: &HelloClient, params: &JsValue) -> JsValue;
}

#[wasm_bindgen]
pub struct HelloContract {
    client: HelloClient,
}

#[wasm_bindgen]
impl HelloContract {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HelloContract {
        let config = JsValue::NULL;
        let client = HelloClient::new(&config);
        HelloContract { client }
    }

    #[wasm_bindgen]
    pub async fn call_hello(&self, to: &str) -> String {
        let params = js_sys::Object::new();
        js_sys::Reflect::set(&params, &JsValue::from_str("to"), &JsValue::from_str(to)).unwrap();

        let result = self.client.hello(&params).await;
        format!(
            "Got result: {:?}",
            result.as_string().unwrap_or("unknown".to_string())
        )
    }
}
