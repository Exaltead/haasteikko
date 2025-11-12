use crate::database::{Repository, query_in_transation};
use crate::library::{LibraryFilter, LibraryItem, LibraryRepository};
use rusqlite::{OptionalExtension, Result};




impl Repository<LibraryItem, LibraryFilter> for LibraryRepository {
    fn create(&mut self, item: &LibraryItem) -> Result<String> {
        let sql = 
            "INSERT INTO library (id, user_id, kind, title, author, added_at, completed_at, favorite, translator) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

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
                &item.completed_at,
                &(if item.favorite { 1i64 } else { 0i64 }),
                &item.translator,
            ],
        )?;

        // Insert new challenge associations
        for challenge_id in &item.activated_challenge_ids {
            tx.execute(
                "INSERT INTO activated_item_challenge (item_id, challenge_id) VALUES (?, ?)",
                &[&item.id, &challenge_id],
            )?;
        }

        tx.commit()?;

        Ok(item.id.clone())
    }

    fn read_by_id(&mut self, id: &str) -> Result<Option<LibraryItem>> {
        let sql = "SELECT l.*, GROUP_CONCAT(aic.challenge_id) as challenge_ids 
            FROM library l 
            LEFT JOIN activated_item_challenge aic ON l.id = aic.item_id 
            WHERE l.id = ?
            GROUP BY l.id";
        let connection = self.conn();
        let item = connection
            .query_row(sql, &[&id], row_to_library_item)
            .optional()?;

        Ok(item)
    }

    fn search(&mut self, filter: LibraryFilter) -> Result<Vec<LibraryItem>> {
        let (conditions, params) = to_sql_params(&filter);

        let sql = format!(
            "SELECT l.*, GROUP_CONCAT(aic.challenge_id) as challenge_ids 
            FROM library l
            LEFT JOIN activated_item_challenge aic ON l.id = aic.item_id 
            WHERE {} 
            GROUP BY l.id",
            conditions
        );

        let tx = self.transaction()?;
        let items = query_in_transation(&tx, &sql, &params, row_to_library_item)?;
        tx.commit()?;

        Ok(items)
    }

    fn update(&mut self, id: &str, item: &LibraryItem) -> Result<bool> {
        // Update the main library item
        let sql = 
            "UPDATE library SET user_id = ?, kind = ?, title = ?, author = ?, completed_at = ?, favorite = ?, translator = ? 
             WHERE id = ?";

        let tx = self.transaction()?;

        let result = tx.execute::<&[&dyn rusqlite::ToSql]>(
            sql,
            &[
                &item.user_id,
                &item.kind,
                &item.title,
                &item.author,
                &item.completed_at,
                &(if item.favorite { 1 } else { 0 }),
                &item.translator,
                &id,
            ],
        )?;

        // Only proceed with challenge updates if the item exists
        if result > 0 {
            // Delete existing challenge associations
            tx.execute(
                "DELETE FROM activated_item_challenge WHERE item_id = ?",
                &[&id],
            )?;

            // Insert new challenge associations
            for challenge_id in &item.activated_challenge_ids {
                tx.execute::<&[&dyn rusqlite::ToSql]>(
                    "INSERT INTO activated_item_challenge (item_id, challenge_id) VALUES (?, ?)",
                    &[&id, &challenge_id],
                )?;
            }
        }

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
    let challenge_ids: Option<String> = row.get(9)?;
    let activated_challenge_ids = challenge_ids
        .map(|ids| ids.split(',').map(String::from).collect())
        .unwrap_or_default();

    Ok(LibraryItem {
        id: row.get(0)?,
        user_id: row.get(1)?,
        kind: row.get(2)?,
        title: row.get(3)?,
        author: row.get(4)?,
        added_at: row.get(5)?,
        completed_at: row.get(6)?,
        favorite: row.get::<_, i64>(7)? != 0,
        translator: row.get(8)?,
        activated_challenge_ids,
    })
}

fn to_sql_params(item: &LibraryFilter) -> (String, Vec<&dyn rusqlite::ToSql>) {
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    let mut conditions = Vec::new();

    params.push(&item.user_id);
    conditions.push("user_id = ?");

    if let Some(item_id) = &item.item_id {
        params.push(item_id);
        conditions.push("l.id = ?");
    }

    let conditions = conditions.join(" AND ");

    (conditions, params)
}
/*

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_item() -> LibraryItem {
        LibraryItem {
            id: String::new(),
            user_id: "test_user".to_string(),
            kind: "Book".to_string(),
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            added_at: Utc::now().to_rfc3339(),
            completed_at: Utc::now().to_rfc3339(),
            favorite: true,
            activated_challenge_ids: Vec::new(),
            translator: Some("Test Translator".to_string()),
        }
    }

    #[test]
    fn test_crud_operations() -> Result<()> {
        let mut db = Database::new(":memory:")?;

        // Create test tables
        db.execute(
            "CREATE TABLE library (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                added_at TEXT NOT NULL,
                completed_at TEXT NOT NULL,
                favorite INTEGER NOT NULL,
                translator TEXT
            )",
            &[],
        )?;

        db.execute(
            "CREATE TABLE challenge (
                id TEXT PRIMARY KEY
            )",
            &[],
        )?;

        db.execute(
            "CREATE TABLE activated_item_challenge (
                item_id TEXT NOT NULL REFERENCES library(id) ON DELETE CASCADE,
                challenge_id TEXT NOT NULL REFERENCES challenge(id) ON DELETE CASCADE,
                PRIMARY KEY (item_id, challenge_id)
            )",
            &[],
        )?;

        // Create a test challenge
        let challenge_id = "test_challenge";
        db.execute("INSERT INTO challenge (id) VALUES (?)", &[&challenge_id])?;

        let item = create_test_item();

        // Test create
        let id = db.create(&item)?;
        assert!(!id.is_empty());

        // Test read
        let retrieved = db.read_by_id(&id)?.expect("Item should exist");
        assert_eq!(retrieved.title, item.title);
        assert_eq!(retrieved.author, item.author);
        assert!(retrieved.activated_challenge_ids.is_empty());

        // Add challenge to the item
        db.execute(
            "INSERT INTO activated_item_challenge (item_id, challenge_id) VALUES (?, ?)",
            &[&id, &challenge_id],
        )?;

        // Test search and verify challenge is included
        let filter = LibraryFilter {
            user_id: item.user_id.clone(),
        };
        let items = db.search(filter)?;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, item.title);
        assert_eq!(items[0].activated_challenge_ids, vec![challenge_id]);

        // Test update with challenge modifications
        let mut updated_item = retrieved;
        updated_item.title = "Updated Title".to_string();
        // Add a new challenge ID
        let new_challenge_id = "test_challenge_2";
        db.execute(
            "INSERT INTO challenge (id) VALUES (?)",
            &[&new_challenge_id],
        )?;
        updated_item.activated_challenge_ids = vec![new_challenge_id.to_string()];

        let updated = db.update(&id, &updated_item)?;
        assert!(updated);

        let retrieved = db.read_by_id(&id)?.expect("Item should exist");
        assert_eq!(retrieved.title, "Updated Title");
        assert_eq!(retrieved.activated_challenge_ids, vec![new_challenge_id]);

        // Test delete
        let deleted = db.delete(&id)?;
        assert!(deleted);

        let retrieved = db.read_by_id(&id)?;
        assert!(retrieved.is_none());

        Ok(())
    }
}
*/
