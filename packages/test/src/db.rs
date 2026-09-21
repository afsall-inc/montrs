// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Database testing without a database server.
//!
//! Three tools, in increasing order of behaviour:
//!
//! - [`sqlite_memory`] / [`SqliteFixture`] — a real SQL engine in memory. Use
//!   this when you want to exercise actual SQL and typed rows.
//! - [`RecordingDb`] — wraps any [`DbBackend`] and records every statement, so
//!   you can assert *what* your code ran while it delegates to a real backend.
//! - [`MockDb`] — answers `execute` from expectations and records `query`
//!   calls. Because [`DbBackend::query`] is generic over `T: FromRow` (which
//!   carries no `Deserialize` bound), a mock cannot fabricate typed rows; use
//!   [`sqlite_memory`] for row-level assertions.

use crate::integration::Fixture;
use async_trait::async_trait;
use montrs_orm::{DbBackend, DbError, SqliteBackend, ToSql};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// A real SQLite backend backed by an in-memory database.
///
/// Nothing touches the filesystem or the network.
pub fn sqlite_memory() -> Result<SqliteBackend, DbError> {
    SqliteBackend::new(":memory:")
}

/// A [`Fixture`] that sets up a fresh in-memory SQLite backend.
pub struct SqliteFixture;

#[async_trait]
impl Fixture for SqliteFixture {
    type Context = SqliteBackend;

    async fn setup(&self) -> anyhow::Result<Self::Context> {
        Ok(sqlite_memory()?)
    }
}

/// The kind of statement a backend call represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbCallKind {
    /// An `execute` (INSERT/UPDATE/DELETE).
    Execute,
    /// A `query` (SELECT).
    Query,
}

/// A single recorded backend call.
#[derive(Debug, Clone)]
pub struct DbCall {
    /// The SQL text.
    pub sql: String,
    /// The number of bound parameters.
    pub param_count: usize,
    /// Whether this was an execute or a query.
    pub kind: DbCallKind,
}

/// Wraps a [`DbBackend`], recording every call and delegating to the inner one.
pub struct RecordingDb<D: DbBackend> {
    inner: D,
    calls: Arc<Mutex<Vec<DbCall>>>,
}

impl<D: DbBackend> RecordingDb<D> {
    /// Wrap `inner`, starting with an empty recording.
    pub fn new(inner: D) -> Self {
        Self {
            inner,
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Every recorded call, in order.
    pub fn calls(&self) -> Vec<DbCall> {
        self.calls.lock().unwrap().clone()
    }

    /// The number of recorded calls.
    pub fn call_count(&self) -> usize {
        self.calls.lock().unwrap().len()
    }

    /// True if any recorded statement contains `needle`.
    pub fn ran(&self, needle: &str) -> bool {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .any(|c| c.sql.contains(needle))
    }

    /// Forget all recorded calls.
    pub fn clear(&self) {
        self.calls.lock().unwrap().clear();
    }

    fn record(&self, sql: &str, params: usize, kind: DbCallKind) {
        self.calls.lock().unwrap().push(DbCall {
            sql: sql.to_string(),
            param_count: params,
            kind,
        });
    }
}

#[async_trait]
impl<D: DbBackend> DbBackend for RecordingDb<D> {
    async fn execute(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> Result<usize, DbError> {
        self.record(sql, params.len(), DbCallKind::Execute);
        self.inner.execute(sql, params).await
    }

    async fn query<T: montrs_orm::FromRow>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> Result<Vec<T>, DbError> {
        self.record(sql, params.len(), DbCallKind::Query);
        self.inner.query::<T>(sql, params).await
    }
}

/// Answers `execute` from expectations and records every call.
///
/// Rows are not fabricated (see the module docs); use [`sqlite_memory`] when a
/// test needs real results.
#[derive(Default)]
pub struct MockDb {
    calls: Arc<Mutex<Vec<DbCall>>>,
    execute_results: Arc<Mutex<HashMap<String, usize>>>,
}

impl MockDb {
    /// Create an empty mock (every `execute` returns `0`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Make any `execute` whose SQL contains `sql_needle` return `rows`.
    pub fn expect_execute(
        &self,
        sql_needle: impl Into<String>,
        rows: usize,
    ) -> &Self {
        self.execute_results
            .lock()
            .unwrap()
            .insert(sql_needle.into(), rows);
        self
    }

    /// Every recorded call, in order.
    pub fn calls(&self) -> Vec<DbCall> {
        self.calls.lock().unwrap().clone()
    }

    /// True if any recorded statement contains `needle`.
    pub fn ran(&self, needle: &str) -> bool {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .any(|c| c.sql.contains(needle))
    }

    /// The number of recorded calls of a given kind.
    pub fn count_of(&self, kind: DbCallKind) -> usize {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.kind == kind)
            .count()
    }

    fn record(&self, sql: &str, params: usize, kind: DbCallKind) {
        self.calls.lock().unwrap().push(DbCall {
            sql: sql.to_string(),
            param_count: params,
            kind,
        });
    }
}

#[async_trait]
impl DbBackend for MockDb {
    async fn execute(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> Result<usize, DbError> {
        self.record(sql, params.len(), DbCallKind::Execute);
        let results = self.execute_results.lock().unwrap();
        Ok(results
            .iter()
            .find(|(needle, _)| sql.contains(needle.as_str()))
            .map(|(_, rows)| *rows)
            .unwrap_or(0))
    }

    async fn query<T: montrs_orm::FromRow>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> Result<Vec<T>, DbError> {
        self.record(sql, params.len(), DbCallKind::Query);
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use montrs_orm::FromRow;

    struct User;
    impl FromRow for User {
        fn from_row_sqlite(_row: &rusqlite::Row) -> rusqlite::Result<Self> {
            Ok(User)
        }
    }

    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    #[test]
    fn sqlite_memory_runs_real_sql() {
        let db = sqlite_memory().unwrap();
        block_on(async {
            db.execute("CREATE TABLE t (id INTEGER)", &[])
                .await
                .unwrap();
            let n = db
                .execute("INSERT INTO t (id) VALUES (1)", &[])
                .await
                .unwrap();
            assert_eq!(n, 1);
        });
    }

    #[test]
    fn recording_db_records_and_delegates() {
        let db = RecordingDb::new(sqlite_memory().unwrap());
        block_on(async {
            db.execute("CREATE TABLE t (id INTEGER)", &[])
                .await
                .unwrap();
            db.query::<User>("SELECT * FROM t", &[]).await.unwrap();
        });
        assert_eq!(db.call_count(), 2);
        assert!(db.ran("CREATE TABLE"));
        assert!(db.ran("SELECT * FROM t"));
    }

    #[test]
    fn mock_db_answers_execute_expectations() {
        let db = MockDb::new();
        db.expect_execute("INSERT INTO users", 3);
        let rows =
            block_on(db.execute("INSERT INTO users (id) VALUES (1)", &[]))
                .unwrap();
        assert_eq!(rows, 3);
        assert_eq!(db.count_of(DbCallKind::Execute), 1);
        assert!(!db.ran("SELECT"));
    }
}
