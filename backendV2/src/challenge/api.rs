use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    AppState, auth::User, challenge::{NewSharedChallenge, SharedChallenge, domain::{create_challenge, delete_challenge, get_challenge_by_id, get_challenges, update_challenge}}, utils::map_to_internal_error
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/challenge", get(get_all_challenges))
        .route("/challenge", post(create_new_challenge))
        .route("/challenge/{id}", get(get_challenge))
        .route("/challenge/{id}", put(update_existing_challenge))
        .route("/challenge/{id}", delete(delete_existing_challenge))
}

async fn get_all_challenges(
    State(state): State<AppState>,
    _user: User
) -> Result<Json<Vec<SharedChallenge>>, StatusCode> {
    match get_challenges(&state) {
        Ok(items) => Ok(Json(items)),
        Err(err) => Err(map_to_internal_error(err)),
    }
}

async fn get_challenge(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _user: User
) -> Result<Json<SharedChallenge>, StatusCode> {
    match get_challenge_by_id(&state, &id) {
        Ok(Some(item)) => Ok(Json(item)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => Err(map_to_internal_error(err)),
    }
}

async fn create_new_challenge(
    State(state): State<AppState>,
    user: User,
    Json(challenge): Json<NewSharedChallenge>,
) -> Result<Json<String>, StatusCode> {
    match create_challenge(&user, &state, &challenge) {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(map_to_internal_error(err)),
    }
}

async fn update_existing_challenge(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<String>,
    Json(challenge): Json<NewSharedChallenge>,
) -> Result<StatusCode, StatusCode> {
    match update_challenge(&user, &state, &id, &challenge) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            if err.to_string().contains("Not authorized") {
                Err(StatusCode::FORBIDDEN)
            } else if err.to_string().contains("not found") {
                Err(StatusCode::NOT_FOUND)
            } else {
                Err(map_to_internal_error(err))
            }
        }
    }
}

async fn delete_existing_challenge(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match delete_challenge(&user, &state, &id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            if err.to_string().contains("Not authorized") {
                Err(StatusCode::FORBIDDEN)
            } else if err.to_string().contains("not found") {
                Err(StatusCode::NOT_FOUND)
            } else {
                Err(map_to_internal_error(err))
            }
        }
    }
}
