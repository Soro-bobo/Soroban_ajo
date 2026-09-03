ALTER TABLE payout_schedule
    ADD COLUMN reminder_sent_at TIMESTAMPTZ;

CREATE INDEX idx_payout_schedule_reminder_pending
    ON payout_schedule (scheduled_date)
    WHERE paid_at IS NULL AND reminder_sent_at IS NULL;
