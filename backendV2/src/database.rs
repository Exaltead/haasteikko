use rusqlite::{Connection, Result, Row, ToSql, Transaction};

pub struct Database {
    pub conn: Connection,
}

pub trait Repository<TType, TFilter> {
    fn create(&mut self, item: &TType) -> Result<String>;
    fn read_by_id(&mut self, id: &str) -> Result<Option<TType>>;
    fn search(&mut self, filter: TFilter) -> Result<Vec<TType>>;
    fn update(&mut self, id: &str, item: &TType) -> Result<bool>;
    fn delete(&mut self, id: &str) -> Result<bool>;

    fn conn(&mut self) -> &mut rusqlite::Connection;
    fn transaction(&mut self) -> rusqlite::Result<rusqlite::Transaction<'_>> {
        self.conn().transaction()
    }
}

pub fn query_in_transation<T, F>(
    tx: &Transaction,
    query: &str,
    params: &[&dyn ToSql],
    f: F,
) -> Result<Vec<T>>
where
    F: FnMut(&Row<'_>) -> Result<T>,
{
    let mut stmt = tx.prepare(query)?;

    let rows = stmt.query_map(params, f)?.collect::<Result<Vec<T>>>()?;
    Ok(rows)
}

pub fn query_singe_in_transation<T, F>(
    tx: &Transaction,
    query: &str,
    params: &[&dyn ToSql],
    f: F,
) -> Result<Option<T>>
where
    F: FnOnce(&Row<'_>) -> Result<T>,
{
    match tx.query_row(query, params, f) {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
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
}
