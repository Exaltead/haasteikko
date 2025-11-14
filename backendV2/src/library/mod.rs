use crate::{
    AppState,
    auth::User,
    database::Database,
    library::domain::{
        create_library_item, delete_library_item, get_library_item_by_id, get_library_items,
        update_library_item,
    },
    utils::map_to_internal_error,
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
pub struct LibraryFilter {
    pub user_id: String,
    pub item_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    pub id: String,
    pub user_id: String,
    pub kind: String,
    pub title: String,
    pub author: String,
    pub added_at: String,
    pub completed_at: String,
    pub favorite: bool,
    pub activated_challenge_ids: Vec<String>,
    pub translator: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewLibraryItem {
    pub kind: String,
    pub title: String,
    pub author: String,
    pub completed_at: String,
    pub favorite: bool,
    pub activated_challenge_ids: Vec<String>,
    pub translator: Option<String>,
}

pub struct LibraryRepository {
    db: Database,
}

impl LibraryRepository {
    pub fn new(db: Database) -> Self {
        LibraryRepository { db }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct IdResponse {
    id: String,
}

pub fn library_routes() -> Router<AppState> {
    Router::new()
        .route("/library", get(get_library_items_route))
        .route("/library/{id}", get(get_library_item_by_id_route))
        .route("/library", post(create_library_item_route))
        .route("/library/{id}", put(update_library_item_route))
        .route("/library/{id}", delete(delete_library))
}

async fn get_library_items_route(
    user: User,
    state: State<AppState>,
) -> Result<Json<Vec<LibraryItem>>, StatusCode> {
    let items = get_library_items(&user, &state).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(items))
}

async fn get_library_item_by_id_route(
    user: User,
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<LibraryItem>, StatusCode> {
    let item = get_library_item_by_id(&user, &state, &id).map_err(map_to_internal_error)?;

    match item {
        Some(lib) => Ok(Json(lib)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_library_item_route(
    user: User,
    state: State<AppState>,
    Json(item): Json<NewLibraryItem>,
) -> Result<Json<IdResponse>, StatusCode> {
    let id = create_library_item(&user, &state, &item).map_err(map_to_internal_error)?;

    Ok(Json(IdResponse { id: id }))
}

async fn update_library_item_route(
    user: User,
    state: State<AppState>,
    Path(id): Path<String>,
    Json(library): Json<NewLibraryItem>,
) -> Result<Json<LibraryItem>, StatusCode> {
    let success =
        update_library_item(&user, &state, &id, &library).map_err(map_to_internal_error)?;

    if !success {
        return Err(StatusCode::NOT_FOUND);
    }

    let item = get_library_item_by_id(&user, &state, &id).map_err(map_to_internal_error)?;

    match item {
        Some(lib) => Ok(Json(lib)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_library(
    user: User,
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let success = delete_library_item(&user, &state, &id).map_err(map_to_internal_error)?;

    if !success {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
