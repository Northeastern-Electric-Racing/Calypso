-- A test_profile is a named collection of CAN messages.
-- Each can_message row belongs to exactly one profile via profile_id.
CREATE TABLE test_profile (
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE can_message (
    id          INTEGER PRIMARY KEY NOT NULL,
    profile_id  INTEGER NOT NULL REFERENCES test_profile(id) ON DELETE CASCADE,
    can_id      INTEGER NOT NULL,
    is_extended INTEGER NOT NULL DEFAULT 0 CHECK (is_extended IN (0, 1)),
    data        BLOB    NOT NULL CHECK (length(data) <= 8),
    mode        TEXT    NOT NULL CHECK (mode IN ('oneshot', 'broadcast')),
    offset_ms   INTEGER NOT NULL CHECK (offset_ms >= 0),
    period_ms   INTEGER          CHECK (period_ms IS NULL OR period_ms > 0),
    -- broadcast messages must define a period; oneshot messages must not
    CHECK (
        (mode = 'broadcast' AND period_ms IS NOT NULL)
        OR (mode = 'oneshot' AND period_ms IS NULL)
    ),
    -- keep the ID within range for its frame format
    CHECK (
        (is_extended = 0 AND can_id BETWEEN 0 AND 0x7FF)
        OR (is_extended = 1 AND can_id BETWEEN 0 AND 0x1FFFFFFF)
    )
);

CREATE INDEX idx_can_message_profile ON can_message(profile_id);
