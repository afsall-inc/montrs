//! montrs-orm: A trait-driven ORM layer for MontRS.
//! This crate defines the DbBackend trait and provides implementations for
//! SQLite and PostgreSQL, enabling unified database access.
//!
//! // @agent-tool: name="db_query" desc="Executes a SQL query on the configured database backend."

use async_trait::async_trait;
#[cfg(feature = "postgres")]
use deadpool_postgres::{Config, Pool, Runtime};
use montrs_core::AgentError;
#[cfg(feature = "sqlite")]
use rusqlite::Connection;
use thiserror::Error;
#[cfg(feature = "postgres")]
use tokio_postgres::NoTls;

/// Errors that can occur during database operations.
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    Connection(String),
    #[error("Query execution error: {0}")]
    Query(String),
    #[error("Migration error: {0}")]
    Migration(String),
}

impl AgentError for DbError {
    fn error_code(&self) -> &'static str {
        match self {
            DbError::Connection(_) => "DB_CONNECTION",
            DbError::Query(_) => "DB_QUERY",
            DbError::Migration(_) => "DB_MIGRATION",
        }
    }

    fn explanation(&self) -> String {
        match self {
            DbError::Connection(e) => {
                format!("Failed to establish a connection to the database: {}.", e)
            }
            DbError::Query(e) => format!("An error occurred while executing a SQL query: {}.", e),
            DbError::Migration(e) => format!("Database migration failed: {}.", e),
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            DbError::Connection(_) => vec![
                "Verify that the database server is running.".to_string(),
                "Check your connection string and credentials in the environment configuration.".to_string(),
                "Ensure that the network path to the database is accessible.".to_string(),
            ],
            DbError::Query(_) => vec![
                "Check the SQL syntax for errors.".to_string(),
                "Ensure all tables and columns referenced in the query exist.".to_string(),
                "Verify that the parameters passed to the query match the expected types.".to_string(),
            ],
            DbError::Migration(_) => vec![
                "Check for conflicts in migration files.".to_string(),
                "Ensure the database user has sufficient permissions to modify the schema.".to_string(),
                "Verify that the migration scripts are compatible with the target database backend.".to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "orm"
    }
}

/// A unified trait for parameters to support multiple backends.
/// Bridges the gap between rusqlite::ToSql and tokio_postgres::types::ToSql.
pub trait ToSql: Send + Sync {
    /// Returns a reference that can be used by rusqlite.
    #[cfg(feature = "sqlite")]
    fn as_rusqlite(&self) -> &dyn rusqlite::ToSql;
}

// Implementations for common types to be used as query parameters.
impl ToSql for String {
    #[cfg(feature = "sqlite")]
    fn as_rusqlite(&self) -> &dyn rusqlite::ToSql {
        self
    }
}
impl ToSql for i32 {
    #[cfg(feature = "sqlite")]
    fn as_rusqlite(&self) -> &dyn rusqlite::ToSql {
        self
    }
}
impl ToSql for bool {
    #[cfg(feature = "sqlite")]
    fn as_rusqlite(&self) -> &dyn rusqlite::ToSql {
        self
    }
}
impl ToSql for &str {
    #[cfg(feature = "sqlite")]
    fn as_rusqlite(&self) -> &dyn rusqlite::ToSql {
        self
    }
}

/// Trait for mapping database rows to Rust types.
/// Requires backend-specific mapping methods.
pub trait FromRow: Sized {
    /// Maps a rusqlite row to the implementor type.
    #[cfg(feature = "sqlite")]
    fn from_row_sqlite(row: &rusqlite::Row) -> rusqlite::Result<Self>;
    /// Maps a tokio-postgres row to the implementor type.
    #[cfg(feature = "postgres")]
    fn from_row_postgres(row: &tokio_postgres::Row) -> Result<Self, DbError>;
}

/// The core abstraction for database backends.
/// Provides async methods for executing and querying.
#[async_trait]
pub trait DbBackend: Send + Sync + 'static {
    /// Executes a non-query SQL statement (INSERT, UPDATE, DELETE).
    async fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize, DbError>;
    /// Executes a query SQL statement and returns a vector of results.
    async fn query<T: FromRow>(&self, sql: &str, params: &[&dyn ToSql]) -> Result<Vec<T>, DbError>;
}

/// SQLite-specific database backend implementation.
/// Uses synchronous rusqlite under the hood with internal locking.
#[cfg(feature = "sqlite")]
#[derive(Clone)]
pub struct SqliteBackend {
    conn: Arc<Mutex<Connection>>,
}

#[cfg(feature = "sqlite")]
impl SqliteBackend {
    /// Creates a new SqliteBackend connecting to the specified path (or :memory:).
    pub fn new(path: &str) -> Result<Self, DbError> {
        let conn = if path == ":memory:" {
            Connection::open_in_memory()
        } else {
            Connection::open(path)
        }
        .map_err(|e| DbError::Connection(e.to_string()))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[cfg(feature = "sqlite")]
#[async_trait]
impl DbBackend for SqliteBackend {
    async fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize, DbError> {
        let conn = self.conn.lock().unwrap();
        // Convert unified params to rusqlite-compatible params.
        let sqlite_params: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|p| p.as_rusqlite()).collect();
        conn.execute(sql, rusqlite::params_from_iter(sqlite_params))
            .map_err(|e| DbError::Query(e.to_string()))
    }

    async fn query<T: FromRow>(&self, sql: &str, params: &[&dyn ToSql]) -> Result<Vec<T>, DbError> {
        let conn = self.conn.lock().unwrap();
        let sqlite_params: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|p| p.as_rusqlite()).collect();
        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| DbError::Query(e.to_string()))?;
        let rows = stmt
            .query_map(rusqlite::params_from_iter(sqlite_params), |row| {
                T::from_row_sqlite(row)
            })
            .map_err(|e| DbError::Query(e.to_string()))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| DbError::Query(e.to_string()))?);
        }
        Ok(results)
    }
}

/// PostgreSQL-specific database backend implementation.
/// Uses deadpool-postgres for async connection pooling.
#[cfg(feature = "postgres")]
#[derive(Clone)]
pub struct PostgresBackend {
    pool: Pool,
}

#[cfg(feature = "postgres")]
impl PostgresBackend {
    /// Creates a new PostgresBackend with the provided configuration.
    pub fn new(config: Config) -> Result<Self, DbError> {
        let pool = config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| DbError::Connection(e.to_string()))?;
        Ok(Self { pool })
    }
}

#[cfg(feature = "postgres")]
#[async_trait]
impl DbBackend for PostgresBackend {
    async fn execute(&self, sql: &str, _params: &[&dyn ToSql]) -> Result<usize, DbError> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| DbError::Connection(e.to_string()))?;
        // tokio-postgres requires different params handling, implementing skeleton
        client
            .execute(sql, &[])
            .await
            .map(|n| n as usize)
            .map_err(|e| DbError::Query(e.to_string()))
    }

    async fn query<T: FromRow>(
        &self,
        sql: &str,
        _params: &[&dyn ToSql],
    ) -> Result<Vec<T>, DbError> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| DbError::Connection(e.to_string()))?;
        let rows = client
            .query(sql, &[])
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(T::from_row_postgres(&row)?);
        }
        Ok(results)
    }
}
