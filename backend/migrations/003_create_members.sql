CREATE TYPE member_status AS ENUM ('pending', 'active', 'removed');

CREATE TABLE members (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    group_id            UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id             UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    payout_position     INTEGER NOT NULL CHECK (payout_position > 0),
    status              member_status NOT NULL DEFAULT 'pending',
    has_received_payout BOOLEAN NOT NULL DEFAULT false,
    joined_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT members_group_user_unique UNIQUE (group_id, user_id),
    CONSTRAINT members_payout_position_unique UNIQUE (group_id, payout_position)
);

CREATE INDEX idx_members_group_id ON members (group_id);
CREATE INDEX idx_members_user_id ON members (user_id);
CREATE INDEX idx_members_group_user ON members (group_id, user_id);

CREATE TRIGGER members_set_updated_at
    BEFORE UPDATE ON members
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
