# MontRS Architecture: The "Shape" of a Framework

MontRS is built on the principle that a framework should be a **specification** first, and an implementation second. This document outlines the core architectural layers and how they interact.

---

## 🏗️ The Layered Cake

1.  **The Engine (`montrs-core`)**: Defines the foundational traits (`Plate`, `Loader`, `Action`, `AgentError`). It is the "grammar" of the framework.
2.  **The Blueprint (`AppSpec`)**: A serializable representation of the entire application. It knows about every route, every plate, and every data validator.
3.  **The Orchestrator (`montrs-cli`)**: The primary interface for developers. It uses the `AppSpec` to build, serve, and test the app.
4.  **The Sidecar (`montrs-agent`)**: An agent-facing layer that consumes the `AppSpec` and produces machine-optimized context (`agent.json`, `tools.json`).

---

## 🔄 The Lifecycle of a Request

Understanding how a request moves through MontRS is key to building idiomatic apps.

### 1. Discovery & Routing
When a request arrives (or a command is run), the `Router` uses the `AppSpec` to find the matching `Plate` and its associated `Loader` or `Action`.

### 2. The `Context` Object
Every `Loader` and `Action` receives a `Context`. This object is the "glue" that provides access to:
- **Services**: Database pools, cache clients, or external APIs.
- **Runtime**: Environment variables, current time, and configuration.
- **Request Info**: Headers, parameters, and user session (if applicable).

### 3. Execution Flow
1.  **Validation**: `montrs-validator` validates the input before it reaches your logic.
2.  **Logic**: Your `Loader` or `Action` implementation executes.
3.  **Persistence**: The logic interacts with the `Database` via the `Context`.
4.  **Response**: The output is serialized and returned to the caller.

---

## 🧱 Key Architectural Patterns

### Specification-First Discovery
MontRS does not rely on global state or hidden registration. Instead, it uses **Heuristic Discovery**. The CLI scans your `src/` directory for implementations of `Plate`, `Loader`, and `Action`. This ensures that the `AppSpec` is always a true reflection of your code.

### Deterministic Runtimes
In a standard run, the `Context` provides access to real services. In a test run, the `TestRuntime` replaces these with mocks. Because your logic only interacts with traits (via `Context`), it remains unaware of whether it is running in production or a test environment.

---

## 🤖 Agent-first by design

Architecture in MontRS is not just for humans. Every trait implementation is encouraged to provide metadata:

```rust
impl Loader for MyLoader {
    fn description(&self) -> Option<String> {
        Some("Fetches user profile data by ID".to_string())
    }
    
    fn input_validator(&self) -> Option<Validator> {
        Some(UserId::validator())
    }
}
```

This metadata is picked up by `montrs-agent` and exposed to agents, allowing them to understand the *intent* and *contract* of the code without reading the entire implementation.
