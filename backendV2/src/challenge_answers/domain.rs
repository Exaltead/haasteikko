use crate::{
    AppState,
    auth::User,
    challenge_answers::repository::ChallengeAnswerRepository,
    database::{Database, Repository},
    library::LibraryRepository,
};
use rusqlite::Result;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct AnswerFilter<'a> {
    pub user_id: &'a str,
    pub item_id: Option<&'a str>,
    pub challenge_id: Option<&'a str>,
}

impl AnswerFilter<'_> {
    pub fn new(user_id: &str) -> AnswerFilter<'_> {
        AnswerFilter {
            user_id,
            item_id: None,
            challenge_id: None,
        }
    }

    pub fn with_item_id<'a>(&'a self, item_id: &'a str) -> AnswerFilter<'a> {
        let new = AnswerFilter {
            item_id: Some(item_id),
            ..self.clone()
        };
        new
    }
    pub fn with_challenge_id<'a>(&'a self, challenge_id: &'a str) -> AnswerFilter<'a> {
        let new = AnswerFilter {
            challenge_id: Some(challenge_id),
            ..self.clone()
        };
        new
    }
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub id: String,
    pub question_id: String,
    pub challenge_id: String,
    pub user_id: String,
    pub kind: String,
    pub answered: bool,
    pub answer: String,
    pub item_id: String,
}

impl Answer {
    fn is_answered(&self) -> bool {
        match self.kind.as_str() {
            "Boolean" => true,
            "TextInput" => !self.answer.trim().is_empty(),
            _ => false,
        }
    }
}

pub fn upsert_answers(
    user: &User,
    state: &AppState,
    item_id: &str,
    challenge_id: &str,
    answer_set: &[Answer],
) -> Result<Vec<Answer>> {
    // These should most likely use the same db
    let db = Database::new(&state.database_path)?;
    let mut repo = ChallengeAnswerRepository::new(db);
    let mut library_repo = LibraryRepository::new(Database::new(&state.database_path)?);
    if let Some(item) = library_repo.read_by_id(item_id)? {
        if item.user_id != user.id {
            return Err(rusqlite::Error::InvalidQuery); // Unauthorized
        }
    } else {
        return Err(rusqlite::Error::QueryReturnedNoRows); // Item not found
    }

    let current_answers = repo.search(
        AnswerFilter::new(&user.id)
            .with_item_id(&item_id)
            .with_challenge_id(&challenge_id),
    )?;

    let current_answer_ids: HashSet<String> = current_answers.into_iter().map(|a| a.id).collect();

    for answer in answer_set
        .iter()
        .filter(|a| !current_answer_ids.contains(&a.id))
    {
        let answer = Answer {
            answered: answer.is_answered(),
            ..answer.clone()
        };
        repo.create(&answer)?;
    }

    for answer in answer_set
        .iter()
        .filter(|a| current_answer_ids.contains(&a.id))
    {
        let answer = Answer {
            answered: answer.is_answered(),
            ..answer.clone()
        };
        repo.update(&answer.id, &answer)?;
    }

    let result = repo.search(AnswerFilter::new(&user.id).with_item_id(&item_id))?;
    Ok(result)
}

pub fn get_challenge_answers(database_path: &str, filter: AnswerFilter) -> Result<Vec<Answer>> {
    let db = Database::new(database_path)?;
    let mut repo = ChallengeAnswerRepository::new(db);

    repo.search(filter)
}
