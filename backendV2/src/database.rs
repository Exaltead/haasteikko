use rusqlite::{Connection, Result, Row, params};

/// A trait that must be implemented by any type that wants to be stored in the database
/*pub trait DbModel: Sized {
    /// Name of the table where this model is stored
    fn table_name() -> String;

    /// Convert a database row into this type
    fn from_row(row: &Row) -> Result<Self>;

    /// Get the values for inserting this instance into the database
    fn to_params(&self) -> Vec<Box<dyn rusqlite::ToSql>>;

    /// Get the names of columns for this model
    fn column_names() -> Vec<String>;
}*/

pub struct Database {
    pub conn: Connection,
}

pub trait Repository<TType, TFilter> {
    fn create(&mut self, item: &TType) -> Result<String>;
    fn read_by_id(&self, id: &str) -> Result<Option<TType>>;
    fn search(&self, filter: TFilter) -> Result<Vec<TType>>;
    fn update(&mut self, id: &str, item: &TType) -> Result<bool>;
    fn delete(&mut self, id: &str) -> Result<bool>;
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

    /// Execute a prepared statement with parameters
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        self.conn.execute(sql, params)
    }

    /// Query multiple rows with custom SQL
    pub fn query_map<T, F>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
        f: F,
    ) -> Result<Vec<T>>
    where
        F: FnMut(&Row<'_>) -> Result<T>,
    {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params, f)?;

        let mut results = Vec::new();
        for result in rows {
            results.push(result?);
        }
        Ok(results)
    }

    /// Query a single row with custom SQL
    pub fn query_row<T, F>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
        f: F,
    ) -> Result<Option<T>>
    where
        F: FnOnce(&Row<'_>) -> Result<T>,
    {
        match self.conn.query_row(sql, params, f) {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /*
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
    }*/
}
