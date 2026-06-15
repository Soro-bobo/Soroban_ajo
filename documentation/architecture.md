# Ajo Platform — Architecture

## Overview

Ajo Platform is a three-tier system: a Next.js 14 frontend, a Rust/Axum backend, and Soroban smart contracts on Stellar. The backend acts as an authoritative off-chain layer for user management, group state synchronization, and transaction verification, while the contracts enforce core rules on-chain.

```
┌─────────────────────────────────────────────────────────────────┐
│  Browser (Next.js 14 App Router)                                │
│  ┌────────────────┐  ┌────────────────┐  ┌──────────────────┐  │
│  │  React Query   │  │   Zustand      │  │  Freighter API   │  │
│  │  (server state)│  │ (client state) │  │  (wallet/sign)   │  │
│  └───────┬────────┘  └────────────────┘  └───────┬──────────┘  │
└──────────┼──────────────────────────────────────┼──────────────┘
           │ HTTPS / Axios (JWT Bearer)            │
           ▼                                       ▼
┌──────────────────────────────────┐  ┌────────────────────────┐
│  Rust Backend (Axum / Tokio)     │  │  Stellar Horizon API   │
│                                  │  │  (tx verify, balance)  │
│  ┌─────────┐  ┌──────────────┐  │  └────────────┬───────────┘
│  │ Routes  │  │  Middleware  │  │               │
│  │  /auth  │  │ JWT Auth     │  │               ▼
│  │ /groups │  │ Rate Limit   │  │  ┌────────────────────────┐
│  │/contrib │  │ Request ID   │  │  │  Soroban RPC           │
│  └────┬────┘  └──────────────┘  │  │  (contract simulation) │
│       │                          │  └────────────┬───────────┘
│  ┌────▼─────────────────────┐   │               │
│  │  Services                │   │               ▼
│  │  auth_service            │   │  ┌────────────────────────┐
│  │  group_service           │   │  │  Soroban Contract      │
│  │  contribution_service    │   │  │  (ajo-contract)        │
│  │  stellar_service         ├───┼──►  create_group          │
│  └──────────────────────────┘   │  │  join_group            │
│                                  │  │  contribute            │
│  ┌──────────────────────────┐   │  │  distribute_payout     │
│  │  PostgreSQL (SQLx)       │   │  └────────────────────────┘
│  │  users / groups / members│   │
│  │  contributions / payouts │   │
│  └──────────────────────────┘   │
└──────────────────────────────────┘
```

## Authentication Flow

1. User POSTs `/auth/register` → password bcrypt-hashed, stored.
2. User POSTs `/auth/login` → receives `{ access_token (15min), refresh_token (7d) }`.
3. Frontend stores tokens in `localStorage` via `authStore`.
4. Axios interceptor attaches `Authorization: Bearer <access_token>` on every request.
5. On 401, interceptor calls `/auth/refresh` with the refresh token.
6. New token pair issued; old refresh token invalidated on next use.

## Group Lifecycle

```
PENDING ──► ACTIVE ──► COMPLETED
              │
              └──► PAUSED
```

- **PENDING**: Group created, waiting for members. Creator auto-joins at position 1.
- **ACTIVE**: Group full or creator manually activates. Contributions begin.
- **COMPLETED**: All members have received their payout.
- **PAUSED**: Admin/creator suspended the group.

## Contribution & Verification

1. Member broadcasts XLM payment on Stellar directly via Freighter.
2. Member submits `tx_hash` + `amount` to `POST /contributions`.
3. Backend calls `stellar_service.verify_transaction(tx_hash)` → Horizon API.
4. If successful, contribution is saved as `confirmed`.
5. Cron job (Tokio task, hourly) marks `pending` contributions older than 48h as `missed`.

## Payout Rotation

Members are assigned integer `payout_position` (1, 2, 3...) at join time.
`groups.current_payout_position` advances after each successful round.
The backend/admin calls `advance_payout()` to record the distribution;
the corresponding Soroban event is emitted on-chain.

## Smart Contract Design

- **Instance storage**: `GroupCounter`, `Admin` — cheap to read, always live.
- **Persistent storage**: `Group(id)`, `Member(group_id, address)` — archived via TTL bumping.
- **No token custody**: The contract records acknowledgments; actual XLM transfers happen via Stellar payment operations, not token contracts.
- **Events on every state change**: `GroupCreated`, `MemberJoined`, `ContributionMade`, `PayoutDistributed`, `GroupStatusChanged`.

## Observability

- Every HTTP request gets a `x-request-id` UUID injected by middleware.
- `request_id` propagated in all `tracing` spans.
- Structured JSON logs: `{ level, timestamp, request_id, method, path, status, latency_ms }`.
- Health endpoint: `GET /api/v1/health` reports `{ status, db_ok, version, uptime_secs }`.

## Security

- Rate limiting: 100 req/min per IP (token bucket, in-memory, cleaned every 5 min).
- CORS: locked to `FRONTEND_URL` env var.
- All SQL via `sqlx` prepared statements.
- `password_hash` never returned in any response or logged.
- JWT secret loaded from env, 64+ char recommended.
