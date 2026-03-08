# Yew Soroban Scaffold

**The ultimate production-ready bridge between high-performance Rust web services and the Stellar/Soroban ecosystem.**

This scaffold provides a sophisticated, resilient architecture for building decentralized applications that require a robust backend layer alongside a high-performance WebAssembly frontend.

---

## Deep Backend Architecture: Axum + Soroban Fusion

The backend is not just an API; it's a **Scalable Contract Management Infrastructure** designed to handle the complexities of blockchain interaction at scale.

### Resilience & Reliability
-   **Circuit Breaker Protection**: Prevents cascading failures when RPC nodes are slow or unresponsive. Automatically "trips" to protect system resources and provides fallback behaviors.
-   **Intelligent RPC Pooling**: Custom `StellarRpcPool` manages multiple connections to Soroban RPC providers, optimizing throughput while staying within rate limits.
-   **Async Transaction Queue**: A sophisticated background processing system that handles transaction submission outside the request-response cycle, featuring:
    -   **Priority-based execution** (Low, Normal, High).
    -   **Automatic retry logic** for transient network errors.
    -   **Persistent operation tracking** for long-running chain interactions.

### Performance Optimization (Multi-Level Caching)
We minimize expensive RPC calls and blockchain latency through a strategically implemented caching layer:
-   **XDR Cache**: Cached transaction envelopes for hot contract functions.
-   **Event Cache**: 30-second TTL for contract events, drastically reducing dashboard load times.
-   **Simulation Cache**: Expensive pre-flight simulations are cached to reduce latency for repeated user actions.
-   **State Cache**: Durability-aware caching for `Persistent` and `Temporary` ledger entries.

### Advanced Soroban Integration
-   **Generic Contract Invocation**: Call **any** Soroban contract function dynamically via simple JSON payloads. No need for pre-generated bindings for every contract.
-   **Deep Simulation Engine**: Real-time pre-flight checks that provide:
    -   Accurate **Gas & Resource Fee** estimation.
    -   **Footprint Analysis** (Read/Write sets).
    -   **State Change Preview**: See exactly what the transaction will do before signing.
-   **On-Chain Event Streaming**: High-performance event querying with complex filters (topics, contract IDs, event types) and cursor-based pagination.
-   **Ledger Data Access**: Direct access to contract storage with support for different durability levels and TTL monitoring.

---

## Enterprise-Grade Security
-   **Secure JWT Auth**: State-of-the-art authentication using JSON Web Tokens.
-   **Stealth Sessions**: HTTP-Only, Secure, and SameSite cookies prevent XSS and CSRF attacks.
-   **Cryptographic Strength**: Password hashing using Argon2/Bcrypt and secure wallet-to-account linking.
-   **PostgreSQL Persistence**: Robust session management and user metadata storage.

---

## Frontend: Yew (WebAssembly)
A modern, component-based frontend built entirely in Rust, compiled to WebAssembly for near-native performance.
-   **Component Architecture**: Clean, reusable UI components with efficient state management.
-   **Wallet Integration**: First-class support for **Freighter**, enabling seamless on-chain signing.
-   **Unified API Service**: A shared DTO-based communication layer ensuring type safety between frontend and backend.
-   **Real-time UI**: Reactive updates using event hooks and blockchain state synchronization.

---

## Industrial-Strength Testing
-   **Backend Integration Suite**: Testing core API endpoints against live testnet mocks.
-   **Layered Testing**: Repository-level, service-level, and middleware-level unit tests.
-   **WASM Browser Tests**: Validating frontend logic within the WebAssembly environment.

---

## Tech Stack
-   **Backend**: Rust, Axum, Tokio, SQLx (PostgreSQL), Soroban-Client
-   **Frontend**: Rust, Yew, WebAssembly, TailwindCSS (optional)
-   **Blockchain**: Stellar, Soroban SDK
-   **DevOps**: Docker ready, GitHub Actions workflow templates

---

## Getting Started

1.  **Clone the Repo**: `git clone --recursive https://github.com/trilltino/yew-scaffold.git`
2.  **Environment Setup**: Copy `.env.example` to `.env` in the root and backend folders.
3.  **Run Backend**: `cd backend && cargo run`
4.  **Run Frontend**: `cd frontend && trunk serve`

---

*Built with passion for the Stellar/Soroban community. Empowering builders to create the next generation of resilient DeFi and Web3 applications.*
