use crate::{
    AppState,
    auth::User,
    database::Database,
    preferences::domain::{get_user_preferences, upsert_user_preferences},
    utils::map_to_internal_error,
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, put},
};
use serde::{Deserialize, Serialize};

mod domain;
mod repository;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub library_year_filter: Option<String>,
    pub library_type_filter: Option<Vec<String>>,
}

pub struct PreferencesRepository {
    db: Database,
}

impl PreferencesRepository {
    pub fn new(db: Database) -> Self {
        PreferencesRepository { db }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/preferences", get(get_preferences_route))
        .route("/preferences", put(update_preferences_route))
}

async fn get_preferences_route(
    user: User,
    state: State<AppState>,
) -> Result<Json<UserPreferences>, StatusCode> {
    let preferences = get_user_preferences(&user, &state).map_err(map_to_internal_error)?;
    Ok(Json(preferences))
}

async fn update_preferences_route(
    user: User,
    state: State<AppState>,
    Json(preferences): Json<UserPreferences>,
) -> Result<Json<UserPreferences>, StatusCode> {
    upsert_user_preferences(&user, &state, &preferences).map_err(map_to_internal_error)?;
    Ok(Json(preferences))
}
