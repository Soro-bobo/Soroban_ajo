# Getting Started

This guide walks you from zero to a running local Ajo Platform instance.

## 1. Prerequisites

Install these before starting:

| Tool | Install |
|------|---------|
| Rust (stable) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` |
| wasm32 target | `rustup target add wasm32-unknown-unknown` |
| soroban-cli | `cargo install --locked soroban-cli@20.0.0` |
| Node.js ≥ 20 | https://nodejs.org |
| pnpm | `npm i -g pnpm` |
| PostgreSQL 15+ | https://www.postgresql.org/download/ |

## 2. Clone & Install

```bash
git clone https://github.com/your-org/ajo-platform.git
cd ajo-platform
pnpm install
```

## 3. Database Setup

```bash
createdb ajo_db
createuser ajo_user -P   # set password to ajo_pass (or update .env)
psql -c "GRANT ALL ON DATABASE ajo_db TO ajo_user;"
```

## 4. Configure Environment

```bash
cp backend/.env.example backend/.env
# Edit DATABASE_URL, JWT_SECRET, CONTRACT_ID
# CONTRACT_ID can be any placeholder until you deploy

cp frontend/.env.example frontend/.env.local
# Edit NEXT_PUBLIC_API_URL, NEXTAUTH_SECRET
```

## 5. Run Migrations

```bash
cd backend
cargo install sqlx-cli --no-default-features --features postgres,rustls
sqlx migrate run
cd ..
```

## 6. Seed Data (Optional)

```bash
bash scripts/seed_db.sh
# Creates 3 users: alice@example.com, bob@example.com, carol@example.com
# Password for all: Password1
```

## 7. Start the Backend

```bash
cd backend
cargo run
# Listening on http://localhost:8080
```

In a separate terminal:
```bash
curl http://localhost:8080/api/v1/health
# {"status":"healthy","db_ok":true,"version":"0.1.0","uptime_secs":5}
```

## 8. Start the Frontend

```bash
pnpm dev:frontend
# http://localhost:3000
```

## 9. Deploy Contract (Testnet)

```bash
bash scripts/generate_keypair.sh
# Fund the deployer address via Friendbot

bash scripts/deploy_testnet.sh
# Outputs CONTRACT_ID — copy to both .env files
```

## 10. Connect Freighter

1. Install the [Freighter browser extension](https://www.freighter.app/).
2. Switch to Testnet in Freighter settings.
3. Visit `http://localhost:3000/wallet` and click "Connect Wallet".

## Next Steps

- Read [architecture.md](../architecture.md) to understand the system design.
- Review the [API Reference](../api-reference.md).
- See [contributing.md](contributing.md) to submit improvements.
