# Core Package Invariants

## 1. Responsibility
`montrs-core` is the foundational engine of the framework. It defines the core traits, data structures, and contracts that all other packages must follow.

## 2. Invariants
- **Single Allowed Dependency**: The only `montrs-*` dependency permitted is `montrs-platform` (layer-0, target enum + platform adapter trait). No other `montrs-*` packages may be imported.
- **Trait-Driven Design**: All framework capabilities (Plates, Routes, Config) must be defined via traits to ensure extensibility and agent-readiness.
- **Mandatory Metadata**: Every public trait intended for application use MUST implement `.description()` and `.metadata()` methods.
- **Structured Errors**: All error types must implement the `AgentError` trait, providing stable error codes and suggested fixes.
- **IO Abstraction**: This package must NOT implement concrete IO (DB drivers, file system logic). It only defines the interfaces.
- **Deterministic Casing**: To eliminate agent hallucination, the following casing rules are mandatory:
    - **Component Names**: Must be `PascalCase` (e.g., `<MyComponent />`).
    - **HTML/Component Attributes**: Must be `kebab-case` (e.g., `attr-name="value"`).
    - **Internal Variables**: Should be `snake_case` (standard Rust).
    - **Files/Folders**: Should be `kebab-case`.

## 3. Boundary Definitions
- **In-Scope**: Trait definitions, routing logic, validation primitives, error trait definitions.
- **Out-of-Scope**: UI rendering, DB implementations, CLI commands, agent inference.

## 4. Agent Guidelines
- When adding a new trait, ensure it is documented with `description()` so other agents can discover it.
- Never introduce a dependency on another `montrs-*` package here.
- Use the `validation.rs` primitives for any new data structures.
