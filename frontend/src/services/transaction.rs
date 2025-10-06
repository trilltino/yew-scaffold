use gloo_net::http::Request;
use serde::Serialize;
use crate::types::{ContractFunction, XdrResponse, SubmitResponse};
use crate::wallet::{ConnectedWallet, sign_transaction};

/// Generate XDR via backend service
pub async fn generate_xdr(source_account: &str, wallet_type: &str, function: &ContractFunction) -> Result<String, String> {
    let function_name = function.name();

    let url = format!("http://127.0.0.1:3001/generate-xdr?source_account={}&wallet_type={}&function_name={}",
                     source_account, wallet_type, function_name);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Backend error: HTTP {}", response.status()));
    }

    let xdr_response: XdrResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    if xdr_response.success {
        Ok(xdr_response.xdr)
    } else {
        Err(xdr_response.message)
    }
}

/// Submit signed transaction to backend and get contract result
pub async fn submit_signed_transaction(signed_xdr: &str, wallet_type: &str, function: &ContractFunction) -> Result<String, String> {
    #[derive(Serialize)]
    struct SubmitRequest {
        signed_xdr: String,
        wallet_type: Option<String>,
        function_name: Option<String>,
    }

    let payload = SubmitRequest {
        signed_xdr: signed_xdr.to_string(),
        wallet_type: Some(wallet_type.to_string()),
        function_name: Some(function.name().to_string()),
    };

    let response = Request::post("http://127.0.0.1:3001/submit-transaction")
        .json(&payload)
        .map_err(|e| format!("Failed to serialize request: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Network request failed: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Backend error: HTTP {}", response.status()));
    }

    let submit_response: SubmitResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    if submit_response.success {
        Ok(format!("Contract executed successfully!\n\nContract Result: {}\nTransaction Hash: {}\n\n{}",
            submit_response.result,
            submit_response.transaction_hash,
            submit_response.message))
    } else {
        Err(submit_response.message)
    }
}

/// Sign transaction with connected wallet and submit to backend
pub async fn sign_hello_transaction(connected_wallet: &ConnectedWallet, function: &ContractFunction) -> String {
    let source_account = &connected_wallet.address;

    // Step 1: Generate XDR via backend
    let xdr = match generate_xdr(source_account, "freighter", function).await {
        Ok(xdr) => xdr,
        Err(error) => return format!("XDR generation failed: {}", error),
    };

    // Step 2: Sign with wallet
    let network = "Test SDF Network ; September 2015";
    let signed_xdr = match sign_transaction(&xdr, network).await {
        Ok(signed_xdr) => signed_xdr,
        Err(error) => return format!("Transaction signing failed: {}", error),
    };

    // Step 3: Submit signed transaction to backend
    match submit_signed_transaction(&signed_xdr, "freighter", function).await {
        Ok(result) => result,
        Err(error) => format!("Transaction submission failed: {}", error),
    }
}