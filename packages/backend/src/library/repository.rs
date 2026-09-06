use crate::database::{Repository, query_in_transation};
use crate::library::{LibraryFilter, LibraryItem, LibraryRepository};
use rusqlite::{OptionalExtension, Result};

impl Repository<LibraryItem, LibraryFilter> for LibraryRepository {
    fn create(&mut self, item: &LibraryItem) -> Result<String> {
        let sql =
            "INSERT INTO library (id, user_id, kind, title, author, added_at, favorite, translator)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

        let tx = self.transaction()?;

        tx.execute::<&[&dyn rusqlite::ToSql]>(
            &sql,
            &[
                &item.id,
                &item.user_id,
                &item.kind,
                &item.title,
                &item.author,
                &item.added_at,
                &(if item.favorite { 1i64 } else { 0i64 }),
                &item.translator,
            ],
        )?;

        tx.commit()?;

        Ok(item.id.clone())
    }

    fn read_by_id(&mut self, id: &str) -> Result<Option<LibraryItem>> {
        let sql = "SELECT * FROM library WHERE id = ?";
        let connection = self.conn();
        let item = connection
            .query_row(sql, &[&id], row_to_library_item)
            .optional()?;

        Ok(item)
    }

    fn search(&mut self, filter: LibraryFilter) -> Result<Vec<LibraryItem>> {
        let (conditions, params) = to_sql_params(&filter);

        let sql = format!("SELECT * FROM library WHERE {}", conditions);

        let tx = self.transaction()?;
        let items = query_in_transation(&tx, &sql, &params, row_to_library_item)?;
        tx.commit()?;

        Ok(items)
    }

    fn update(&mut self, id: &str, item: &LibraryItem) -> Result<bool> {
        let sql =
            "UPDATE library SET user_id = ?, kind = ?, title = ?, author = ?, favorite = ?, translator = ?
             WHERE id = ?";

        let tx = self.transaction()?;

        let result = tx.execute::<&[&dyn rusqlite::ToSql]>(
            sql,
            &[
                &item.user_id,
                &item.kind,
                &item.title,
                &item.author,
                &(if item.favorite { 1 } else { 0 }),
                &item.translator,
                &id,
            ],
        )?;

        tx.commit()?;
        Ok(result > 0)
    }

    fn delete(&mut self, id: &str) -> Result<bool> {
        let sql = "DELETE FROM library WHERE id = ?";
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

fn row_to_library_item(row: &rusqlite::Row) -> Result<LibraryItem> {
    Ok(LibraryItem {
        id: row.get(0)?,
        user_id: row.get(1)?,
        kind: row.get(2)?,
        title: row.get(3)?,
        author: row.get(4)?,
        added_at: row.get(5)?,
        favorite: row.get::<_, i64>(6)? != 0,
        translator: row.get(7)?,
    })
}

fn to_sql_params(item: &LibraryFilter) -> (String, Vec<&dyn rusqlite::ToSql>) {
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    let mut conditions = Vec::new();

    params.push(&item.user_id);
    conditions.push("user_id = ?");

    if let Some(item_id) = &item.item_id {
        params.push(item_id);
        conditions.push("id = ?");
    }

    let conditions = conditions.join(" AND ");

    (conditions, params)
}
