/// Examples of Stellar contract interactions
/// Use these patterns to add new contract functions

use soroban_client::*;

// Current contract: CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF
// Current function: simple() -> String

/// Example 1: Contract function with parameters
/*
Contract function signature:
pub fn greet(name: String) -> String

Backend implementation:
*/
async fn call_greet_function(
    client: &SorobanClient,
    source_account: &str,
    name: &str,
) -> Result<String, String> {
    let contract_id = "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF";

    // Create parameters
    let params = vec![
        ScVal::String(ScString(name.try_into().unwrap())),
    ];

    // Build transaction
    let tx = client
        .contract_call(source_account, contract_id, "greet", params)
        .await?;

    Ok(tx.to_xdr())
}

/// Example 2: Contract function with multiple parameters
/*
Contract function signature:
pub fn add_numbers(a: i64, b: i64) -> i64

Backend implementation:
*/
async fn call_add_numbers(
    client: &SorobanClient,
    source_account: &str,
    a: i64,
    b: i64,
) -> Result<String, String> {
    let contract_id = "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF";

    let params = vec![
        ScVal::I64(a),
        ScVal::I64(b),
    ];

    let tx = client
        .contract_call(source_account, contract_id, "add_numbers", params)
        .await?;

    Ok(tx.to_xdr())
}

/// Example 3: Contract function with storage
/*
Contract function signature:
pub fn store_data(key: String, value: String) -> String

Backend implementation:
*/
async fn call_store_data(
    client: &SorobanClient,
    source_account: &str,
    key: &str,
    value: &str,
) -> Result<String, String> {
    let contract_id = "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF";

    let params = vec![
        ScVal::String(ScString(key.try_into().unwrap())),
        ScVal::String(ScString(value.try_into().unwrap())),
    ];

    let tx = client
        .contract_call(source_account, contract_id, "store_data", params)
        .await?;

    Ok(tx.to_xdr())
}

/// Example 4: Read-only contract call (query)
async fn query_contract_data(
    client: &SorobanClient,
    key: &str,
) -> Result<String, String> {
    let contract_id = "CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF";

    let params = vec![
        ScVal::String(ScString(key.try_into().unwrap())),
    ];

    // For read-only calls, you might not need a source account
    let result = client
        .simulate_transaction(contract_id, "get_data", params)
        .await?;

    Ok(result)
}

/// Example 5: Frontend component for contract interaction
/*
#[function_component(ContractGreeting)]
pub fn contract_greeting() -> Html {
    let name = use_state(|| String::new());
    let result = use_state(|| String::new());
    let loading = use_state(|| false);

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_submit = {
        let name = name.clone();
        let result = result.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let name = name.clone();
            let result = result.clone();
            let loading = loading.clone();

            spawn_local(async move {
                loading.set(true);

                match call_greet_endpoint(&*name).await {
                    Ok(response) => result.set(response),
                    Err(error) => result.set(format!("Error: {}", error)),
                }

                loading.set(false);
            });
        })
    };

    html! {
        <div class="contract-greeting">
            <h3>{"Contract Greeting"}</h3>
            <div class="form-group">
                <input
                    type="text"
                    placeholder="Enter your name"
                    value={(*name).clone()}
                    oninput={on_name_change}
                />
                <button onclick={on_submit} disabled={*loading}>
                    {if *loading { "Calling..." } else { "Greet" }}
                </button>
            </div>
            <div class="result">
                {&*result}
            </div>
        </div>
    }
}

async fn call_greet_endpoint(name: &str) -> Result<String, String> {
    let url = format!("http://127.0.0.1:3001/greet?name={}", name);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Backend error: HTTP {}", response.status()));
    }

    let greet_response: GreetResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    if greet_response.success {
        Ok(greet_response.message)
    } else {
        Err(greet_response.error)
    }
}
*/

/// Example 6: Contract deployment (for reference)
/*
async fn deploy_contract(
    client: &SorobanClient,
    deployer_account: &str,
    wasm_hash: &str,
) -> Result<String, String> {
    let tx = client
        .deploy_contract(deployer_account, wasm_hash)
        .await?;

    Ok(tx.to_xdr())
}
*/

/// Example 7: Event listening (if supported)
/*
async fn listen_for_contract_events(
    client: &SorobanClient,
    contract_id: &str,
) -> Result<Vec<ContractEvent>, String> {
    let events = client
        .get_events(contract_id)
        .await?;

    Ok(events)
}
*/

// Testing contract functions with Stellar CLI:
/*
# Install Stellar CLI
cargo install --locked stellar-cli

# Configure network
stellar network add testnet --rpc-url https://soroban-testnet.stellar.org --network-passphrase "Test SDF Network ; September 2015"

# Invoke contract function
stellar contract invoke \
  --id CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF \
  --source-account GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54 \
  --network testnet \
  -- simple

# Invoke with parameters
stellar contract invoke \
  --id CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF \
  --source-account GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54 \
  --network testnet \
  -- greet --name "Claude"
*/