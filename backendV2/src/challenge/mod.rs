use crate::{
    AppState,
    auth::User,
    challenge::domain::{
        create_challenge, delete_challenge, get_challenge_by_id, get_challenges, update_challenge,
    },
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};

mod domain;
mod repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub kind: String,
    pub question: String,
    pub id: String,
    pub number: i32,
    pub question_cluster_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChallenge {
    pub id: String,
    pub name: String,
    pub status: String,
    pub target_media: String,
    pub questions: Vec<Question>,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSharedChallenge {
    pub name: String,
    pub status: String,
    pub target_media: String,
    pub questions: Vec<Question>,
}

fn map_to_internal_error(err: Box<dyn std::error::Error>) -> StatusCode {
    eprintln!("Internal error: {}", err);
    StatusCode::INTERNAL_SERVER_ERROR
}

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
) -> Result<Json<Vec<SharedChallenge>>, StatusCode> {
    match get_challenges(&state) {
        Ok(items) => Ok(Json(items)),
        Err(err) => Err(map_to_internal_error(err)),
    }
}

async fn get_challenge(
    State(state): State<AppState>,
    Path(id): Path<String>,
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
