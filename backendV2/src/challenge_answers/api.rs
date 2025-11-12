use axum::{
    Json, Router, debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    auth::User,
    challenge_answers::domain::{Answer, AnswerFilter, get_challenge_answers, upsert_answers},
    utils::map_to_internal_error,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/anwers", get(search_answers))
        .route("/answers/{itemId}", post(upsert_answers_route))
}

#[derive(Deserialize, Debug)]
struct AnswersQuery {
    item_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnswersList {
    answers: Vec<Answer>,
}

#[debug_handler]
async fn search_answers(
    State(state): State<AppState>,
    Query(query): Query<AnswersQuery>,
    user: User,
) -> Result<Json<AnswersList>, StatusCode> {
    let filter = AnswerFilter {
        user_id: &user.id,
        item_id: query.item_id.as_deref(),
    };
    let answers = get_challenge_answers(&state.database_path, filter)
        .map_err(|err| map_to_internal_error(Box::new(err)))?;

    Ok(Json(AnswersList { answers }))
}

#[debug_handler]
async fn upsert_answers_route(
    user: User,
    State(state): State<AppState>,
    Path(item_id): Path<String>,
    Json(answer_list): Json<AnswersList>,
) -> Result<Json<AnswersList>, StatusCode> {
    let answers = upsert_answers(&user, &state, &item_id, &answer_list.answers)
        .map_err(|err| map_to_internal_error(Box::new(err)))?;

    Ok(Json(AnswersList { answers }))
}
