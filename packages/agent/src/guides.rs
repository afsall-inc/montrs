pub const ARCHITECTURE_GUIDE: &str = r#"
# MontRS Architecture Guide
MontRS is a trait-driven, deterministic web framework built on Leptos 0.8.

## Core Concepts
- **AppSpec**: The single source of truth for the application blueprint.
- **Plate**: A unit of composition (Auth, Blog, etc.) that registers unified routes.
- **Unified Route**: A single struct implementing `Route` that unifies Params, Loader, Action, and View.
- **Loaders**: Read-only, idempotent data fetching.
- **Actions**: State-changing mutations.

## How to build a plate
1. Implement the `Plate` trait.
2. Define explicit dependencies using the `dependencies()` method if your plate requires other plates to be initialized first.
3. Define your `Route` implementation (which includes its `Loader`, `Action`, and `View`).
4. Register the route in `register_routes` using `router.register(MyRoute)`.
"#;

pub const DEBUGGING_GUIDE: &str = r#"
# Debugging MontRS
MontRS provides an `errorfile.json` in the `.agent/errorfiles` folder when a build or test fails.
Agents should use these files to understand the context of the error and propose fixes.
"#;
