# MontRS: Agent-first condensed onboarding

This document is a technical specification for agents, agent-powered IDEs, and automated tools. It defines the core abstractions, invariants, and implementation patterns required to generate idiomatic, compilable MontRS code.

---

### 1. What MontRS Is (Agent Summary)

MontRS is a modular, trait-driven Rust framework for building cross-platform applications. It enforces a strict separation between business logic (Loaders/Actions), state management (Signals), and platform-specific UI rendering. It uses a "Specification-First" approach where application structure is discoverable via metadata.

- **Supported Targets**: Web (WASM), Mobile (iOS/Android via FFI/Bridge), Desktop (Native).
- **Core Design Goals**: Determinism, modularity, agent-discoverability, zero-magic traits.
- **Non-Goals**: Implicit global state, runtime reflection, direct UI-to-DB coupling.

---

### 2. Core Invariants (Non-Negotiable Rules)

- **State Locality**: State must live in `Signals` or `Plates`. Never use global `static mut`.
- **Explicit Mutation**: Mutation only happens inside `Actions` or via explicit `Signal::set()`.
- **Side Effect Isolation**: Side effects (I/O, DB, Network) must be encapsulated in `Plates` and exposed via `Loaders` or `Actions`.
- **Determinism**: Given the same input and state, a `Loader` must return the same output.
- **Router Sovereignty**: The `Router` is the single source of truth for the application's functional surface area.

---

### 3. The Golden Path (Default Behavior)

1. **Define Validator**: Create structs with `#[derive(Validator)]`.
2. **Implement Logic**: Wrap logic in `Loader` (read) or `Action` (write).
3. **Register Route**: Attach logic to a path in a `Plate`.
4. **Expose Metadata**: Implement `description()` and `input_validator()` for agent discovery.

**Flow Diagram:**
`UI (View) -> Action (Mutation) -> Plate (Side Effect) -> DB/Store -> Loader (Read) -> Signal (Update) -> UI (Reactive)`

---

### 4. Routing Rules for Agents

- **Loaders**: Read-only operations. Must implement `Loader<Input, Output>`. Never mutate state.
- **Actions**: Write operations. Must implement `Action<Input, Output>`. Responsible for state changes.
- **Composition**: Routes are registered in `Plate::register_routes(&mut router)`.
- **Constraints**: No business logic in the `main` function. No direct DB calls in the UI layer.

**Example Loader:**
```rust
pub struct UserLoader;
impl Loader<UserId, UserProfile> for UserLoader {
    fn load(&self, input: UserId) -> Result<UserProfile, Error> {
        // Pure read logic
    }
}
```

**Example Action:**
```rust
pub struct UpdateEmailAction;
impl Action<UpdateEmailInput, ()> for UpdateEmailAction {
    fn call(&self, input: UpdateEmailInput) -> Result<(), Error> {
        // Mutation logic + Side effects
    }
}
```

---

### 5. State & Reactivity Rules

- **Signals**: Always use `Signal<T>` for reactive state.
- **Never Implicit**: No hidden observers. State changes must be triggered by explicit calls.
- **Derived State**: Use `DerivedSignal` for values computed from other signals.
- **Never Sync**: Never manually sync state between different signals; use the framework's reactive graph.

---

### 6. Plates & Side Effects

- **Plates**: The unit of composition. They own their own services (DB pools, clients).
- **Side Effects**: Always inject dependencies into Plates during initialization.
- **Uncertainty**: If unsure where logic belongs, put it in a `Plate` service and expose it via an `Action`.

---

### 7. Testing & Determinism

- **TestRuntime**: Use `TestRuntime` to mock environment variables, DB, and time.
- **Predictability**: Avoid `SystemTime::now()` or random numbers without a seedable generator provided by the runtime.
- **Testable Code**: Write logic that accepts traits instead of concrete implementations to allow mocking.

---

### 8. Common Agent Failure Modes

- **Anti-Pattern**: Direct UI-to-Database access.
  - *Fix*: Always go through a `Loader`.
- **Anti-Pattern**: Business logic inside `register_routes`.
  - *Fix*: Keep routes as thin mapping layers.
- **Anti-Pattern**: Using `std::sync::RwLock` for app state.
  - *Fix*: Use `Signal<T>` for reactivity.
- **Anti-Pattern**: Forgetting `#[derive(Validator)]` on input/output types.
  - *Fix*: Agents must ensure all transit types are Validator-ready.

---

### 9. Output Expectations

Before emitting code, the agent must verify:
- [ ] All inputs/outputs derive `Validator`, `Serialize`, and `Deserialize`.
- [ ] No side effects exist outside of `Plates` or `Actions`.
- [ ] `Loaders` are strictly read-only.
- [ ] Routes are registered via `Plate` traits.
- [ ] No global mutable state is introduced.
- [ ] Metadata methods (`description`, `tags`) are implemented for all traits.
