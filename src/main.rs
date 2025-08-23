use yew::prelude::*;
use yew_router::prelude::*;
mod freighter;
mod helloworld_bindings;
mod nav; 
mod wallet;// new: centralize wallet state
use wallet::use_wallet;
use wallet::WalletCtx;

#[function_component(App)]
fn app() -> Html {
    let main_style = "padding: 20px; font-family: 'Fira Sans', Helvetica, Arial, sans-serif; background-color: #f5f3f0; color: #5c4a37; min-height: 100vh;";
    let contract_result = use_state(|| "No contract calls yet".to_string());
    let wallet_ctx = use_wallet(); // single source of truth for wallet

    html! {
        <HashRouter>
            <ContextProvider<WalletCtx> context={wallet_ctx.clone()}>
                <Nav contract_result={contract_result.clone()} />
                <div style={main_style}>
                    <h1 style={"font-family: 'Alfa Slab One', serif;"}>{ "Welcome to your app!" }</h1>
                    <p>{ "This is a basic template to get your dapp started with the Stellar Design System and Soroban contracts. You can customize it further by adding your own contracts, components, and styles." }</p>

                    <h2 style={"font-family: 'Alfa Slab One', serif;"}>{ "Contract Result" }</h2>
                    <textarea
                        style={"width: 100%; height: 100px; padding: 10px; font-family: 'Fira Sans', Helvetica, Arial, sans-serif; border: 1px solid #5c4a37; border-radius: 4px; background-color: white; resize: vertical;"}
                        readonly=true
                        value={contract_result.to_string()}
                        placeholder="Contract results will appear here..."
                    />

                    <h2 style={"font-family: 'Alfa Slab One', serif;"}>{ "Wallet Status" }</h2>
                    <p>{ wallet_ctx.state.last_message.clone() }</p>
                </div>
            </ContextProvider<WalletCtx>>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
