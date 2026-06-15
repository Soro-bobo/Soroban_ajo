CREATE TABLE payout_schedule (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    group_id        UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    member_id       UUID NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    payout_round    INTEGER NOT NULL CHECK (payout_round > 0),
    scheduled_date  DATE NOT NULL,
    paid_at         TIMESTAMPTZ,
    tx_hash         TEXT,
    amount          NUMERIC(20, 7),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT payout_schedule_group_round_unique UNIQUE (group_id, payout_round)
);

CREATE INDEX idx_payout_schedule_group_id ON payout_schedule (group_id);
CREATE INDEX idx_payout_schedule_member_id ON payout_schedule (member_id);
CREATE INDEX idx_payout_schedule_scheduled_date ON payout_schedule (scheduled_date);
CREATE INDEX idx_payout_schedule_paid_at ON payout_schedule (paid_at) WHERE paid_at IS NOT NULL;

CREATE TRIGGER payout_schedule_set_updated_at
    BEFORE UPDATE ON payout_schedule
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
