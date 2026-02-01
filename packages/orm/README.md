# montrs-orm

A trait-driven, async-first ORM layer for MontRS.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-orm` provides a unified, SQL-centric interface for interacting with different database backends (SQLite, PostgreSQL). It abstracts driver-specific details while maintaining high-performance async access.

## 2. What problems it solves
- **Backend Lock-in**: Allows switching between SQLite (local dev) and PostgreSQL (production) with minimal code changes.
- **Boilerplate**: Automates row-to-struct mapping and connection pool management.
- **Type Safety**: Ensures that database queries and results are verified at compile-time or through structured traits.

## 3. What it intentionally does NOT do
- **Query DSL**: It does not force a complex, non-SQL DSL (like Diesel). It prefers clean SQL or simple builder patterns.
- **Migration Management**: It does not handle complex database migrations (use external tools like `sqlx-cli` or `sea-orm-cli` as recommended in our docs).
- **Auto-Caching**: It does not implement implicit query caching.

## 4. How it fits into the MontRS system
It provides the **persistence layer** for `Plate`s. It implements the traits defined in `montrs-core` for data access.

## 5. When a user should reach for this package
- When building a feature that requires persistent data storage.
- When implementing a custom database backend or driver.
- When defining data models that need to be persisted in a relational database.

## 6. Deeper Documentation
- [ORM Architecture](../../docs/orm/index.md)
- [Backend Support Guide](../../docs/orm/backends.md)
- [Async Patterns in ORM](../../docs/orm/async.md)

## 7. Notes for Agents
- **SQL Preference**: Prefer writing raw SQL queries using the provided backend execute methods over complex abstractions.
- **Table Discovery**: Read the struct definitions that implement `FromRow` to understand the database structure.
- **Error Mapping**: This package implements `AgentError` for `DbError`, providing specific codes for constraint violations, connection failures, etc.
- **Constraints**: Assume all database operations are `async` and must be awaited within an `Action` or `Loader`.
