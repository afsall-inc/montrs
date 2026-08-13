# montrs-log

Structured log store for MontRS services.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-log` captures, stores, streams, and prunes service logs. It supports text, JSON, and logfmt formats with retention policies and archiving.

## 2. What problems it solves
- **Log persistence**: Service output is captured to per-service files.
- **Querying**: Filter by service, level, limit, and offset.
- **Retention**: Line-count and age-based trimming with optional archive rotation.

## 3. What it intentionally does NOT do
- **Log shipping**: No remote aggregation (that's a collector's job).
- **Structured analytics**: It stores and queries, not aggregates.

## 4. How it fits into the MontRS system
Layer 2, no dependencies on other MontRS packages. Used by `montrs-services` for daemon output and by `montrs services logs`.

## 5. When a user should reach for this package
- Viewing service logs.
- Building a log viewer dashboard.
- Implementing log retention/rotation for a service.

## 6. Quick start
```rust
let store = LogStore::default()?;
store.append("api", "info", "listening on :3000").await?;
let entries = store.query(LogQuery {
    service: Some("api".into()), limit: 50, ..Default::default()
}).await?;
```

## 7. Deeper Documentation
- [Invariants](docs/invariants.md)
- [Store](src/store.rs)
- [Format](src/format.rs)