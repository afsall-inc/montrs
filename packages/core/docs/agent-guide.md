# Agent Guide: montrs-core

This guide is designed to help agents understand and use the core building blocks of the MontRS framework.

## Core Concepts

### 1. The Plate Trait
Every MontRS component is a `Plate`. When building an app, you should define your logic within a struct that implements `Plate`.
- **Metadata**: Always provide a descriptive name and metadata for agent discoverability.
- **Dependencies**: Use `dependencies()` to declare other plates this plate requires. This prevents runtime panics and allows `montrs agent check` to verify your architecture.
- **Initialization**: Use `init` for async setup logic.
- **Routing**: Use `register_routes` to attach loaders and actions.

### 2. The Unified Route Trait
- **Consolidation**: A `Route` implementation unifies `Params`, `Loader`, `Action`, and `View`.
- **Registration**: Use `router.register(MyRoute)` inside `Plate::register_routes`.
- **Type Safety**: The `Route` trait ensures that the output of a `Loader` matches what the `View` expects, and that `Actions` have clearly defined input/output validators.

### 3. Loaders and Actions
- **RouteLoader**: Read-only operations that fetch data. They should be deterministic and idempotent.
- **RouteAction**: Write operations that modify state. They should include validation and error handling.
- **Context**: Both receive `RouteContext`, which provides access to the application config and environment.

### 4. Environment and Configuration
- Use `EnvConfig` for type-safe environment variable access.
- `EnvError` provides detailed agent-accessible metadata when variables are missing or invalid.

### 5. Validation
- Use the `Validate` trait for data integrity.
- `ValidationError` is the standard way to report validation failures to the agent for self-correction.

### 6. Local Invariants
Every package in the MontRS workspace contains a `docs/invariants.md` file.
- **Rules of Engagement**: These files define the specific architectural boundaries and "rules" for that package.
- **Mandatory Check**: Before modifying any package, you MUST read its `docs/invariants.md`.
- **Consistency**: Your changes must not violate the invariants stated in these documents. If you introduce a new invariant, you must update the file.

## Agent Usage Patterns

### Generating a New Plate
When asked to create a new feature or logical boundary:
1.  **Use the CLI**: Run `montrs generate plate <name>`.
2.  **Verify**: Ensure the plate file is created in `src/plates/<name>.rs`.
3.  **Register**:
    - Add `pub mod <name>;` to `src/plates/mod.rs`.
    - Register it in `src/main.rs` using `.with_plate(Box::new(<Name>Plate))`.

### Generating a New Route
When asked to add a page or an endpoint:
1.  **Use the CLI**: Run `montrs generate route <path> --plate <plate_name>`.
2.  **Implementation**:
    - The CLI generates a unified `Route` struct bundling `Params`, `Loader`, `Action`, and `View`.
    - Customize the `load` and `act` logic in the generated file.
3.  **Register**:
    - Add `pub mod <route_name>;` to the plate's `routes/mod.rs`.
    - Register it in the plate's `register_routes` method: `router.register(<RouteName>Route);`.

Example of manual implementation (if not using CLI):
```rust
pub struct UserRoute;

impl Route<AppConfig> for UserRoute {
    type Params = UserParams;
    type Loader = UserLoader;
    type Action = UserAction;
    type View = UserView;

    fn path() -> &'static str { "/users/:id" }
    fn loader(&self) -> Self::Loader { UserLoader }
    fn action(&self) -> Self::Action { UserAction }
    fn view(&self) -> Self::View { UserView }
}
```

### Error Recovery
If you encounter an `AgentError`, check its `error_code` and `suggested_fixes`. The `explanation` field provides a structured reason for the failure.
