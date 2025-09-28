use soroban_client::{
    Server, Options,
    transaction::{Account, TransactionBuilder, AccountBehavior, TransactionBuilderBehavior, TransactionBehavior},
    contract::{Contracts, ContractBehavior},
    xdr::{Limits, WriteXdr, ScVal, ScString},
};
use std::{cell::RefCell, rc::Rc};
use tracing::info;

use crate::error::{AppError, Result};

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

impl XdrConfig {
    pub fn validate(&self) -> Result<()> {
        if self.contract_id.is_empty() {
            return Err(AppError::Config("Contract ID cannot be empty".to_string()));
        }

        if !self.contract_id.starts_with('C') || self.contract_id.len() != 56 {
            return Err(AppError::Config("Invalid contract ID format".to_string()));
        }

        if self.network_passphrase.is_empty() {
            return Err(AppError::Config("Network passphrase cannot be empty".to_string()));
        }

        if self.rpc_url.is_empty() {
            return Err(AppError::Config("RPC URL cannot be empty".to_string()));
        }

        Ok(())
    }
}

pub async fn generate_hello_yew_xdr(config: &XdrConfig, source_account: &str) -> Result<String> {
    config.validate()?;

    info!("🚀 Generating XDR using soroban-client for contract: {}", config.contract_id);
    info!("📡 Using RPC: {}", config.rpc_url);
    info!("🌐 Network: {}", config.network_passphrase);

    let rpc = Server::new(&config.rpc_url, Options::default())
        .map_err(|e| AppError::StellarRpc(format!("Failed to connect to Soroban RPC: {:?}", e)))?;

    info!("🔍 Fetching account info for: {}", source_account);
    let account_response = rpc.get_account(source_account).await
        .map_err(|e| AppError::Account(format!("Failed to get account info: {:?}", e)))?;

    info!("✅ Account found, sequence: {}", account_response.sequence_number());

    let account = Account::new(source_account, &account_response.sequence_number())
        .map_err(|e| AppError::Account(format!("Failed to create account: {:?}", e)))?;

    let account_rc = Rc::new(RefCell::new(account));
    let mut tx_builder = TransactionBuilder::new(
        account_rc,
        &config.network_passphrase,
        None
    );

    info!("💰 Setting fee: 1,000,000 stroops (0.1 XLM)");
    tx_builder.fee(1000000u32);

    info!("📋 Creating contract call for function: simple");
    let contract = Contracts::new(&config.contract_id)
        .map_err(|e| AppError::Transaction(format!("Failed to create contract: {:?}", e)))?;

    let function_name = "simple";
    info!("🔍 Function name being used: '{}'", function_name);
    info!("🔍 Testing simple function with no parameters");
    let invoke_operation = contract.call(function_name, None);
    tx_builder.add_operation(invoke_operation);

    info!("🔨 Building transaction...");
    let tx = tx_builder.build();

    info!("⚙️ Preparing transaction (adding footprint and resource fees)...");
    let prepared_tx = rpc.prepare_transaction(&tx).await
        .map_err(|e| AppError::Transaction(format!("Failed to prepare transaction: {:?}", e)))?;

    info!("📦 Creating transaction envelope...");
    let envelope = prepared_tx.to_envelope()
        .map_err(|e| AppError::XdrEncoding(format!("Failed to create transaction envelope: {:?}", e)))?;

    info!("🔄 Encoding to base64 XDR...");
    let tx_envelope_xdr = envelope.to_xdr_base64(Limits::none())
        .map_err(|e| AppError::XdrEncoding(format!("Failed to encode XDR to base64: {:?}", e)))?;

    info!("✨ Successfully generated XDR envelope!");
    info!("📊 XDR length: {} characters", tx_envelope_xdr.len());
    info!("🔗 ACTUAL XDR: {}", tx_envelope_xdr);
    info!("🎯 Ready to send to Freighter wallet for signing");

    Ok(tx_envelope_xdr)
}

pub async fn submit_signed_transaction(signed_xdr: &str) -> Result<(String, String)> {
    info!("📤 Received signed transaction submission request");
    info!("📊 Signed XDR length: {} characters", signed_xdr.len());

    if signed_xdr.is_empty() {
        return Err(AppError::InvalidInput("Signed XDR cannot be empty".to_string()));
    }

    if signed_xdr.len() < 100 {
        return Err(AppError::InvalidInput("Signed XDR appears too short to be valid".to_string()));
    }

    let mock_hash = format!("{}...{}", &signed_xdr[..8], &signed_xdr[signed_xdr.len()-8..]);
    let contract_result = "✨ Signed XDR received! You can now submit this transaction to the Stellar network.\n\nExpected contract result: [\"yew-scaffold\", \"RPC\"]\n\nTo submit manually, use: stellar contract invoke --id CAZYXFLPF5JJFZYAOWWVJJB7FCVJPWRERJHJSYBBYAXIRKMVCTNBBKBN --source-account YOUR_ACCOUNT --network testnet -- hello_yew --to \"Yew Frontend\"".to_string();

    info!("✅ XDR processing complete");

    Ok((mock_hash, contract_result))
}