<img width="1918" height="990" alt="Screenshot 2025-08-17 123131" src="https://github.com/user-attachments/assets/a5c65996-605b-42f2-b33c-3896d10fdc6e" />

Yew Template for Soroban dApps
A Yew SPA wired directly to Soroban smart contracts.
Built with Rust, compiled to WebAssembly using Trunk.
Freighter wallet integration for account connection and signing.
Client-side routing out of the box (About, Voters, Projects, 404).
Deployable as static files, so end users only need a browser and Freighter.

Why it matters

This template is a stepping stone to building an entire decentralized apps in Rust. 

How it can scale

Axum as backend: Instead of Express or Next.js, you add Axum. 
It runs fast, async-native, and composable with tower middleware. It handles API routes, auth, and contract indexing with minimal boilerplate.
Shared types: Define your domain types in a shared crate. Yew, Axum, and Soroban all use the same Rust structs. This removes JSON parsing errors that plague JS backends.

The bigger picture

Yew + Axum + Soroban is more than just a template. It’s a path to a full-stack Rust ecosystem where: You don’t switch between Rust and JavaScript.
You write once, reuse everywhere.
You get the speed and safety guarantees of Rust across all layers.
This makes it possible to create teams of full-stack Rust developers who can build secure, scalable, production-grade dApps without relying on Node.js or TypeScript for the frontend/backend split.

Similarity to Scaffold Stellar

This project plays a role for Yew that Scaffold Stellar does for React/Next.js:
Scaffold Stellar provides a React-based starter with contract integration and wallet support.
This Yew template provides the same foundation, but for a Rust-only frontend workflow.
Both aim to reduce boilerplate, accelerate prototyping, and standardize dApp structure.
Where Scaffold Stellar helps JS developers onboard to Soroban, this template helps Rust developers stay in their ecosystem without switching stacks.


# Dev prerequisites
Install Rust
rustup target add wasm32-unknown-unknown
cargo install trunk
git clone https://github.com/trilltino/yew-scaffold
cd yew-scaffold
cargo build
trunk serve


# To Do
Tutorial


