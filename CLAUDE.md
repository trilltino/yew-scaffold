# Claude Code Assistant Guide

This file helps Claude understand and work with this project efficiently.

## Project Overview
- **Type**: Full-stack Rust Stellar Soroban dApp Scaffold & Building Tool
- **Purpose**: Template for building decentralized applications on Stellar blockchain
- **Frontend**: Yew (Rust WASM) on port 8080
- **Backend**: Axum (Rust) on port 3001
- **Database**: PostgreSQL with SQLx migrations
- **Wallets**: Freighter + Lobstr integration
- **Contract**: `CCFF5EA2CKR6VTHUTEKN7LNA26EPRSLZ6ZVBZFI2TRNTTD5C24BOKUIF`
- **Network**: Stellar Testnet

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

# Database migrations
cd backend && sqlx migrate run
```

## Project Structure

### Backend (`backend/src/`)
- **Authentication**: JWT-based auth with cookies
  - `auth/jwt.rs` - Token generation/validation
  - `auth/password.rs` - Password hashing (bcrypt)
  - `auth/cookies.rs` - Cookie management
  - `middleware/auth.rs` - Auth middleware
  - `extractors/current_user.rs` - User extraction

- **Database Layer**: PostgreSQL with repository pattern
  - `database/connection.rs` - Database pool
  - `database/models.rs` - User models
  - `database/repositories/user_repository.rs` - User CRUD
  - `migrations/` - SQLx migrations

- **Soroban Integration**: Advanced Stellar smart contract interaction
  - `services/soroban/client.rs` - RPC client
  - `services/soroban/manager.rs` - Contract manager
  - `services/soroban/cache.rs` - Response caching
  - `services/soroban/circuit_breaker.rs` - Fault tolerance
  - `services/soroban/queue.rs` - Request queuing
  - `services/soroban/pool.rs` - Connection pooling
  - `services/soroban/registry.rs` - Contract registry
  - `services/soroban/simulation.rs` - Transaction simulation
  - `services/soroban/events.rs` - Event streaming
  - `services/soroban/state.rs` - State management

- **API Handlers**:
  - `handlers/auth.rs` - Login, register, logout endpoints
  - `handlers/soroban.rs` - Smart contract endpoints
  - `handlers.rs` - Legacy handlers

- **Configuration**:
  - `config.rs` - Environment-based config
  - `error.rs` - Custom error types
  - `types.rs` - Shared types
  - `utils.rs` - CORS setup

### Frontend (`frontend/src/`)
- **Routing**: `router.rs` - Page routing with yew-router
- **Pages**:
  - `pages/home.rs` - Home page
  - `pages/login.rs` - Authentication page

- **Components**:
  - `components/navigation.rs` - Nav bar
  - `components/contract.rs` - Contract interaction
  - `components/blend.rs` - Blend Protocol integration
  - `components/reflector_oracle.rs` - Price oracle
  - `components/live_price_feed.rs` - Real-time prices
  - `components/soroban_metrics.rs` - Metrics display
  - `components/soroban_metrics_live.rs` - Live metrics
  - `components/soroban_test.rs` - Testing UI
  - `components/about.rs` - About page

- **Services**:
  - `services/api.rs` - Backend API client
  - `services/soroban_api.rs` - Soroban RPC client
  - `services/transaction.rs` - Transaction handling

- **Wallet Integration**:
  - `wallet/mod.rs` - Wallet trait abstraction
  - `wallet/freighter.rs` - Freighter wallet

- **State Management**:
  - `state.rs` - Global state context

### Reference Projects (`reference/`)
Read-only examples for learning best practices:

#### 1. **yew/** - Official Yew Framework
- **Examples**: 40+ working examples
  - `examples/router/` - Routing patterns
  - `examples/contexts/` - Global state management
  - `examples/futures/` - Async operations
  - `examples/function_todomvc/` - Complete app
- **Use Cases**:
  - Wallet state management → Study `contexts/`
  - Async API calls → Study `futures/`
  - Form handling → Study `function_todomvc/`

#### 2. **axum/** - Official Axum Framework
- **Examples**: 40+ examples
  - `examples/jwt/` - JWT authentication
  - `examples/sessions/` - Session management
  - `examples/cors/` - CORS configuration
  - `examples/error-handling-and-dependency-injection/`
  - `examples/websockets/` - WebSocket support

#### 3. **rust-web-app/** - Production Axum Best Practices
- **Error Handling**: `crates/libs/lib-core/src/model/error.rs`
- **Authentication**: `crates/services/web-server/src/web/mw_auth.rs`
- **Repository Pattern**: `crates/libs/lib-core/src/model/`
- **API Structure**: `crates/services/web-server/src/web/routes_*.rs`
- **Testing**: `crates/services/web-server/tests/`
- **Database**: `crates/libs/lib-core/src/model/store/`

#### 4. **rs-soroban-sdk/** - Soroban Smart Contract SDK
- Official SDK source code
- Contract examples and tests
- Token standards

#### 5. **rs-soroban-client/** - Soroban Client Library
- RPC client implementation
- Transaction building examples
- Network interaction patterns

#### 6. **reflector-contract/** - Oracle Contract Example
- Price feed oracle implementation
- Real-world Soroban contract

## Architecture Patterns

### Current Implementation
| Feature | Implementation | File |
|---------|---------------|------|
| Frontend Framework | Yew (function components) | `frontend/src/main.rs` |
| Backend Framework | Axum with Tower middleware | `backend/src/main.rs` |
| Database | PostgreSQL + SQLx | `backend/src/database/` |
| Auth | JWT + HTTP-only cookies | `backend/src/auth/` |
| Wallet | Trait-based abstraction | `frontend/src/wallet/mod.rs` |
| Soroban Client | Advanced client with caching | `backend/src/services/soroban/` |
| Error Handling | Custom error types | `backend/src/error.rs` |
| State Management | Yew contexts | `frontend/src/state.rs` |

### Reference Comparisons
| Feature | This Project | rust-web-app | Notes |
|---------|-------------|--------------|-------|
| Framework | Axum | Axum | ✅ Same |
| Database | PostgreSQL + SQLx | PostgreSQL + SQLx | ✅ Same |
| Auth | JWT + cookies | JWT + middleware | 🔄 Similar pattern |
| Frontend | Yew (WASM) | None | ➕ Additional |
| Purpose | Stellar dApp | General API | 🎯 Specialized |

## Development Workflow
1. **Wallet First**: Always test wallet connections before contract calls
2. **Logging**: Use detailed console logging for debugging
   - Frontend: `log::info!()`, `log::error!()`
   - Backend: `tracing::info!()`, `tracing::error!()`
3. **XDR Details**: Backend logs show transaction XDR generation
4. **CORS**: Configured for `127.0.0.1:8080` in `backend/src/utils.rs`
5. **Database**: Run migrations before starting backend
6. **Testing**: Test both frontend and backend independently

## Key Features

### ✅ Implemented
- Multi-wallet support (Freighter, Lobstr)
- JWT authentication with secure cookies
- User registration and login
- PostgreSQL database with migrations
- Soroban contract interaction
- Blend Protocol integration (read-only queries)
- Reflector Oracle price feeds
- Live metrics and monitoring
- Circuit breaker pattern for reliability
- Request caching and queuing
- Transaction simulation
- Event streaming

### 🚧 Advanced Features Available
See [BLEND_FEATURES.md](BLEND_FEATURES.md) for:
- Reserve analytics
- User position tracking
- Emissions & rewards
- Liquidation monitoring
- Backstop analytics

## Claude Commands
- **"Run tests"** → See [TESTING.md](TESTING.md) for comprehensive guide
  - Backend: `cd backend && cargo test`
  - Frontend: `cd frontend && cargo test`
- **"Check wallet"** → Look at browser console for Freighter detection
- **"Debug CORS"** → Check `access-control-allow-origin` headers
- **"Add wallet"** → Extend `frontend/src/wallet/mod.rs` trait
- **"Test contract"** → Use `stellar contract invoke` CLI
- **"Run migrations"** → `cd backend && sqlx migrate run`
- **"Study pattern X"** → Check `reference/` for examples

## Reference Study Guide

### When Implementing New Features

**Authentication/Authorization**:
```bash
# Study these files
reference/rust-web-app/crates/services/web-server/src/web/mw_auth.rs
reference/axum/examples/jwt/src/main.rs
# Compare with
backend/src/auth/jwt.rs
backend/src/middleware/auth.rs
```

**Component Patterns**:
```bash
# Study these files
reference/yew/examples/function_todomvc/src/lib.rs
reference/yew/examples/contexts/src/main.rs
# Compare with
frontend/src/components/blend.rs
frontend/src/state.rs
```

**Error Handling**:
```bash
# Study these files
reference/rust-web-app/crates/libs/lib-core/src/model/error.rs
reference/axum/examples/error-handling-and-dependency-injection/
# Compare with
backend/src/error.rs
```

**Database Patterns**:
```bash
# Study these files
reference/rust-web-app/crates/libs/lib-core/src/model/store/
# Compare with
backend/src/database/repositories/
```

## Environment Variables
```bash
# Backend (.env)
DATABASE_URL=postgresql://user:pass@localhost:5432/dbname
JWT_SECRET=your-secret-key
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_NETWORK_PASSPHRASE=Test SDF Network ; September 2015

# Frontend (built into WASM)
BACKEND_URL=http://127.0.0.1:3001
```

## Common Tasks

### Add New Component
1. Create in `frontend/src/components/`
2. Export in `frontend/src/components/mod.rs`
3. Import in parent component
4. Study `reference/yew/examples/` for patterns

### Add New API Endpoint
1. Define handler in `backend/src/handlers/`
2. Add route in `backend/src/main.rs`
3. Add type in `backend/src/types.rs`
4. Study `reference/rust-web-app/` for patterns

### Add Database Table
1. Create migration: `sqlx migrate add <name>`
2. Write SQL in `backend/migrations/`
3. Add model to `backend/src/database/models.rs`
4. Create repository in `backend/src/database/repositories/`
5. Run: `sqlx migrate run`

### Add New Wallet
1. Implement trait in `frontend/src/wallet/mod.rs`
2. Add detection logic
3. Test connection flow
4. Update documentation

## Debugging Tips
- **Frontend not connecting**: Check CORS in browser DevTools
- **Contract call fails**: Check XDR in backend logs
- **Wallet not detected**: Check browser extension installation
- **Database errors**: Verify migrations ran successfully
- **Auth failing**: Check JWT secret matches in .env
- **Reference docs**: All repos have clean git status, ready to read

## Testing

**📚 Comprehensive testing guide:** [TESTING.md](TESTING.md) **← START HERE**

### Test Structure
```
backend/tests/
├── common/mod.rs          # Test infrastructure (TestDb, TestUser, helpers)
├── health_tests.rs        # Health endpoint tests
├── auth_tests.rs          # Authentication flow tests (40+ tests)
├── repository_tests.rs    # Database layer tests
├── middleware_tests.rs    # Auth & CORS middleware tests
└── soroban_tests.rs       # Soroban integration tests

frontend/src/
├── types_test.rs          # Type method tests
└── state_test.rs          # State reducer tests
```

### Quick Test Commands
```bash
# Backend (requires PostgreSQL running)
cd backend && cargo test

# Frontend (pure Rust, no WASM)
cd frontend && cargo test

# Run specific test
cargo test test_signup_success

# With output
cargo test -- --nocapture
```

### Test Coverage Summary
- ✅ **60+ automated tests** (40 backend + 20 frontend)
- ✅ Isolated test databases (parallel execution safe)
- ✅ Full auth flow coverage (signup, login, logout, protected routes)
- ✅ Repository CRUD operations (create, read, update)
- ✅ Middleware testing (auth, CORS)
- ✅ State management logic (reducer, transitions)
- ✅ Error case testing (invalid inputs, duplicates, auth failures)

## Resources
- **🧪 Testing Guide**: [TESTING.md](TESTING.md) - **Complete Rust testing tutorial**
- **Stellar Docs**: https://developers.stellar.org/docs/soroban
- **Blend Docs**: https://docs.blend.capital/
- **Yew Book**: https://yew.rs/docs/
- **Axum Docs**: https://docs.rs/axum/
- **SQLx Guide**: https://github.com/launchbadge/sqlx

## Notes for Claude
- Reference repos are **read-only** - don't modify them
- Focus on understanding patterns, not copying code
- This scaffold is a starting point - adapt to user needs
- All git submodules are complete and up-to-date
- **📚 Read [TESTING.md](TESTING.md) for testing guidance**
- **✅ Always run tests before committing** - `cargo test`
- Prioritize code quality and type safety (it's all Rust!)