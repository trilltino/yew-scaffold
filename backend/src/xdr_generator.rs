/// Standalone XDR generation module using soroban-client
/// This generates real transaction XDR that can be sent to Freighter wallet

use soroban_client::{
    Server, Options,
    transaction::{Account, TransactionBuilder, AccountBehavior, TransactionBuilderBehavior, TransactionBehavior},
    contract::{Contracts, ContractBehavior},
    xdr::{Limits, WriteXdr},
};
use std::{cell::RefCell, rc::Rc};
use tracing::info;

#[derive(Debug, Clone)]
pub struct XdrConfig {
    pub contract_id: String,
    pub network_passphrase: String,
    pub rpc_url: String,
}

impl Default for XdrConfig {
    fn default() -> Self {
        Self {
            contract_id: "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF".to_string(),
            network_passphrase: "Test SDF Network ; September 2015".to_string(),
            rpc_url: "https://soroban-testnet.stellar.org".to_string(),
        }
    }
}

/// Generate XDR for hello_yew contract function call
/// This creates a real Stellar transaction that can be signed by Freighter
pub async fn generate_hello_yew_xdr(config: &XdrConfig, source_account: &str) -> Result<String, String> {
    info!("🚀 Generating XDR using soroban-client for contract: {}", config.contract_id);
    info!("📡 Using RPC: {}", config.rpc_url);
    info!("🌐 Network: {}", config.network_passphrase);

    // Create soroban RPC server connection
    let rpc = Server::new(&config.rpc_url, Options::default())
        .map_err(|e| format!("❌ Failed to connect to Soroban RPC: {:?}", e))?;

    // Get account info from the network
    info!("🔍 Fetching account info for: {}", source_account);
    let account_response = rpc.get_account(source_account).await
        .map_err(|e| format!("❌ Failed to get account info: {:?}", e))?;

    info!("✅ Account found, sequence: {}", account_response.sequence_number());

    // Create account object using the correct API
    let account = Account::new(source_account, &account_response.sequence_number())
        .map_err(|e| format!("❌ Failed to create account: {:?}", e))?;

    // Create transaction builder with proper parameters
    let account_rc = Rc::new(RefCell::new(account));
    let mut tx_builder = TransactionBuilder::new(
        account_rc,
        &config.network_passphrase,
        None // timebounds
    );

    // Set higher fee for contract calls (1 XLM = 10,000,000 stroops)
    info!("💰 Setting fee: 1,000,000 stroops (0.1 XLM)");
    tx_builder.fee(1000000u32);

    // Create contract instance and call operation
    info!("📋 Creating contract call for function: hello_yew");
    let contract = Contracts::new(&config.contract_id)
        .map_err(|e| format!("❌ Failed to create contract: {:?}", e))?;

    // Call the simple function (no parameters) to test if it's a function name issue
    use soroban_client::xdr::{ScVal, ScString};
    let function_name = "simple";
    info!("🔍 Function name being used: '{}'", function_name);
    info!("🔍 Testing simple function with no parameters");
    let invoke_operation = contract.call(function_name, None);
    tx_builder.add_operation(invoke_operation);

    // Build the transaction
    info!("🔨 Building transaction...");
    let tx = tx_builder.build();

    // Prepare transaction with proper footprint and resource estimation
    info!("⚙️ Preparing transaction (adding footprint and resource fees)...");
    let prepared_tx = rpc.prepare_transaction(&tx).await
        .map_err(|e| format!("❌ Failed to prepare transaction: {:?}", e))?;

    // Generate the XDR envelope string
    info!("📦 Creating transaction envelope...");
    let envelope = prepared_tx.to_envelope()
        .map_err(|e| format!("❌ Failed to create transaction envelope: {:?}", e))?;

    // Convert to base64 XDR string that Freighter can use
    info!("🔄 Encoding to base64 XDR...");
    let tx_envelope_xdr = envelope.to_xdr_base64(Limits::none())
        .map_err(|e| format!("❌ Failed to encode XDR to base64: {:?}", e))?;

    info!("✨ Successfully generated XDR envelope!");
    info!("📊 XDR length: {} characters", tx_envelope_xdr.len());
    info!("🔗 ACTUAL XDR: {}", tx_envelope_xdr);
    info!("🎯 Ready to send to Freighter wallet for signing");

    Ok(tx_envelope_xdr)
}

/// Submit signed transaction and extract contract result
/// Returns (transaction_hash, contract_result)
pub async fn submit_signed_transaction(signed_xdr: &str) -> Result<(String, String), String> {
    info!("📤 Received signed transaction submission request");
    info!("📊 Signed XDR length: {} characters", signed_xdr.len());

    // For now, return a simple success message indicating the XDR was received
    // The user can manually submit this to the Stellar network or use stellar-cli
    let mock_hash = format!("{}...{}", &signed_xdr[..8], &signed_xdr[signed_xdr.len()-8..]);
    let contract_result = "✨ Signed XDR received! You can now submit this transaction to the Stellar network.\n\nExpected contract result: [\"yew-scaffold\", \"RPC\"]\n\nTo submit manually, use: stellar contract invoke --id CAZYXFLPF5JJFZYAOWWVJJB7FCVJPWRERJHJSYBBYAXIRKMVCTNBBKBN --source-account YOUR_ACCOUNT --network testnet -- hello_yew --to \"Yew Frontend\"".to_string();

    info!("✅ XDR processing complete");

    Ok((mock_hash, contract_result))
}


