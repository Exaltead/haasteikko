use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    auth::User,
    event::{
        Event, NewEvent,
        domain::{create_event, delete_event, get_event_by_id, get_events, update_event},
    },
    utils::map_to_internal_error,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/event", get(get_events_route))
        .route("/event", post(create_event_route))
        .route("/event/{id}", get(get_event_by_id_route))
        .route("/event/{id}", put(update_event_route))
        .route("/event/{id}", delete(delete_event_route))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EventQuery {
    item_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct IdResponse {
    id: String,
}

async fn get_events_route(
    user: User,
    state: State<AppState>,
    Query(query): Query<EventQuery>,
) -> Result<Json<Vec<Event>>, StatusCode> {
    let events =
        get_events(&user, &state, query.item_id.as_deref()).map_err(map_to_internal_error)?;
    Ok(Json(events))
}

async fn get_event_by_id_route(
    user: User,
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Event>, StatusCode> {
    let event = get_event_by_id(&user, &state, &id).map_err(map_to_internal_error)?;

    match event {
        Some(event) => Ok(Json(event)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_event_route(
    user: User,
    state: State<AppState>,
    Json(event): Json<NewEvent>,
) -> Result<Json<IdResponse>, StatusCode> {
    let id = create_event(&user, &state, &event).map_err(map_to_internal_error)?;

    Ok(Json(IdResponse { id }))
}

async fn update_event_route(
    user: User,
    state: State<AppState>,
    Path(id): Path<String>,
    Json(event): Json<NewEvent>,
) -> Result<Json<Event>, StatusCode> {
    let success = update_event(&user, &state, &id, &event).map_err(map_to_internal_error)?;

    if !success {
        return Err(StatusCode::NOT_FOUND);
    }

    let event = get_event_by_id(&user, &state, &id).map_err(map_to_internal_error)?;

    match event {
        Some(event) => Ok(Json(event)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_event_route(
    user: User,
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let success = delete_event(&user, &state, &id).map_err(map_to_internal_error)?;

    if !success {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
