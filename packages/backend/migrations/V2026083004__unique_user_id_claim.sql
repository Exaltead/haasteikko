-- One user row per login. Without this, a race could create duplicate users and break their lookups.
CREATE UNIQUE INDEX IF NOT EXISTS user_id_claim ON user (id_claim);
