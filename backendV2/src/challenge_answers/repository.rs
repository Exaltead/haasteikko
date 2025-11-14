use crate::{
    challenge_answers::domain::Answer,
    database::{Database, Repository, query_in_transation},
};
use rusqlite::{OptionalExtension, Result};

use super::domain::AnswerFilter;

pub struct ChallengeAnswerRepository {
    pub db: Database,
}

impl ChallengeAnswerRepository {
    pub fn new(db: Database) -> Self {
        ChallengeAnswerRepository { db }
    }
}

impl Repository<Answer, super::domain::AnswerFilter<'_>> for ChallengeAnswerRepository {
    fn conn(&mut self) -> &mut rusqlite::Connection {
        &mut self.db.conn
    }

    fn create(&mut self, answer: &Answer) -> Result<String> {
        let tx = self.transaction()?;

        let insert_count = tx.execute::<&[&dyn rusqlite::ToSql]>(
            "INSERT INTO answer (id,  question_id, challenge_id, user_id, answered, answer, kind, item_id) 
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &answer.id,
                &answer.question_id,
                &answer.challenge_id,
                &answer.user_id,
                &(if answer.answered { 1i64 } else { 0i64 }),
                &answer.answer,
                &answer.kind,
                &answer.item_id,
            ],
        )?;

        if insert_count == 1 {
            tx.commit()?;
            Ok(answer.id.clone())
        } else {
            Err(rusqlite::Error::ExecuteReturnedResults)
        }
    }

    fn read_by_id(&mut self, id: &str) -> Result<Option<Answer>> {
        let sql = "SELECT id, question_id, challenge_id, user_id, answered, answer, kind, item_id 
            FROM answer 
            WHERE id = ?";
        let connection = self.conn();
        let answer = connection
            .query_row(sql, &[&id], row_to_answer)
            .optional()?;
        Ok(answer)
    }

    fn search(&mut self, filter: AnswerFilter) -> Result<Vec<Answer>> {
        let (conditions, params) = to_sql_params(&filter);
        let sql = format!(
            "SELECT id, question_id, challenge_id, user_id, answered, answer, kind, item_id 
             FROM answer 
             WHERE {}
             ORDER BY id",
            conditions
        );
        let tx = self.transaction()?;
        let items = query_in_transation(&tx, &sql, &params, row_to_answer)?;
        Ok(items)
    }

    fn update(&mut self, id: &str, item: &Answer) -> Result<bool> {
        let sql = "UPDATE answer 
            SET question_id = ?, challenge_id = ?, user_id = ?, answered = ?, answer = ?, kind = ?, item_id = ? 
            WHERE id = ?";

        let tx = self.transaction()?;
        let result = tx.execute::<&[&dyn rusqlite::ToSql]>(
            sql,
            &[
                &item.question_id,
                &item.challenge_id,
                &item.user_id,
                &(if item.answered { 1i64 } else { 0i64 }),
                &item.answer,
                &item.kind,
                &item.item_id,
                &id,
            ],
        )?;

        if result == 1 {
            tx.commit()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn delete(&mut self, id: &str) -> Result<bool> {
        let sql = "DELETE FROM answer WHERE id = ?";
        let tx = self.transaction()?;
        let result = tx.execute(&sql, &[&id])?;
        if result == 1 {
            tx.commit()?;
        }
        Ok(result == 1)
    }
}

fn row_to_answer(row: &rusqlite::Row) -> Result<Answer> {
    Ok(Answer {
        id: row.get(0)?,
        question_id: row.get(1)?,
        challenge_id: row.get(2)?,
        user_id: row.get(3)?,
        answered: row.get::<_, i64>(4)? != 0,
        answer: row.get(5)?,
        kind: row.get(6)?,
        item_id: row.get(7)?,
    })
}

fn to_sql_params<'a>(filter: &'a AnswerFilter) -> (String, Vec<&'a dyn rusqlite::ToSql>) {
    let mut conditions = Vec::new();
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

    params.push(&filter.user_id);
    conditions.push("user_id = ?");

    if let Some(item_id) = &filter.item_id {
        conditions.push("item_id = ?");
        params.push(item_id);
    }

    if let Some(challenge_id) = &filter.challenge_id {
        conditions.push("challenge_id = ?");
        params.push(challenge_id);
    }

    let conditions = conditions.join(" AND ");

    (conditions, params)
}
