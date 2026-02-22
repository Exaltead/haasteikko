use crate::{
    AppState,
    auth::User,
    database::Database,
    preferences::{PreferencesRepository, UserPreferences, repository::PreferencesRepositoryTrait},
};

pub fn get_user_preferences(
    user: &User,
    state: &AppState,
) -> Result<UserPreferences, Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = PreferencesRepository::new(db);
    let preferences = repo.get_preferences(&user.id)?;
    Ok(preferences.unwrap_or_default())
}

pub fn upsert_user_preferences(
    user: &User,
    state: &AppState,
    preferences: &UserPreferences,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new(&state.database_path)?;
    let mut repo = PreferencesRepository::new(db);
    repo.upsert_preferences(&user.id, preferences)?;
    Ok(())
}
