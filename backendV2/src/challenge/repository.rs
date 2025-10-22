use rusqlite::OptionalExtension;

use crate::challenge::{Question, SharedChallenge};
use crate::database::{Database, Repository};

#[derive(Default)]
pub struct ChallengeFilter;

pub struct ChallengeRepository {
    db: Database,
}

impl ChallengeRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    fn conn(&self) -> &rusqlite::Connection {
        &self.db.conn
    }

    fn transaction(&mut self) -> rusqlite::Result<rusqlite::Transaction<'_>> {
        self.db.conn.transaction()
    }
}

impl Repository<SharedChallenge, ChallengeFilter> for ChallengeRepository {
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

    fn read_by_id(&self, id: &str) -> rusqlite::Result<Option<SharedChallenge>> {
        let mut stmt = self
            .conn()
            .prepare("SELECT id, name, status, target_media, kind FROM challenge WHERE id = ?1")?;

        let challenge = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok(SharedChallenge {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    status: row.get(2)?,
                    target_media: row.get(3)?,
                    kind: row.get(4)?,
                    questions: Vec::new(), // Will be populated below
                })
            })
            .optional()?;

        if let Some(mut challenge) = challenge {
            let mut stmt = self.conn().prepare(
                "SELECT id, kind, question, number, question_cluster_size 
                 FROM question WHERE challenge_id = ?1",
            )?;

            let questions = stmt
                .query_map(rusqlite::params![id], |row| {
                    Ok(Question {
                        id: row.get(0)?,
                        kind: row.get(1)?,
                        question: row.get(2)?,
                        number: row.get(3)?,
                        question_cluster_size: row.get(4)?,
                    })
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;

            challenge.questions = questions;
            Ok(Some(challenge))
        } else {
            Ok(None)
        }
    }

    fn search(&self, _filter: ChallengeFilter) -> rusqlite::Result<Vec<SharedChallenge>> {
        let mut stmt = self
            .conn()
            .prepare("SELECT id, name, status, target_media, kind FROM challenge")?;

        let challenges = stmt
            .query_map([], |row| {
                Ok(SharedChallenge {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    status: row.get(2)?,
                    target_media: row.get(3)?,
                    kind: row.get(4)?,
                    questions: Vec::new(), // Will be populated below
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut result = Vec::new();
        for mut challenge in challenges {
            let mut stmt = self.conn().prepare(
                "SELECT id, kind, question, number, question_cluster_size 
                 FROM question WHERE challenge_id = ?1",
            )?;

            let questions = stmt
                .query_map(rusqlite::params![challenge.id], |row| {
                    Ok(Question {
                        id: row.get(0)?,
                        kind: row.get(1)?,
                        question: row.get(2)?,
                        number: row.get(3)?,
                        question_cluster_size: row.get(4)?,
                    })
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;

            challenge.questions = questions;
            result.push(challenge);
        }

        Ok(result)
    }

    fn update(&mut self, id: &str, challenge: &SharedChallenge) -> rusqlite::Result<bool> {
        let tx = self.transaction()?;

        let rows_affected = tx.execute(
            "UPDATE challenge SET name = ?2, status = ?3, target_media = ?4, kind = ?5 WHERE id = ?1",
            rusqlite::params![id, challenge.name, challenge.status, challenge.target_media, challenge.kind],
        )?;

        // Delete existing questions
        tx.execute(
            "DELETE FROM question WHERE challenge_id = ?1",
            rusqlite::params![id],
        )?;

        // Insert new questions
        for question in &challenge.questions {
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
