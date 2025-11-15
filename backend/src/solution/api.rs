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
    solution::{
        QuestionSolution, SolutionFilter,
        domain::{get_solutions, upsert_solutions},
    },
    utils::map_to_internal_error,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/solution", get(search_solutions))
        .route("/solution/{challengeId}", post(upsert_solutions_route))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SolutionsQuery {
    challenge_id: Option<String>,
}

//TODO: investiate using a sum type for api
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiQuestionSolution {
    pub id: Option<String>,
    pub question_id: String,
    pub kind: String,
    pub single_answer_item_id: Option<String>,
    pub multiple_answer_item_ids: Option<Vec<String>>,
}

impl ApiQuestionSolution {
    fn from(solution: &QuestionSolution) -> Self {
        ApiQuestionSolution {
            id: Some(solution.id.clone()),
            question_id: solution.question_id.clone(),
            kind: solution.kind.clone(),
            single_answer_item_id: solution.single_answer_item_id.clone(),
            multiple_answer_item_ids: solution.multiple_answer_item_ids.clone(),
        }
    }

    fn to_domain(&self, user_id: &str, challenge_id: &Uuid) -> QuestionSolution {
        QuestionSolution {
            id: self.id.clone().unwrap_or(Uuid::new_v4().to_string()),
            user_id: user_id.to_string(),
            challenge_id: challenge_id.to_string(),
            question_id: self.question_id.clone(),
            kind: self.kind.clone(),
            single_answer_item_id: self.single_answer_item_id.clone(),
            multiple_answer_item_ids: self.multiple_answer_item_ids.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SolutionsList {
    solutions: Vec<ApiQuestionSolution>,
}

#[debug_handler]
async fn search_solutions(
    State(state): State<AppState>,
    user: User,
    Query(query): axum::extract::Query<SolutionsQuery>,
) -> Result<Json<SolutionsList>, StatusCode> {
    let filter = SolutionFilter::new(&user.id);
    let filter = if let Some(challenge_id) = query.challenge_id.as_deref() {
        filter.with_challenge_id(challenge_id)
    } else {
        filter
    };

    let results = get_solutions(&state.database_path, filter)
        .map_err(|err| map_to_internal_error(Box::new(err)))?;

    // convert to api
    let list = SolutionsList {
        solutions: results
            .iter()
            .map(|s| ApiQuestionSolution::from(s))
            .collect(),
    };

    Ok(Json(list))
}

#[debug_handler]
async fn upsert_solutions_route(
    State(state): State<AppState>,
    Path(challenge_id): Path<Uuid>,
    user: User,
    Json(list): Json<SolutionsList>,
) -> Result<Json<SolutionsList>, StatusCode> {
    let domain_solutions: Vec<QuestionSolution> = list
        .solutions
        .iter()
        .map(|s| s.to_domain(&user.id, &challenge_id))
        .collect();

    let res = upsert_solutions(
        &user,
        &state.database_path,
        &challenge_id.to_string(),
        &domain_solutions,
    )
    .map_err(|err| map_to_internal_error(Box::new(err)))?;

    Ok(Json(SolutionsList {
        solutions: res
            .into_iter()
            .map(|s| ApiQuestionSolution::from(&s))
            .collect(),
    }))
}
