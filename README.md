<img width="1918" height="990" alt="Screenshot 2025-08-17 123131" src="https://github.com/user-attachments/assets/a5c65996-605b-42f2-b33c-3896d10fdc6e" />

# Yew Soroban dApp Template

A production-ready template for building full-stack decentralized applications on Stellar using Rust's modern web ecosystem.

## Architecture Overview

This template demonstrates a **zero-JavaScript** approach to dApp development, leveraging Rust's type system and performance characteristics across the entire stack:

- **Frontend**: Yew framework compiled to WebAssembly
- **Wallet Integration**: Freighter wallet with type-safe bindings
- **Smart Contracts**: Direct Soroban contract interaction without RPC abstraction layers

## Technical Philosophy

### Type-Safe Contract Integration
Unlike traditional dApp architectures that rely on runtime type checking and JSON serialization, this template uses Rust's ownership model and type system to guarantee contract call safety at compile time.

### Shared Domain Models
The shared crate pattern eliminates serialization overhead and runtime type errors common in polyglot stacks:

```rust
// Shared types across all layers
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    pub address: String,
    pub amount: i128,
}
```

### Wasm - Bindgen
The crate allows for JSValues to be handled, "freighterApi" is called in index.html
```rust
fn get_freighter_api() -> Result<JsValue, FreighterError> {
    let window = window().ok_or(FreighterError::NoWindow)?;
    let api = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi"))?;
    if api.is_undefined() || api.is_null() {
        return Err(FreighterError::FreighterExtNotFound);
    }
    Ok(api)
}


## Planned Architecture: Full-Stack Rust


```


┌─────────────┐    ┌──────────────┐    ┌─────────────────┐
│  Yew (WASM) │────│ Axum Backend │────│ Soroban Network │
│   Frontend  │    │   API/Auth   │    │   Contracts     │
└─────────────┘    └──────────────┘    └─────────────────┘
        │                   │                     │
        └───────── Shared Rust Types ─────────────┘


```



### Axum Integration (Roadmap)
- **Async-first**: Built on Tokio for optimal I/O performance
- **Composable middleware**: Type-safe request/response transformation
- **Contract indexing**: Efficient event parsing and state aggregation
- **Authentication**: JWT/session management with compile-time route protection


### Prerequisites
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# WASM target and tools
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

# Freighter wallet (browser extension)
# Install from Chrome Web Store or Firefox Add-ons
```

### Quick Start
```bash
git clone https://github.com/trilltino/yew-scaffold
cd yew-scaffold

# Build and serve with hot-reload
trunk serve

# Production build
trunk build --release
```

## Implementation Status

- ✅ Yew frontend foundation
- ✅ Freighter wallet integration
- ✅ Soroban contract calling patterns
- ✅ Shared type definitions
- 🚧 Error handling & user feedback
- 🚧 Comprehensive documentation
- 📋 Axum backend integration
- 📋 Contract event indexing
- 📋 Production deployment guides

## Contributing

This project aims to establish patterns and best practices for full-stack Rust dApp development. Contributions focusing on developer experience, type safety, and performance optimizations are particularly welcome.

### Areas of Interest
- Advanced error handling patterns
- Contract state synchronization strategies  
- WASM bundle optimization techniques
- Production deployment configurations



