use crate::auth::User;
use crate::database::Database;
use crate::database::Repository as _RepositoryTrait; // bring trait methods into scope for SolutionRepository
use crate::solution::repository::SolutionRepository;
use rusqlite::Result;

// TODO: model with a sum type
#[derive(Debug, Clone)]
pub struct QuestionSolution {
    pub id: String,
    pub user_id: String,
    pub challenge_id: String,
    pub question_id: String,
    pub kind: String,
    pub single_answer_item_id: Option<String>,
    pub multiple_answer_item_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy)]
pub struct SolutionFilter<'a> {
    pub user_id: &'a str,
    pub challenge_id: Option<&'a str>,
}

impl<'a> SolutionFilter<'a> {
    pub fn new(user_id: &'a str) -> SolutionFilter<'a> {
        SolutionFilter {
            user_id: user_id,
            challenge_id: None,
        }
    }

    pub fn with_challenge_id(mut self, challenge_id: &'a str) -> Self {
        self.challenge_id = Some(challenge_id);
        self
    }
}

pub fn get_solutions(database_path: &str, filter: SolutionFilter) -> Result<Vec<QuestionSolution>> {
    let db = Database::new(database_path)?;
    let mut repo = SolutionRepository::new(db);
    repo.search(filter)
}

pub fn upsert_solutions(
    user: &User,
    database_path: &str,
    challenge_id: &str,
    solutions: &[QuestionSolution],
) -> Result<Vec<QuestionSolution>> {
    let db = Database::new(database_path)?;
    let mut repo = SolutionRepository::new(db);

    // Only allow the user to add/update their own solutions
    // For now assume any user can add for themselves (user.id)

    let validated_solutions = solutions
        .iter()
        .map(|s| {
            let single_answer = s.single_answer_item_id.clone().filter(|a| !a.is_empty());
            let multi_answer = s.multiple_answer_item_ids.clone().filter(|a| a.len() > 0);
            println!("Adding {:?}", s);
            QuestionSolution {
                single_answer_item_id: single_answer,
                multiple_answer_item_ids: multi_answer,
                ..s.clone()
            }
        })
        .collect::<Vec<_>>();

    // Read existing solutions for this user and challenge
    let existing = repo.search(SolutionFilter::new(&user.id).with_challenge_id(challenge_id))?;
    let existing_ids: std::collections::HashSet<String> =
        existing.iter().map(|s| s.id.clone()).collect();

    for sol in validated_solutions.iter() {
        if existing_ids.contains(&sol.id) {
            // update existing
            repo.update(&sol.id, sol)?;
        } else {
            // create new
            repo.create(sol)?;
        }
    }

    // Delete old solutions that are not present in the incoming set
    let incoming_ids: std::collections::HashSet<String> =
        solutions.iter().map(|s| s.id.clone()).collect();
    for old in existing.iter() {
        if !incoming_ids.contains(&old.id) {
            repo.delete(&old.id)?;
        }
    }

    // Return current set for the user + challenge
    repo.search(SolutionFilter::new(&user.id).with_challenge_id(challenge_id))
}
