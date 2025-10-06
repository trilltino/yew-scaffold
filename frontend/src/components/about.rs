use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="main">
            <div class="wallet-section">
                <h2>{"About Stellar dApp"}</h2>
                <p>{"This is a modern Stellar Soroban decentralized application built with Yew (Rust WebAssembly) and integrated with Freighter wallet."}</p>

                <h3>{"Features"}</h3>
                <ul>
                    <li>{"Freighter wallet integration"}</li>
                    <li>{"XDR transaction generation"}</li>
                    <li>{"Smart contract interaction"}</li>
                    <li>{"Stellar testnet support"}</li>
                    <li>{"Built with Rust & WebAssembly"}</li>
                </ul>

                <h3>{"Technology Stack"}</h3>
                <ul>
                    <li>{"Frontend: Yew (Rust WebAssembly)"}</li>
                    <li>{"Backend: Axum (Rust)"}</li>
                    <li>{"Blockchain: Stellar Soroban"}</li>
                    <li>{"Wallet: Freighter"}</li>
                </ul>

                <h3>{"Contract Information"}</h3>
                <p>{"Contract ID: CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF"}</p>
                <p>{"Network: Stellar Testnet"}</p>
            </div>
        </div>
    }
}