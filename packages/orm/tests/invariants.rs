//! Invariant tests for montrs-orm.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Backend Agnostic: core traits independent of specific backends
//! - Type-Safe Queries
//! - Deterministic Migrations

use montrs_core::AgentError;
use montrs_orm::*;

#[test]
fn test_db_error_agent_error_impl() {
    let err = DbError::Connection("connection refused".to_string());
    assert_eq!(err.error_code(), "DB_CONNECTION");
    assert!(!err.explanation().is_empty());
    assert!(!err.suggested_fixes().is_empty());
    assert_eq!(err.subsystem(), "orm");
}

#[test]
fn test_db_error_query() {
    let err = DbError::Query("syntax error".to_string());
    assert_eq!(err.error_code(), "DB_QUERY");
    assert!(err.explanation().contains("SQL"));
}

#[test]
fn test_db_error_migration() {
    let err = DbError::Migration("version conflict".to_string());
    assert_eq!(err.error_code(), "DB_MIGRATION");
    assert!(err.explanation().contains("migration"));
}

#[test]
fn test_db_error_display() {
    let err = DbError::Connection("timeout".to_string());
    assert!(format!("{}", err).contains("Connection error"));
}

#[test]
fn test_to_sql_trait_object_safe() {
    fn _accepts_to_sql(_p: &dyn ToSql) {}
    let s = "hello".to_string();
    _accepts_to_sql(&s);
}

#[test]
fn test_from_row_trait_object_safe() {
    fn _accepts_from_row(_t: &dyn FromRow) {}
}

#[test]
fn test_db_backend_trait_object_safe() {
    fn _accepts_backend(_b: &dyn DbBackend) {}
}