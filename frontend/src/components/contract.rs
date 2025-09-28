use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContractProps {
    pub wallet_address: Option<String>,
    pub is_processing: bool,
    pub result_message: String,
    pub on_sign_transaction: Callback<()>,
}

#[function_component(ContractSection)]
pub fn contract_section(props: &ContractProps) -> Html {
    let sign_button_disabled = props.wallet_address.is_none() || props.is_processing;

    let on_sign = {
        let callback = props.on_sign_transaction.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <>
            <div class="contract-section">
                <h2>{"Contract Interaction"}</h2>
                <p>{"Call the hello_yew function on the Stellar testnet"}</p>
                <button
                    class="btn btn-success"
                    onclick={on_sign}
                    disabled={sign_button_disabled}
                >
                    {if props.is_processing { "Processing..." } else { "🔗 Call hello_yew" }}
                </button>
            </div>

            <div class="result-section">
                <h2>{"Result"}</h2>
                <div class="result-box">
                    {&props.result_message}
                </div>
            </div>
        </>
    }
}