CREATE TYPE group_status AS ENUM ('PENDING', 'ACTIVE', 'COMPLETED', 'PAUSED');
CREATE TYPE contribution_frequency AS ENUM ('weekly', 'biweekly', 'monthly');

CREATE TABLE groups (
    id                        UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                      TEXT NOT NULL,
    description               TEXT,
    contribution_amount       NUMERIC(20, 7) NOT NULL CHECK (contribution_amount > 0),
    frequency                 contribution_frequency NOT NULL,
    max_members               INTEGER NOT NULL CHECK (max_members BETWEEN 2 AND 50),
    current_members           INTEGER NOT NULL DEFAULT 0 CHECK (current_members >= 0),
    status                    group_status NOT NULL DEFAULT 'PENDING',
    start_date                DATE NOT NULL,
    creator_id                UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    contract_group_id         TEXT,
    current_payout_position   INTEGER NOT NULL DEFAULT 0,
    created_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_groups_creator_id ON groups (creator_id);
CREATE INDEX idx_groups_status ON groups (status);
CREATE INDEX idx_groups_start_date ON groups (start_date);

CREATE TRIGGER groups_set_updated_at
    BEFORE UPDATE ON groups
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
