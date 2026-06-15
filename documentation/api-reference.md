# Ajo Platform — API Reference

Base URL: `http://localhost:8080/api/v1`

All protected routes require `Authorization: Bearer <access_token>`.

Error responses follow:
```json
{ "error": { "status": 404, "message": "Group not found" } }
```

---

## Health

### GET /health

No auth required.

**Response 200**
```json
{
  "status": "healthy",
  "db_ok": true,
  "version": "0.1.0",
  "uptime_secs": 3610
}
```

---

## Auth

### POST /auth/register

**Body**
```json
{
  "email": "alice@example.com",
  "password": "Password1",
  "display_name": "Alice Okonkwo",
  "wallet_address": "GBZXN7PI..."
}
```

**Response 201**
```json
{
  "id": "uuid",
  "email": "alice@example.com",
  "display_name": "Alice Okonkwo",
  "wallet_address": "GBZXN7PI...",
  "is_active": true,
  "created_at": "2025-01-01T00:00:00Z"
}
```

### POST /auth/login

**Body**
```json
{ "email": "alice@example.com", "password": "Password1" }
```

**Response 200**
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "expires_in": 900
}
```

### POST /auth/refresh

**Body**
```json
{ "refresh_token": "eyJ..." }
```

**Response 200** — same shape as login.

---

## Groups

### GET /groups

Query params: `status` (PENDING|ACTIVE|COMPLETED|PAUSED), `cursor` (UUID), `limit` (1-100, default 20).

**Response 200**
```json
{
  "data": [ { ...group } ],
  "meta": { "total": 42, "limit": 20, "next_cursor": "uuid", "has_more": true }
}
```

### POST /groups — Protected

**Body**
```json
{
  "name": "Family Circle",
  "description": "Monthly savings",
  "contribution_amount": "100.0000000",
  "frequency": "monthly",
  "max_members": 5,
  "start_date": "2025-02-01"
}
```

**Response 201** — Group object.

### GET /groups/:id

**Response 200** — Group object.

### POST /groups/:id/join — Protected

No body required.

**Response 201** — Member object.

### GET /groups/:id/members

**Response 200** — Array of MemberWithUser objects.

---

## Contributions

### POST /contributions — Protected

**Body**
```json
{
  "group_id": "uuid",
  "tx_hash": "64-char-hex-hash",
  "amount": "100.0000000"
}
```

**Response 201** — Contribution object.

### GET /groups/:group_id/contributions

Query params: `cursor`, `limit`, `member_id`, `status`.

**Response 200** — Paginated contributions.

---

## Members

### GET /groups/:group_id/me — Protected

Returns the current user's membership in the group.

**Response 200** — Member object.

### DELETE /groups/:group_id/members/:member_id — Protected (creator only)

**Response 200**
```json
{ "message": "Member removed" }
```

---

## Data Models

### Group
```typescript
{
  id: string;
  name: string;
  description: string | null;
  contribution_amount: string;  // decimal string, 7 dp
  frequency: "weekly" | "biweekly" | "monthly";
  max_members: number;
  current_members: number;
  status: "PENDING" | "ACTIVE" | "COMPLETED" | "PAUSED";
  start_date: string;           // YYYY-MM-DD
  creator_id: string;
  contract_group_id: string | null;
  current_payout_position: number;
  created_at: string;
  updated_at: string;
}
```

### Member
```typescript
{
  id: string;
  group_id: string;
  user_id: string;
  payout_position: number;
  status: "pending" | "active" | "removed";
  has_received_payout: boolean;
  joined_at: string;
  updated_at: string;
}
```

### Contribution
```typescript
{
  id: string;
  group_id: string;
  member_id: string;
  amount: string;
  tx_hash: string;
  status: "pending" | "confirmed" | "failed" | "missed";
  period_date: string;
  confirmed_at: string | null;
  created_at: string;
  updated_at: string;
}
```
