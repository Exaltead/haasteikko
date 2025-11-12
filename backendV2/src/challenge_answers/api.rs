use axum::{
    Json, Router, debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    auth::User,
    challenge_answers::{
        Answer, AnswerFilter,
        domain::{get_challenge_answers, upsert_answers},
    },
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiAnswer {
    pub id: Option<String>,
    pub question_id: String,
    pub challenge_id: String,
    pub kind: String,
    pub answered: bool,
    pub answer: String,
    pub item_id: String,
}

impl ApiAnswer {
    fn from(answer: &Answer) -> Self {
        ApiAnswer {
            id: Some(answer.id.clone()),
            question_id: answer.question_id.clone(),
            challenge_id: answer.challenge_id.clone(),
            kind: answer.kind.clone(),
            answered: answer.answered,
            answer: answer.answer.clone(),
            item_id: answer.item_id.clone(),
        }
    }

    fn to_domain(&self, user_id: &str) -> Answer {
        Answer {
            id: self.id.clone().unwrap_or(Uuid::new_v4().to_string()),
            question_id: self.question_id.clone(),
            challenge_id: self.challenge_id.clone(),
            user_id: user_id.to_string(),
            kind: self.kind.clone(),
            answered: self.answered,
            answer: self.answer.clone(),
            item_id: self.item_id.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AnswersList {
    answers: Vec<ApiAnswer>,
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

    Ok(Json(AnswersList {
        answers: convert_to_api_answers(answers),
    }))
}

#[debug_handler]
async fn upsert_answers_route(
    user: User,
    State(state): State<AppState>,
    Path(item_id): Path<String>,
    Json(answer_list): Json<AnswersList>,
) -> Result<Json<AnswersList>, StatusCode> {
    let domain_awnsers: Vec<Answer> = answer_list
        .answers
        .iter()
        .map(|a| a.to_domain(&user.id))
        .collect();
    let answers = upsert_answers(&user, &state, &item_id, &domain_awnsers)
        .map_err(|err| map_to_internal_error(Box::new(err)))?;

    Ok(Json(AnswersList {
        answers: convert_to_api_answers(answers),
    }))
}

fn convert_to_api_answers(answers: Vec<Answer>) -> Vec<ApiAnswer> {
    answers.into_iter().map(|a| ApiAnswer::from(&a)).collect()
}
