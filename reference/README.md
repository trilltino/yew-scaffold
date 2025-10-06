# Reference Documentation

This directory contains reference implementations and documentation for the yew-scaffold project.

## 📁 Contents

### Reference Repositories (Git Submodules)

These are complete, production-grade examples for learning patterns:

- **[axum/](axum/)** - Official Axum web framework examples
  - Testing patterns in `examples/testing/`
  - Middleware, error handling, state management

- **[yew/](yew/)** - Official Yew framework repository
  - Component patterns, hooks, routing
  - WASM-specific implementations

- **[rust-web-app/](rust-web-app/)** - Production Rust web application patterns
  - Full-stack architecture
  - Database migrations, authentication

- **[rs-soroban-sdk/](rs-soroban-sdk/)** - Stellar Soroban smart contract SDK
  - Contract development patterns
  - Testing Soroban contracts

- **[rs-soroban-client/](rs-soroban-client/)** - Soroban RPC client
  - Transaction building
  - XDR encoding/decoding

- **[reflector-contract/](reflector-contract/)** - Reflector oracle contract example
  - Real-world Soroban implementation
  - Price feed oracle patterns

### Documentation Guides

- **[WASM_TESTING_GUIDE.md](WASM_TESTING_GUIDE.md)** ✨ NEW!
  - Complete guide to testing WASM applications
  - Based on official wasm-bindgen documentation
  - Explains native vs WASM testing
  - Includes async testing patterns
  - Browser testing with wasm-pack
  - CI/CD configuration examples

## 🎯 How to Use These References

### For Backend Development

Look at:
- `axum/examples/testing/` for HTTP testing patterns
- `rust-web-app/` for production architecture
- `rs-soroban-client/` for Stellar/Soroban integration

### For Frontend Development

Look at:
- `yew/` for component and state management patterns
- `WASM_TESTING_GUIDE.md` for testing strategies
- `axum/examples/testing/` for understanding the backend API

### For Soroban/Stellar Development

Look at:
- `rs-soroban-sdk/` for contract development
- `rs-soroban-client/` for client-side integration
- `reflector-contract/` for real-world contract examples

## 📚 Testing Documentation

We have comprehensive testing coverage:

### Native Tests (Current Implementation)
- **Backend**: 48 integration tests across 6 test files
- **Frontend**: 20 unit tests (native Rust, no WASM)
- **Total**: 68 automated tests

See [../TESTING.md](../TESTING.md) for complete testing guide.

### WASM Tests (Optional, When Needed)
- Use for browser-specific APIs only
- See [WASM_TESTING_GUIDE.md](WASM_TESTING_GUIDE.md) for complete guide
- Not needed for current implementation (pure logic testing)

## 🔄 Keeping References Updated

The git submodules can be updated with:

```bash
# Update all submodules to latest
git submodule update --remote

# Update specific submodule
git submodule update --remote reference/axum
```

## 🌟 Key Patterns Demonstrated

### From Axum Examples
- `ServiceExt::oneshot()` for testing without HTTP server
- Tower middleware composition
- Error handling with custom error types

### From Yew Examples
- Component composition patterns
- State management with reducers
- Hook usage patterns

### From Rust Web App
- Layered architecture (handlers, services, repositories)
- Database connection pooling
- JWT authentication

### From Soroban References
- Contract invocation patterns
- XDR transaction building
- Network configuration

## 📖 Documentation Index

| Document | Purpose | Status |
|----------|---------|--------|
| [WASM_TESTING_GUIDE.md](WASM_TESTING_GUIDE.md) | Complete guide to WASM testing with wasm-bindgen | ✅ Complete |
| [../TESTING.md](../TESTING.md) | Rust testing fundamentals + our test implementation | ✅ Complete |
| [../CLAUDE.md](../CLAUDE.md) | Project overview for Claude Code assistant | ✅ Complete |

## 🚀 Quick Start for New Developers

1. **Clone with submodules**:
   ```bash
   git clone --recursive https://github.com/your-repo/yew-scaffold.git
   ```

2. **Read the main testing guide**:
   - Start with [../TESTING.md](../TESTING.md)
   - Understand native vs WASM testing with [WASM_TESTING_GUIDE.md](WASM_TESTING_GUIDE.md)

3. **Explore reference code**:
   - Backend patterns: `axum/examples/testing/`
   - Frontend patterns: `yew/examples/`
   - Soroban patterns: `reflector-contract/`

4. **Run existing tests**:
   ```bash
   # Backend tests
   cd backend && cargo test

   # Frontend tests
   cd frontend && cargo test
   ```

## 📝 Notes

- All reference repositories are read-only git submodules
- Documentation is kept in sync with actual implementation
- Tests use patterns from these references
- WASM testing guide is based on official wasm-bindgen documentation (https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/)
