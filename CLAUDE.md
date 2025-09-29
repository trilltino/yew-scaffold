# Claude Code Assistant Guide

This file helps Claude understand and work with this project efficiently.

## Project Overview
- **Type**: Stellar Soroban Multi-Wallet dApp
- **Frontend**: Yew (Rust WASM) on port 8080
- **Backend**: Axum (Rust) on port 3001
- **Wallets**: Freighter + Lobstr integration
- **Contract**: `CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF`

## Quick Commands
```bash
# Start frontend
cd frontend && trunk serve --port 8080

# Start backend
cd backend && cargo run

# Test backend
curl http://127.0.0.1:3001/health

# Build & test
cd frontend && trunk build
cd backend && cargo test
```

## Architecture
- **Wallet Abstraction**: `frontend/src/wallet/mod.rs`
- **API Handlers**: `backend/src/handlers.rs`
- **CORS Config**: `backend/src/utils.rs`
- **Contract Calls**: `backend/src/services/stellar.rs`

## Development Workflow
1. Always test wallet connections first
2. Use detailed console logging for debugging
3. Backend logs show XDR generation details
4. CORS is configured for `127.0.0.1:8080`
5. Network: Stellar Testnet

## Claude Commands
- "Run tests" → `cargo test` in both frontend/backend
- "Check wallet detection" → Look at browser console
- "Debug CORS" → Check `access-control-allow-origin` headers
- "Add wallet" → Extend `wallet/mod.rs` trait
- "Test contract" → Use `stellar contract invoke` CLI