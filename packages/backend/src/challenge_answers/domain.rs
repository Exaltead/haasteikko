use crate::{
    AppState,
    auth::User,
    challenge_answers::repository::ChallengeAnswerRepository,
    database::{Database, Repository},
    library::LibraryRepository,
};
use rusqlite::Result;

#[derive(Debug, Clone)]
pub struct AnswerFilter<'a> {
    pub user_id: &'a str,
    pub item_id: Option<&'a str>,
    pub challenge_id: Option<&'a str>,
    /// Only answers whose item has an event linking the answer's challenge. Answers of unlinked
    /// challenges are kept so relinking restores them, but they don't count toward the challenge.
    pub only_linked: bool,
}

impl AnswerFilter<'_> {
    pub fn new(user_id: &str) -> AnswerFilter<'_> {
        AnswerFilter {
            user_id,
            item_id: None,
            challenge_id: None,
            only_linked: false,
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

    let answers: Vec<Answer> = answer_set
        .iter()
        .map(|answer| Answer {
            answered: answer.is_answered(),
            ..answer.clone()
        })
        .collect();
    repo.upsert_for_item(&answers)?;

    let filter = AnswerFilter::new(&user.id);
    let filter = filter.with_item_id(item_id);
    let filter = filter.with_challenge_id(challenge_id);
    repo.search(filter)
}

pub fn get_challenge_answers(database_path: &str, filter: AnswerFilter) -> Result<Vec<Answer>> {
    let db = Database::new(database_path)?;
    let mut repo = ChallengeAnswerRepository::new(db);

    repo.search(filter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrations::Migrator;
    use std::path::PathBuf;
    use tempfile::{TempDir, tempdir};

    const USER: &str = "user-1";
    const ITEM: &str = "00000000-0000-4000-8000-000000000001";
    const CHALLENGE: &str = "00000000-0000-4000-8000-000000000002";
    const QUESTION: &str = "question-1";

    fn setup() -> (TempDir, AppState) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let migrations_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
        Migrator::new(&db_path, &migrations_path)
            .unwrap()
            .run_migrations()
            .unwrap();

        let db = Database::new(db_path.to_str().unwrap()).unwrap();
        db.conn
            .execute_batch(&format!(
                "INSERT INTO library (id, user_id, kind, title, author, added_at)
                    VALUES ('{ITEM}', '{USER}', 'Book', 'Kirja', 'Kirjailija', '2026-01-01');
                 INSERT INTO challenge (id, name, status, target_media, kind)
                    VALUES ('{CHALLENGE}', 'Haaste', 'active', 'Book', 'Basic');
                 INSERT INTO question (id, challenge_id, kind, question, number, question_cluster_size)
                    VALUES ('{QUESTION}', '{CHALLENGE}', 'Boolean', 'Kysymys?', 1, 1);"
            ))
            .unwrap();

        let state = AppState {
            jwks: jsonwebtoken::jwk::JwkSet { keys: vec![] },
            required_audience: String::new(),
            database_path: db_path.to_str().unwrap().to_string(),
        };
        (dir, state)
    }

    fn link_challenge_via_event(state: &AppState, event_id: &str) {
        let db = Database::new(&state.database_path).unwrap();
        db.conn
            .execute_batch(&format!(
                "INSERT INTO event (id, user_id, item_id, completed_at)
                    VALUES ('{event_id}', '{USER}', '{ITEM}', '2026-01-01');
                 INSERT INTO event_challenge (event_id, challenge_id)
                    VALUES ('{event_id}', '{CHALLENGE}');"
            ))
            .unwrap();
    }

    fn answer(id: &str, value: &str) -> Answer {
        Answer {
            id: id.to_string(),
            question_id: QUESTION.to_string(),
            challenge_id: CHALLENGE.to_string(),
            user_id: USER.to_string(),
            kind: "Boolean".to_string(),
            answered: false,
            answer: value.to_string(),
            item_id: ITEM.to_string(),
        }
    }

    #[test]
    fn upserting_with_a_new_id_updates_the_existing_answer_of_the_question() {
        let (_dir, state) = setup();
        let user = User::new(USER.to_string());

        upsert_answers(&user, &state, ITEM, CHALLENGE, &[answer("a", "no")]).unwrap();
        let result =
            upsert_answers(&user, &state, ITEM, CHALLENGE, &[answer("b", "yes")]).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "a");
        assert_eq!(result[0].answer, "yes");
    }

    #[test]
    fn challenge_wide_search_only_counts_items_linked_by_an_event() {
        let (_dir, state) = setup();
        let user = User::new(USER.to_string());
        upsert_answers(&user, &state, ITEM, CHALLENGE, &[answer("a", "yes")]).unwrap();

        let base = AnswerFilter::new(USER);
        let linked_only = AnswerFilter {
            only_linked: true,
            ..base.with_challenge_id(CHALLENGE)
        };

        let unlinked = get_challenge_answers(&state.database_path, linked_only.clone()).unwrap();
        assert!(unlinked.is_empty());
        let by_item = get_challenge_answers(&state.database_path, base.with_item_id(ITEM)).unwrap();
        assert_eq!(by_item.len(), 1);

        link_challenge_via_event(&state, "event-1");
        link_challenge_via_event(&state, "event-2");

        let linked = get_challenge_answers(&state.database_path, linked_only).unwrap();
        assert_eq!(linked.len(), 1);
    }

    #[test]
    fn another_users_event_on_the_item_does_not_link_its_answers() {
        let (_dir, state) = setup();
        let user = User::new(USER.to_string());
        upsert_answers(&user, &state, ITEM, CHALLENGE, &[answer("a", "yes")]).unwrap();

        let db = Database::new(&state.database_path).unwrap();
        db.conn
            .execute_batch(&format!(
                "INSERT INTO event (id, user_id, item_id, completed_at)
                    VALUES ('foreign-event', 'user-2', '{ITEM}', '2026-01-01');
                 INSERT INTO event_challenge (event_id, challenge_id)
                    VALUES ('foreign-event', '{CHALLENGE}');"
            ))
            .unwrap();

        let base = AnswerFilter::new(USER);
        let linked_only = AnswerFilter {
            only_linked: true,
            ..base.with_challenge_id(CHALLENGE)
        };
        let linked = get_challenge_answers(&state.database_path, linked_only).unwrap();
        assert!(linked.is_empty());
    }

    #[test]
    fn concurrent_upserts_for_a_fresh_item_all_succeed() {
        use std::sync::{Arc, Barrier};

        const THREADS: usize = 8;
        let (_dir, state) = setup();
        let state = Arc::new(state);
        let barrier = Arc::new(Barrier::new(THREADS));

        let handles: Vec<_> = (0..THREADS)
            .map(|i| {
                let (state, barrier) = (state.clone(), barrier.clone());
                std::thread::spawn(move || {
                    let user = User::new(USER.to_string());
                    // Every client generated its own id for the not yet saved answer
                    let answer = answer(&format!("answer-{i}"), &format!("value-{i}"));
                    barrier.wait();
                    upsert_answers(&user, &state, ITEM, CHALLENGE, &[answer])
                        .map_err(|e| e.to_string())
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap().expect("concurrent upsert failed");
        }

        let base = AnswerFilter::new(USER);
        let answers = get_challenge_answers(&state.database_path, base.with_item_id(ITEM)).unwrap();
        assert_eq!(answers.len(), 1);
    }
}
