#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$(dirname "$SCRIPT_DIR")/backend/.env"

if [[ -f "$ENV_FILE" ]]; then
  source "$ENV_FILE"
fi

if [[ -z "${DATABASE_URL:-}" ]]; then
  echo "Error: DATABASE_URL not set. Source backend/.env or export the variable." >&2
  exit 1
fi

echo "=== Seeding Ajo Database ==="
echo "Database: $DATABASE_URL"
echo ""

psql "$DATABASE_URL" <<'SQL'
-- Seed users (passwords are bcrypt of 'Password1')
INSERT INTO users (id, email, password_hash, display_name, wallet_address, is_active)
VALUES
  ('a1b2c3d4-0000-0000-0000-000000000001',
   'alice@example.com',
   '$2b$12$qkiFLYCR62aHxGbB8oeGR.PBA3zPPrai3QkMG5KOMKNbMfNTqNciO',
   'Alice Okonkwo',
   'GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPGOZ3ZZXCJV5ZZKMG47CTMCVP',
   true),
  ('a1b2c3d4-0000-0000-0000-000000000002',
   'bob@example.com',
   '$2b$12$qkiFLYCR62aHxGbB8oeGR.PBA3zPPrai3QkMG5KOMKNbMfNTqNciO',
   'Bob Adeyemi',
   NULL,
   true),
  ('a1b2c3d4-0000-0000-0000-000000000003',
   'carol@example.com',
   '$2b$12$qkiFLYCR62aHxGbB8oeGR.PBA3zPPrai3QkMG5KOMKNbMfNTqNciO',
   'Carol Mensah',
   NULL,
   true)
ON CONFLICT (email) DO NOTHING;

-- Seed a group
INSERT INTO groups (id, name, description, contribution_amount, frequency, max_members, current_members, status, start_date, creator_id, current_payout_position)
VALUES (
  'b2c3d4e5-0000-0000-0000-000000000001',
  'Family Ajo Circle',
  'Monthly savings group for family members',
  100.0000000,
  'monthly',
  5,
  3,
  'ACTIVE',
  CURRENT_DATE + INTERVAL '7 days',
  'a1b2c3d4-0000-0000-0000-000000000001',
  1
)
ON CONFLICT DO NOTHING;

-- Seed members
INSERT INTO members (id, group_id, user_id, payout_position, status)
VALUES
  ('c3d4e5f6-0000-0000-0000-000000000001', 'b2c3d4e5-0000-0000-0000-000000000001', 'a1b2c3d4-0000-0000-0000-000000000001', 1, 'active'),
  ('c3d4e5f6-0000-0000-0000-000000000002', 'b2c3d4e5-0000-0000-0000-000000000001', 'a1b2c3d4-0000-0000-0000-000000000002', 2, 'active'),
  ('c3d4e5f6-0000-0000-0000-000000000003', 'b2c3d4e5-0000-0000-0000-000000000001', 'a1b2c3d4-0000-0000-0000-000000000003', 3, 'active')
ON CONFLICT DO NOTHING;

SELECT 'Seed complete' AS status;
SQL

echo ""
echo "Seed data inserted successfully."
echo "Login with: alice@example.com / Password1"
