CREATE TYPE contribution_status AS ENUM ('pending', 'confirmed', 'failed', 'missed');

CREATE TABLE contributions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    group_id        UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    member_id       UUID NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    amount          NUMERIC(20, 7) NOT NULL CHECK (amount > 0),
    tx_hash         TEXT NOT NULL UNIQUE,
    status          contribution_status NOT NULL DEFAULT 'pending',
    period_date     TIMESTAMPTZ NOT NULL,
    confirmed_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_contributions_group_id ON contributions (group_id);
CREATE INDEX idx_contributions_member_id ON contributions (member_id);
CREATE INDEX idx_contributions_tx_hash ON contributions (tx_hash);
CREATE INDEX idx_contributions_status ON contributions (status);
CREATE INDEX idx_contributions_period_date ON contributions (period_date);

CREATE TRIGGER contributions_set_updated_at
    BEFORE UPDATE ON contributions
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
