# The Golden Path: Building with MontRS

The "Golden Path" represents the most effective, idiomatic way to build applications with MontRS. Following these patterns ensures your app remains deterministic, testable, and agent-friendly.

## 1. Start with the Validator

Before writing any logic, define the data shape. Use `#[derive(Validator)]` for all inputs and outputs.

```rust
#[derive(Validator, Serialize, Deserialize)]
pub struct CreateTodoInput {
    #[validator(min_len = 1)]
    pub title: String,
}
```

## 2. Define a Unified Route

Instead of mixing data fetching and mutations randomly, use the `Route` trait to unify everything related to a URL path.

- **Params**: Define what inputs the URL accepts.
- **Loader**: Fetch side-effect free state for the view.
- **Action**: Handle validation, persistence, and mutation results.
- **View**: Define the visual representation.

## 3. Modularize with Plates

Group related routes into a `Plate`. If the functionality is reusable across projects, package it as a standalone crate.

```rust
impl Plate<AppConfig> for TodoPlate {
    fn register_routes(&self, router: &mut Router<AppConfig>) {
        router.register(TodoListRoute);
        router.register(TodoCreateRoute);
    }
}
```

## 4. Prioritize Determinism in Tests

Use `montrs-test` to write tests that are fast and reliable. Avoid external dependencies in unit tests by using our built-in mocks and fixtures.

## 5. Embrace agent-first metadata

Always provide a `description()` for your loaders and actions. This small effort pays off significantly when an agent needs to understand or modify your code.

## 6. Use the CLI for Orchestration

Rely on `montrs build`, `montrs serve`, and `montrs test` instead of raw `cargo` commands. The CLI handles framework-specific optimizations and context updates.

## 7. Versioned Error Handling

When errors occur, implement `AgentError`. This provides the agent (and you) with structured data to resolve issues quickly.
