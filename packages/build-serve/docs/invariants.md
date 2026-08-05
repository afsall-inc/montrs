# Build-Serve Package Invariants

## 1. Responsibility
`montrs-build-serve` provides the HTTP dev server for MontRS projects. It serves static files from the site root directory.

## 2. Invariants
- **Static Serving Only**: Serves pre-built files from the site root. Does not trigger builds itself.
- **Trait-Only Dependency**: Depends on `montrs-build-core` for configuration types, not on the concrete pipeline.
- **Lightweight**: Uses `axum` + `tower-http` ServeDir — no SSR logic, no wasm-bindgen.

## 3. Boundary Definitions
- **In-Scope**: Static file serving, dev server configuration, graceful shutdown.
- **Out-of-Scope**: Build orchestration, file watching, wasm-bindgen, cargo invocations.

## 4. Agent Guidelines
- `serve_static` is the main entry point. `serve_with_callback` is for CLI integration where readiness signaling is needed.