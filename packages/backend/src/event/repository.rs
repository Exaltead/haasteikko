use crate::database::{Database, Repository, query_in_transation};
use crate::event::Event;
use rusqlite::{OptionalExtension, Result};

pub struct EventFilter {
    pub user_id: String,
    pub item_id: Option<String>,
}

pub struct EventRepository {
    db: Database,
}

impl EventRepository {
    pub fn new(db: Database) -> Self {
        EventRepository { db }
    }
}

impl Repository<Event, EventFilter> for EventRepository {
    fn create(&mut self, item: &Event) -> Result<String> {
        let sql = "INSERT INTO event (id, user_id, item_id, completed_at) VALUES (?, ?, ?, ?)";

        let tx = self.transaction()?;

        tx.execute::<&[&dyn rusqlite::ToSql]>(
            sql,
            &[&item.id, &item.user_id, &item.item_id, &item.completed_at],
        )?;

        for challenge_id in &item.activated_challenge_ids {
            tx.execute(
                "INSERT INTO event_challenge (event_id, challenge_id) VALUES (?, ?)",
                &[&item.id, challenge_id],
            )?;
        }

        tx.commit()?;

        Ok(item.id.clone())
    }

    fn read_by_id(&mut self, id: &str) -> Result<Option<Event>> {
        let sql = "SELECT e.*, GROUP_CONCAT(ec.challenge_id) as challenge_ids
            FROM event e
            LEFT JOIN event_challenge ec ON e.id = ec.event_id
            WHERE e.id = ?
            GROUP BY e.id";
        let connection = self.conn();
        let item = connection
            .query_row(sql, &[&id], row_to_event)
            .optional()?;

        Ok(item)
    }

    fn search(&mut self, filter: EventFilter) -> Result<Vec<Event>> {
        let (conditions, params) = to_sql_params(&filter);

        let sql = format!(
            "SELECT e.*, GROUP_CONCAT(ec.challenge_id) as challenge_ids
            FROM event e
            LEFT JOIN event_challenge ec ON e.id = ec.event_id
            WHERE {}
            GROUP BY e.id",
            conditions
        );

        let tx = self.transaction()?;
        let items = query_in_transation(&tx, &sql, &params, row_to_event)?;
        tx.commit()?;

        Ok(items)
    }

    fn update(&mut self, id: &str, item: &Event) -> Result<bool> {
        let sql = "UPDATE event SET user_id = ?, item_id = ?, completed_at = ? WHERE id = ?";

        let tx = self.transaction()?;

        let result = tx.execute::<&[&dyn rusqlite::ToSql]>(
            sql,
            &[&item.user_id, &item.item_id, &item.completed_at, &id],
        )?;

        if result > 0 {
            tx.execute("DELETE FROM event_challenge WHERE event_id = ?", &[&id])?;

            for challenge_id in &item.activated_challenge_ids {
                tx.execute::<&[&dyn rusqlite::ToSql]>(
                    "INSERT INTO event_challenge (event_id, challenge_id) VALUES (?, ?)",
                    &[&id, &challenge_id],
                )?;
            }
        }

        tx.commit()?;
        Ok(result > 0)
    }

    fn delete(&mut self, id: &str) -> Result<bool> {
        let sql = "DELETE FROM event WHERE id = ?";
        let tx = self.transaction()?;
        let result = tx.execute(sql, &[&id])?;
        if result == 1 {
            tx.commit()?;
            return Ok(true);
        }
        Ok(false)
    }

    fn conn(&mut self) -> &mut rusqlite::Connection {
        &mut self.db.conn
    }
}

fn row_to_event(row: &rusqlite::Row) -> Result<Event> {
    let challenge_ids: Option<String> = row.get(4)?;
    let activated_challenge_ids = challenge_ids
        .map(|ids| ids.split(',').map(String::from).collect())
        .unwrap_or_default();

    Ok(Event {
        id: row.get(0)?,
        user_id: row.get(1)?,
        item_id: row.get(2)?,
        completed_at: row.get(3)?,
        activated_challenge_ids,
    })
}

fn to_sql_params(filter: &EventFilter) -> (String, Vec<&dyn rusqlite::ToSql>) {
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    let mut conditions = Vec::new();

    params.push(&filter.user_id);
    conditions.push("e.user_id = ?");

    if let Some(item_id) = &filter.item_id {
        params.push(item_id);
        conditions.push("e.item_id = ?");
    }

    let conditions = conditions.join(" AND ");

    (conditions, params)
}
