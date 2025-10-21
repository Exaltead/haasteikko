use crate::database::{Database, Repository};
use crate::library::{LibraryFilter, LibraryItem};
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;



impl Repository<LibraryItem, LibraryFilter> for Database {
    fn create(&self, item: &LibraryItem) -> Result<String> {
        let sql = 
            "INSERT INTO library (id, user_id, kind, title, author, added_at, completed_at, favorite) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

        let id = Uuid::new_v4().to_string();
        self.execute(
            &sql,
            &[
                &id,
                &item.user_id,
                &item.kind,
                &item.title,
                &item.author,
                &item.added_at,
                &item.completed_at,
                &(if item.favorite { 1i64 } else { 0i64 }),
            ],
        )?;

        Ok(id)
    }

    fn read_by_id(&self, id: &str) -> Result<Option<LibraryItem>> {
        let sql = format!(
            "SELECT l.*, GROUP_CONCAT(aic.challenge_id) as challenge_ids 
            FROM {} l 
            LEFT JOIN activated_item_challenge aic ON l.id = aic.item_id 
            WHERE l.id = ?
            GROUP BY l.id",
            "library"
        );

        let row_map = |row: &rusqlite::Row| -> Result<LibraryItem> {
            let challenge_ids: Option<String> = row.get(8)?;
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
                activated_challenge_ids,
            })
        };
        self.query_row(&sql, &[&id], row_map)
    }

    fn search(&self, filter: LibraryFilter) -> Result<Vec<LibraryItem>> {
        let mut conditions = Vec::new();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        
        conditions.push("user_id = ?");
        params.push(&filter.user_id);
        

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT l.*, GROUP_CONCAT(aic.challenge_id) as challenge_ids 
            FROM {} l
            LEFT JOIN activated_item_challenge aic ON l.id = aic.item_id 
            {} 
            GROUP BY l.id",
            "library", where_clause
        );

        let row_map = |row: &rusqlite::Row| -> Result<LibraryItem> {
            let challenge_ids: Option<String> = row.get(8)?;
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
                activated_challenge_ids,
            })
        };
        self.query_map(&sql, &params, row_map)
    }

    fn update(&self, id: &str, item: &LibraryItem) -> Result<bool> {
        let sql = 
            "UPDATE library SET user_id = ?, kind = ?, title = ?, author = ?, completed_at = ?, favorite = ? 
             WHERE id = ?"    
        ;

        let result = self.execute(
            &sql,
            &[
                &item.user_id,
                &item.kind,
                &item.title,
                &item.author,
                &item.completed_at,
                &(if item.favorite { 1i64 } else { 0i64 }),
                &id,
            ],
        )?;

        Ok(result > 0)
    }

    fn delete(&self, id: &str) -> Result<bool> {
        let sql = format!("DELETE FROM {} WHERE id = ?", "library");
        let result = self.execute(&sql, &[&id])?;
        Ok(result > 0)
    }
}

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
        }
    }

    #[test]
    fn test_crud_operations() -> Result<()> {
        let db = Database::new(":memory:")?;

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
                favorite INTEGER NOT NULL
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

        // Test update
        let mut updated_item = retrieved;
        updated_item.title = "Updated Title".to_string();
        let updated = db.update(&id, &updated_item)?;
        assert!(updated);

        let retrieved = db.read_by_id(&id)?.expect("Item should exist");
        assert_eq!(retrieved.title, "Updated Title");

        // Test delete
        let deleted = db.delete(&id)?;
        assert!(deleted);

        let retrieved = db.read_by_id(&id)?;
        assert!(retrieved.is_none());

        Ok(())
    }
}
