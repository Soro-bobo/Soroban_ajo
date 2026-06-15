# Ajo Platform

A decentralized Ajo (ROSCA) savings group platform built on Stellar/Soroban. Participants form savings circles, contribute fixed amounts on a schedule, and receive lump-sum payouts in rotating order.

## Monorepo Structure

```
ajo-platform/
├── frontend/          # Next.js 14 App Router + TypeScript
├── backend/           # Rust + Axum + SQLx + PostgreSQL
├── contracts/ajo/     # Soroban smart contract (Rust)
├── scripts/           # Deploy, seed, keypair utilities
└── documentation/     # Architecture, API reference, guides
```

## Prerequisites

| Tool | Version |
|------|---------|
| Node.js | ≥ 20 |
| pnpm | ≥ 9 |
| Rust | stable (1.78+) |
| cargo | latest |
| soroban-cli | 20.x |
| PostgreSQL | 15+ |
| Docker (optional) | 24+ |

## Quick Start

### 1. Clone and install

```bash
git clone https://github.com/your-org/ajo-platform.git
cd ajo-platform
pnpm install
```

### 2. Configure environment

```bash
# Backend
cp backend/.env.example backend/.env
# Edit backend/.env with your database URL, JWT secret, etc.

# Frontend
cp frontend/.env.example frontend/.env.local
# Edit frontend/.env.local with your API URL
```

### 3. Run database migrations

```bash
cd backend
sqlx database create
sqlx migrate run
```

### 4. Seed the database (optional)

```bash
bash scripts/seed_db.sh
```

### 5. Start the backend

```bash
cd backend
cargo run
# Listens on http://localhost:8080
```

### 6. Start the frontend

```bash
pnpm dev:frontend
# Listens on http://localhost:3000
```

### 7. Build and deploy the contract (Testnet)

```bash
bash scripts/generate_keypair.sh   # generates deployer keypair
bash scripts/deploy_testnet.sh     # builds WASM + deploys to testnet
```

## Development

### Backend

```bash
# Watch mode with auto-reload
cargo install cargo-watch
cd backend && cargo watch -x run

# Run tests
cargo test

# Check without building
cargo check
```

### Frontend

```bash
cd frontend
pnpm dev      # development server
pnpm build    # production build
pnpm lint     # ESLint
```

### Contracts

```bash
cd contracts/ajo
cargo test                                             # unit + integration tests
cargo build --target wasm32-unknown-unknown --release  # build WASM
```

## API Overview

Base URL: `http://localhost:8080/api/v1`

| Method | Path | Description |
|--------|------|-------------|
| POST | `/auth/register` | Create account |
| POST | `/auth/login` | Authenticate, receive token pair |
| POST | `/auth/refresh` | Rotate refresh token |
| GET | `/health` | Health check |
| GET | `/groups` | List groups (paginated) |
| POST | `/groups` | Create group |
| GET | `/groups/:id` | Get group detail |
| POST | `/groups/:id/join` | Join a group |
| GET | `/groups/:id/contributions` | List contributions |
| POST | `/contributions` | Record contribution |

See [documentation/api-reference.md](documentation/api-reference.md) for full request/response schemas.

## Architecture

See [documentation/architecture.md](documentation/architecture.md) for system design, data flow, and contract interaction diagrams.

## Contributing

See [documentation/guides/contributing.md](documentation/guides/contributing.md).

## License

MIT
