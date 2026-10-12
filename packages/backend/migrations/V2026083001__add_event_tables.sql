CREATE TABLE IF NOT EXISTS event (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    item_id TEXT NOT NULL REFERENCES library(id) ON DELETE CASCADE,
    completed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS event_challenge(
    event_id TEXT NOT NULL REFERENCES event(id) ON DELETE CASCADE,
    challenge_id TEXT NOT NULL REFERENCES challenge(id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, challenge_id)
);
