CREATE TABLE IF NOT EXISTS user_preferences (
    user_id TEXT PRIMARY KEY REFERENCES user(id) ON DELETE CASCADE,
    preferences TEXT NOT NULL DEFAULT '{}'
);
