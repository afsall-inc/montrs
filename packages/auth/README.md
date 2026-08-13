# montrs-auth

Comprehensive authentication system for MontRS applications.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-auth` provides a plugin-based authentication framework with core always-on routes and 30+ optional plugins. It covers email/password, OAuth, 2FA, sessions, organizations, API keys, and more — all in pure Rust.

## 2. What problems it solves
- **Auth complexity**: One framework for sign-up, sign-in, sessions, password reset, email verification.
- **Modularity**: Enable only the features you need via `AuthPlugin` implementations.
- **Storage agnosticism**: `DatabaseAdapter` trait with an in-memory adapter for dev, swap in PostgreSQL/MySQL for production.

## 3. What it intentionally does NOT do
- **UI**: It serves JSON APIs only; UI components are in `montrs-ui`.
- **Rendering**: No Leptos bindings (use `montrs-i18n` and `montrs-ui` for that).

## 4. How it fits into the MontRS system
Sits at Layer 2, depending on `montrs-orm`, `montrs-sigstore`, and optionally `montrs-i18n`. Its routes mount into an axum router via `MontrsAuth::axum_router()`.

## 5. When a user should reach for this package
- Adding sign-up / sign-in to a MontRS app.
- Implementing OAuth with GitHub, Google, Apple, Discord, or 30+ other providers.
- Building multi-tenant organizations with RBAC.

## 6. Quick start
```rust
let auth = MontrsAuth::builder()
    .config(AuthConfig::new("your-secret").base_url("http://localhost:3000"))
    .database(Box::new(MemoryDatabaseAdapter::new()))
    .plugin(Box::new(plugins::TwoFactorPlugin::new()))
    .build().await?;
let router = auth.axum_router();
```

## 7. Deeper Documentation
- [Invariants](docs/invariants.md)
- [Core routes](src/core/mod.rs)
- [Plugins](src/plugins/mod.rs)