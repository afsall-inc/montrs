# Agent Guide: montrs-state

## Core Concepts

### 1. Store
The `Store` trait provides `get()`, `set()`, `update()`, and `update_boxed()` for deterministic state management.

- `SimpleStore<T>` — RwLock-backed implementation.
- `StoreContext<T>` — wraps a store for Leptos context integration.
- `use_store()` / `use_store_with_actions()` — Leptos hooks.

### 2. Selectors
`StoreSlice<T>` trait with `select()` for derived state. `FieldSelector` extracts a field via closure.

### 3. Middleware
`Middleware<T>` trait with `on_get()` / `on_set()` hooks. `MiddlewareChain` composes them. Built-in: `LoggerMiddleware`, `ValidationMiddleware`.

### 4. Time Travel
`TimeTravel<T>` provides undo/redo with bounded history. Use `push()`, `undo()`, `redo()`.

### 5. Macros
- `create_store!(Name, StateType, initial)` — creates a named store type.
- `selector!(|s| ...)` — creates a field selector.

### 6. State Machine
- `StateMachine` trait with `initial()` and `transition()`.
- `MachineBuilder` — fluent builder with `.initial()`, `.transition()`, `.build()`.
- `Action<C,E>` — `FunctionAction`, `LogAction`, `AssignAction`.
- `Guard<C,E>` — `FunctionGuard`, `AndGuard`, `OrGuard`, `NotGuard`.
- `MachineHistory<C>` — tracks transition history.

### 7. Leptos Hooks
- `use_machine()` — returns a `MachineHandle<M>` with reactive state.
- `use_machine_with_instance()` — returns `(ReadSignal, Callback)`.
- `use_store_history()` — returns `(ReadSignal, TimeTravel)`.

## Important Rules
- Core is platform-independent; Leptos is optional.
- All transitions are explicit and return structured errors.
- History is bounded by configuration.
- Invalid transitions never panic.