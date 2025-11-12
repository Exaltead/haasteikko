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
    Router::new().route("/answers", get(search_answers)).route(
        "/answers/{itemId}/{challengeId}",
        post(upsert_answers_route),
    )
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AnswersQuery {
    item_id: Option<String>,
    challenge_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiAnswer {
    pub kind: String,
    pub id: Option<String>,
    pub question_id: String,
    pub answered: bool,
    pub answer: String,
    pub item_id: String,
}

impl ApiAnswer {
    fn from(answer: &Answer) -> Self {
        ApiAnswer {
            id: Some(answer.id.clone()),
            question_id: answer.question_id.clone(),
            kind: answer.kind.clone(),
            answered: answer.answered,
            answer: answer.answer.clone(),
            item_id: answer.item_id.clone(),
        }
    }

    fn to_domain(&self, user_id: &str, item_id: &Uuid, challenge_id: &Uuid) -> Answer {
        Answer {
            id: self.id.clone().unwrap_or(Uuid::new_v4().to_string()),
            question_id: self.question_id.clone(),
            challenge_id: challenge_id.to_string(),
            user_id: user_id.to_string(),
            kind: self.kind.clone(),
            answered: self.answered,
            answer: self.answer.clone(),
            item_id: item_id.to_string(),
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
        challenge_id: query.challenge_id.as_deref(),
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
    Path((item_id, challenge_id)): Path<(Uuid, Uuid)>,
    Json(answer_list): Json<AnswersList>,
) -> Result<Json<AnswersList>, StatusCode> {
    let domain_awnsers: Vec<Answer> = answer_list
        .answers
        .iter()
        .map(|a| a.to_domain(&user.id, &item_id, &challenge_id))
        .collect();
    let answers = upsert_answers(
        &user,
        &state,
        &item_id.to_string(),
        &challenge_id.to_string(),
        &domain_awnsers,
    )
    .map_err(|err| map_to_internal_error(Box::new(err)))?;

    Ok(Json(AnswersList {
        answers: convert_to_api_answers(answers),
    }))
}

fn convert_to_api_answers(answers: Vec<Answer>) -> Vec<ApiAnswer> {
    answers.into_iter().map(|a| ApiAnswer::from(&a)).collect()
}
