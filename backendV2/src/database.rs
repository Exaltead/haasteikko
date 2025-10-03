use rusqlite::{Connection, Result, Row, params};

/// A trait that must be implemented by any type that wants to be stored in the database
pub trait DbModel: Sized {
    /// Name of the table where this model is stored
    fn table_name() -> String;

    /// SQL for creating the table for this model
    fn create_table_sql() -> String;

    /// Convert a database row into this type
    fn from_row(row: &Row) -> Result<Self>;

    /// Get the values for inserting this instance into the database
    fn to_params(&self) -> Vec<Box<dyn rusqlite::ToSql>>;

    /// Get the names of columns for this model
    fn column_names() -> Vec<String>;
}

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database connection
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.set_db_config(
            rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY,
            true,
        )?;
        Ok(Database { conn })
    }

    /// Initialize a table for a specific model
    pub fn init_table<T: DbModel>(&self) -> Result<()> {
        self.conn.execute(&T::create_table_sql(), [])?;
        Ok(())
    }

    /// Create a new record in the database
    pub fn create<T: DbModel>(&self, model: &T) -> Result<i64> {
        let columns = T::column_names().join(", ");
        let placeholders = vec!["?"; T::column_names().len()].join(", ");
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            T::table_name(),
            columns,
            placeholders
        );

        self.conn
            .execute(&sql, rusqlite::params_from_iter(model.to_params()))?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Read a record by its ID
    pub fn read<T: DbModel>(&self, id: i64) -> Result<Option<T>> {
        let sql = format!("SELECT * FROM {} WHERE id = ?", T::table_name());
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(T::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Read all records of a specific type
    pub fn read_all<T: DbModel>(&self) -> Result<Vec<T>> {
        let sql = format!("SELECT * FROM {}", T::table_name());
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| T::from_row(row))?;

        let mut results = Vec::new();
        for result in rows {
            results.push(result?);
        }
        Ok(results)
    }

    /// Update a record by its ID
    pub fn update<T: DbModel>(&self, id: i64, model: &T) -> Result<bool> {
        let columns = T::column_names()
            .iter()
            .map(|col| format!("{} = ?", col))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!("UPDATE {} SET {} WHERE id = ?", T::table_name(), columns);

        let mut params = model.to_params();
        params.push(Box::new(id));

        let rows_affected = self
            .conn
            .execute(&sql, rusqlite::params_from_iter(params))?;
        Ok(rows_affected > 0)
    }

    /// Delete a record by its ID
    pub fn delete<T: DbModel>(&self, id: i64) -> Result<bool> {
        let sql = format!("DELETE FROM {} WHERE id = ?", T::table_name());
        let rows_affected = self.conn.execute(&sql, params![id])?;
        Ok(rows_affected > 0)
    }
}

// Example of how to use the generic CRUD operations:
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::types::Value;

    // Example model
    #[derive(Debug, PartialEq)]
    struct User {
        id: Option<i64>,
        name: String,
        email: String,
    }

    impl DbModel for User {
        fn table_name() -> String {
            "users".to_string()
        }

        fn create_table_sql() -> String {
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL
            )"
            .to_string()
        }

        fn from_row(row: &Row) -> Result<Self> {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
            })
        }

        fn to_params(&self) -> Vec<Box<dyn rusqlite::ToSql>> {
            vec![Box::new(self.name.clone()), Box::new(self.email.clone())]
        }

        fn column_names() -> Vec<String> {
            vec!["name".to_string(), "email".to_string()]
        }
    }

    #[test]
    fn test_crud_operations() -> Result<()> {
        let db = Database::new(":memory:")?;
        db.init_table::<User>()?;

        // Create
        let user = User {
            id: None,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        let id = db.create(&user)?;

        // Read
        let retrieved_user = db.read::<User>(id)?.unwrap();
        assert_eq!(retrieved_user.name, "John Doe");

        // Update
        let updated_user = User {
            id: Some(id),
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
        };
        db.update::<User>(id, &updated_user)?;

        // Verify update
        let updated = db.read::<User>(id)?.unwrap();
        assert_eq!(updated.name, "Jane Doe");

        // Delete
        assert!(db.delete::<User>(id)?);
        assert!(db.read::<User>(id)?.is_none());

        Ok(())
    }
}
