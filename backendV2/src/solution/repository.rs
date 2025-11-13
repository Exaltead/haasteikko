use crate::database::{Database, Repository, query_in_transation};
use rusqlite::{Result, Transaction};

use crate::solution::domain::{QuestionSolution, SolutionFilter};

pub struct SolutionRepository {
    db: Database,
}

impl SolutionRepository {
    pub fn new(db: Database) -> Self {
        SolutionRepository { db }
    }
}

impl Repository<QuestionSolution, SolutionFilter<'_>> for SolutionRepository {
    fn conn(&mut self) -> &mut rusqlite::Connection {
        &mut self.db.conn
    }

    fn create(&mut self, solution: &QuestionSolution) -> Result<String> {
        let tx = self.transaction()?;

        let result = tx.execute(
            "INSERT INTO question_solution (id, user_id, challenge_id, 
                question_id, kind, single_answer_item_id) VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                solution.id,
                solution.user_id,
                solution.challenge_id,
                solution.question_id,
                solution.kind,
                solution.single_answer_item_id,
            ],
        )?;
        if result == 1 {
            update_multipart_solution(&tx, solution)?;
        }

        tx.commit()?;
        Ok(solution.id.clone())
    }

    fn read_by_id(&mut self, _id: &str) -> Result<Option<QuestionSolution>> {
        // Not needed for now
        Ok(None)
    }

    fn search(&mut self, filter: SolutionFilter<'_>) -> Result<Vec<QuestionSolution>> {
        let (where_clause, params) = to_sql_params(&filter);

        let sql = format!(
            "SELECT 
                qs.id, qs.user_id, qs.challenge_id, qs.question_id, 
                qs.kind, qs.single_answer_item_id, GROUP_CONCAT(ms.item_id)
            FROM question_solution qs
            LEFT JOIN multipart_solution ms ON ms.solution_id = qs.id
            WHERE {} 
            GROUP BY qs.id
            ORDER BY qs.id",
            where_clause
        );

        let tx = self.transaction()?;
        let items = query_in_transation(&tx, &sql, &params, row_to_solution)?;

        tx.commit()?;
        Ok(items)
    }

    fn update(&mut self, id: &str, item: &QuestionSolution) -> Result<bool> {
        let tx = self.transaction()?;
        let result = tx.execute(
            "UPDATE question_solution SET user_id = ?, challenge_id = ?, 
                question_id = ?, kind = ?,  = ? WHERE id = ?",
            rusqlite::params![
                &item.user_id,
                &item.challenge_id,
                &item.question_id,
                &item.kind,
                &item.single_answer_item_id,
                &id
            ],
        )?;

        if result != 1 {
            return Ok(false);
        }
        update_multipart_solution(&tx, item)?;

        tx.commit()?;
        Ok(true)
    }

    fn delete(&mut self, id: &str) -> Result<bool> {
        let tx = self.transaction()?;
        let result = tx.execute("DELETE FROM question_solution WHERE id = ?", &[&id])?;
        if result == 1 {
            tx.commit()?;
        }
        Ok(result == 1)
    }
}

fn row_to_solution(row: &rusqlite::Row) -> Result<QuestionSolution> {
    let item_ids: Option<String> = row.get(6)?;
    let multipart_items = item_ids.map(|ids| ids.split(',').map(|x| String::from(x)).collect());
    //.unwrap_or_default();

    Ok(QuestionSolution {
        id: row.get(0)?,
        user_id: row.get(1)?,
        challenge_id: row.get(2)?,
        question_id: row.get(3)?,
        kind: row.get(4)?,
        single_answer_item_id: row.get(5)?,
        multiple_answer_item_ids: multipart_items,
    })
}

fn update_multipart_solution(tx: &Transaction, solution: &QuestionSolution) -> Result<()> {
    if let Some(multipart_solution) = &solution.multiple_answer_item_ids {
        tx.execute(
            "DELETE FROM multipart_solution WHERE solution_id = ?",
            &[&solution.id],
        )?;

        for item_id in multipart_solution {
            tx.execute(
                "INSERT INTO multipart_solution(solution_id, item_id) VALUES (?, ?)",
                &[&solution.id, &item_id],
            )?;
        }
    }
    Ok(())
}

fn to_sql_params<'a>(filter: &'a SolutionFilter) -> (String, Vec<&'a dyn rusqlite::ToSql>) {
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    let mut conditions: Vec<String> = Vec::new();

    conditions.push("user_id = ?".to_string());
    params.push(&filter.user_id);
    if let Some(challenge_id) = &filter.challenge_id {
        conditions.push("challenge_id = ?".to_string());
        params.push(challenge_id);
    }

    let conditions = conditions.join(" AND ");

    (conditions, params)
}
