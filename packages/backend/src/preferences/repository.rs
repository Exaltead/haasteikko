use crate::preferences::{PreferencesRepository, UserPreferences};
use rusqlite::{OptionalExtension, Result};

pub trait PreferencesRepositoryTrait {
    fn get_preferences(&mut self, user_id: &str) -> Result<Option<UserPreferences>>;
    fn upsert_preferences(&mut self, user_id: &str, preferences: &UserPreferences) -> Result<()>;
}

impl PreferencesRepositoryTrait for PreferencesRepository {
    fn get_preferences(&mut self, user_id: &str) -> Result<Option<UserPreferences>> {
        let sql = "SELECT preferences FROM user_preferences WHERE user_id = ?";
        let result: Option<String> = self
            .db
            .conn
            .query_row(sql, &[&user_id], |row| row.get(0))
            .optional()?;

        match result {
            Some(json) => {
                let preferences: UserPreferences =
                    serde_json::from_str(&json).unwrap_or_default();
                Ok(Some(preferences))
            }
            None => Ok(None),
        }
    }

    fn upsert_preferences(&mut self, user_id: &str, preferences: &UserPreferences) -> Result<()> {
        let json = serde_json::to_string(preferences).unwrap_or_else(|_| "{}".to_string());
        let sql = "INSERT INTO user_preferences (user_id, preferences) VALUES (?, ?)
                   ON CONFLICT(user_id) DO UPDATE SET preferences = excluded.preferences";
        self.db.conn.execute(sql, rusqlite::params![user_id, json])?;
        Ok(())
    }
}
