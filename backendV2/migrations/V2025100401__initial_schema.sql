CREATE table IF NOT EXISTS library (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    added_at TEXT NOT NULL,
    completed_at TEXT NOT NULL,
    favorite INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS challenge (
    it TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    target_media TEXT NOT NULL,
    kind TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS question (
    id TEXT PRIMARY KEY,
    challenge_id TEXT NOT NULL REFERENCES challenge(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    question TEXT NOT NULL,
    question_cluster_size INTEGER NOT NULL,
    number INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS answer (
    id TEXT PRIMARY KEY,
    question_id TEXT NOT NULL REFERENCES question(id) ON DELETE CASCADE,
    challenge_id TEXT NOT NULL REFERENCES challenge(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    answer TEXT NOT NULL,
    answered INTEGER NOT NULL DEFAULT 0,
    item_id TEXT REFERENCES library(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS question_solution (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    challenge_id TEXT NOT NULL REFERENCES challenge(id) ON DELETE CASCADE,
    question_id TEXT NOT NULL REFERENCES question(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    solution TEXT NOT NULL
);