use rusqlite::Connection;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum MigrationError {
    Sqlite(rusqlite::Error),
    Io(io::Error),
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::Sqlite(e) => write!(f, "SQLite error: {}", e),
            MigrationError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for MigrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MigrationError::Sqlite(e) => Some(e),
            MigrationError::Io(e) => Some(e),
        }
    }
}

impl From<rusqlite::Error> for MigrationError {
    fn from(err: rusqlite::Error) -> Self {
        MigrationError::Sqlite(err)
    }
}

impl From<io::Error> for MigrationError {
    fn from(err: io::Error) -> Self {
        MigrationError::Io(err)
    }
}

pub struct Migration {
    version: String,
    name: String,
    sql: String,
}

impl Migration {
    fn from_path(path: &Path) -> Result<Option<Self>, io::Error> {
        let file_name = match path.file_name().and_then(|f| f.to_str()) {
            Some(name) => name,
            None => return Ok(None),
        };

        // Check if filename matches our pattern V[DATE]__[NAME].sql
        if !file_name.starts_with('V') || !file_name.ends_with(".sql") {
            return Ok(None);
        }

        let parts: Vec<&str> = file_name.trim_end_matches(".sql").split("__").collect();
        if parts.len() != 2 {
            return Ok(None);
        }

        let version = parts[0].trim_start_matches('V').to_string();
        let name = parts[1].to_string();

        // Read the SQL content
        let content = fs::read_to_string(path)?;
        Ok(Some(Migration {
            version,
            name,
            sql: content,
        }))
    }
}

pub struct Migrator {
    conn: Connection,
    migrations_path: PathBuf,
}

impl Migrator {
    pub fn new<P: AsRef<Path>>(db_path: P, migrations_path: P) -> Result<Self, MigrationError> {
        let conn = Connection::open(db_path)?;

        let migrator = Migrator {
            conn,
            migrations_path: migrations_path.as_ref().to_path_buf(),
        };

        Ok(migrator)
    }

    fn init_changelog(&mut self) -> Result<(), MigrationError> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS changelog (
                version TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at DATETIME NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    fn is_migration_applied(&self, version: &str) -> Result<bool, MigrationError> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM changelog WHERE version = ?")?;
        let count: i64 = stmt.query_row([version], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn run_migrations(&mut self) -> Result<(), MigrationError> {
        // Initialize changelog table
        self.init_changelog()?;

        // Read all migration files
        let mut migrations = Vec::new();
        let entries = fs::read_dir(&self.migrations_path)?;
        for entry in entries.flatten() {
            if let Ok(Some(migration)) = Migration::from_path(&entry.path()) {
                migrations.push(migration);
            }
        }

        // Sort migrations by version
        migrations.sort_by(|a, b| a.version.cmp(&b.version));

        // Execute migrations in order
        for migration in migrations {
            if self.is_migration_applied(&migration.version)? {
                println!("Migration {} already applied, skipping", migration.version);
                continue;
            }

            println!(
                "Applying migration {}: {}",
                migration.version, migration.name
            );

            {
                // Start transaction in its own scope
                let tx = self.conn.transaction()?;

                // Execute migration SQL
                tx.execute_batch(&migration.sql)?;

                // Record successful migration
                tx.execute(
                    "INSERT INTO changelog (version, name, applied_at) VALUES (?, ?, ?)",
                    [
                        &migration.version,
                        &migration.name,
                        &chrono::Utc::now().to_string(),
                    ],
                )?;

                // Commit transaction
                tx.commit()?;
            }

            println!("Successfully applied migration {}", migration.version);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_migration_system() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let migrations_dir = temp_dir.path().join("migrations");
        fs::create_dir(&migrations_dir)?;

        // Create test migration file
        let migration_path = migrations_dir.join("V2025100401__create_test_table.sql");
        let migration_content = "CREATE TABLE test (id INTEGER PRIMARY KEY);";
        let mut file = File::create(&migration_path)?;
        file.write_all(migration_content.as_bytes())?;

        // Initialize migrator
        let mut migrator = Migrator::new(&db_path, &migrations_dir)?;

        // Run migrations
        migrator.run_migrations()?;

        // Verify changelog table exists and contains our migration
        let count: i64 = migrator.conn.query_row(
            "SELECT COUNT(*) FROM changelog WHERE version = '2025100401'",
            [],
            |row| row.get(0),
        )?;

        assert_eq!(count, 1);

        // Verify the migration was actually executed
        let count: i64 = migrator.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='test'",
            [],
            |row| row.get(0),
        )?;

        assert_eq!(count, 1);

        Ok(())
    }
}
