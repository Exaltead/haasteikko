use rusqlite::Transaction;

use crate::challenge::{Question, SharedChallenge};
use crate::database::{Database, Repository, query_in_transation, query_singe_in_transation};

pub struct ChallengeFilter {
    pub status: Option<String>,
    pub media_type: Option<String>,
}

impl ChallengeFilter {
    pub fn new() -> Self {
        ChallengeFilter {
            status: None,
            media_type: None,
        }
    }
}

pub struct ChallengeRepository {
    db: Database,
}

impl ChallengeRepository {
    pub fn new(db: Database) -> Self {
        ChallengeRepository { db }
    }
}

impl Repository<SharedChallenge, ChallengeFilter> for ChallengeRepository {
    fn conn(&mut self) -> &mut rusqlite::Connection {
        &mut self.db.conn
    }
    fn create(&mut self, challenge: &SharedChallenge) -> rusqlite::Result<String> {
        let tx = self.transaction()?;

        tx.execute(
            "INSERT INTO challenge (id, name, status, target_media, kind) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                challenge.id,
                challenge.name,
                challenge.status,
                challenge.target_media,
                challenge.kind,
            ],
        )?;

        for question in &challenge.questions {
            tx.execute(
                "INSERT INTO question (id, challenge_id, kind, question, number, question_cluster_size) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    question.id,
                    challenge.id,
                    question.kind,
                    question.question,
                    question.number,
                    question.question_cluster_size,
                ],
            )?;
        }

        tx.commit()?;
        Ok(challenge.id.clone())
    }

    fn read_by_id(&mut self, id: &str) -> rusqlite::Result<Option<SharedChallenge>> {
        let transaction = self.transaction()?;

        let query = "SELECT id, name, status, target_media, kind FROM challenge WHERE id = ?1";
        let params = rusqlite::params![id];
        let challenge = query_singe_in_transation(&transaction, query, params, challenge_from_row)?;

        let result = match challenge {
            Some(mut challenge) => {
                let questions = read_questions_for_challenge_id(&transaction, id)?;
                challenge.questions = questions;
                Some(challenge)
            }
            None => None,
        };
        transaction.commit()?;

        Ok(result)
    }

    fn search(&mut self, filter: ChallengeFilter) -> rusqlite::Result<Vec<SharedChallenge>> {
        let transaction = self.transaction()?;
        let (where_clause, params) = to_sql_params(&filter);
        let query = format!(
            "
            SELECT id, name, status, target_media, kind 
            FROM challenge
            WHERE {}",
            where_clause
        );
        //let params: &[&dyn rusqlite::ToSql] = &[];

        let mut challenges =
            query_in_transation(&transaction, &query, &params, challenge_from_row)?;

        for challenge in &mut challenges {
            let questions = read_questions_for_challenge_id(&transaction, &challenge.id)?;
            challenge.questions = questions;
        }

        Ok(challenges)
    }

    fn update(&mut self, id: &str, challenge: &SharedChallenge) -> rusqlite::Result<bool> {
        let tx = self.transaction()?;

        let rows_affected = tx.execute(
            "UPDATE challenge SET name = ?2, status = ?3, target_media = ?4, kind = ?5 
                WHERE id = ?1",
            rusqlite::params![
                id,
                challenge.name,
                challenge.status,
                challenge.target_media,
                challenge.kind
            ],
        )?;

        // Upsert questions: try to update by id+challenge_id, if none updated then insert
        for question in &challenge.questions {
            let updated = tx.execute(
                "UPDATE question SET kind = ?3, question = ?4, number = ?5, question_cluster_size = ?6 WHERE id = ?1 AND challenge_id = ?2",
                rusqlite::params![
                    question.id,
                    id,
                    question.kind,
                    question.question,
                    question.number,
                    question.question_cluster_size,
                ],
            )?;

            if updated == 0 {
                tx.execute(
                    "INSERT INTO question (id, challenge_id, kind, question, number, question_cluster_size) 
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    rusqlite::params![
                        question.id,
                        id,
                        question.kind,
                        question.question,
                        question.number,
                        question.question_cluster_size,
                    ],
                )?;
            }
        }

        tx.commit()?;
        Ok(rows_affected > 0)
    }

    fn delete(&mut self, id: &str) -> rusqlite::Result<bool> {
        let tx = self.transaction()?;

        tx.execute(
            "DELETE FROM question WHERE challenge_id = ?1",
            rusqlite::params![id],
        )?;

        let rows_affected =
            tx.execute("DELETE FROM challenge WHERE id = ?1", rusqlite::params![id])?;

        tx.commit()?;
        Ok(rows_affected > 0)
    }
}

fn challenge_from_row(row: &rusqlite::Row) -> rusqlite::Result<SharedChallenge> {
    Ok(SharedChallenge {
        id: row.get(0)?,
        name: row.get(1)?,
        status: row.get(2)?,
        target_media: row.get(3)?,
        kind: row.get(4)?,
        questions: Vec::new(), // Will be populated separately
    })
}

fn to_sql_params<'a>(filter: &'a ChallengeFilter) -> (String, Vec<&'a dyn rusqlite::ToSql>) {
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    let mut conditions: Vec<String> = Vec::new();

    if let Some(media_type) = &filter.media_type {
        conditions.push("target_media = ?".to_string());
        params.push(media_type);
    }
    if let Some(status) = &filter.status {
        conditions.push("status = ?".to_string());
        params.push(status);
    }

    if conditions.len() == 0 {
        return (" 1 = 1 ".to_string(), Vec::new());
    }

    let conditions = conditions.join(" AND ");

    (conditions, params)
}

fn read_questions_for_challenge_id(
    tx: &Transaction,
    challenge_id: &str,
) -> rusqlite::Result<Vec<Question>> {
    let query = "SELECT id, kind, question, number, question_cluster_size 
                     FROM question WHERE challenge_id = ?1";

    let questions = query_in_transation(&tx, query, rusqlite::params![challenge_id], |row| {
        Ok(Question {
            id: row.get(0)?,
            kind: row.get(1)?,
            question: row.get(2)?,
            number: row.get(3)?,
            question_cluster_size: row.get(4)?,
        })
    })?;

    Ok(questions)
}
