# montrs-core

The architectural engine of the MontRS framework.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-core` provides the foundational traits and data structures that define the "Shape" of a MontRS application. It is the minimal runtime required to build a deterministic, modular system.

## 2. What problems it solves
- **Architectural Fragmentation**: Provides a unified way to define plates, routes, and configuration.
- **Testing Complexity**: Enables deterministic execution and mocking through trait-driven interfaces.
- **Agent Discoverability**: Implements the base metadata hooks that allow agents to understand the codebase.

## 3. What it intentionally does NOT do
- **Rendering**: It does not handle UI rendering (that's handled by Leptos/UI/renderer packages).
- **IO Operations**: It defines interfaces for DBs and Files but does not implement the drivers (see `montrs-orm`).
- **Build Logic**: It has no knowledge of how the app is compiled (see `montrs-cli`).

## 4. How it fits into the MontRS system
This is the **root dependency**. Every other package in the ecosystem depends on `montrs-core`. It acts as the "contract" between different parts of the framework.

## 5. When a user should reach for this package
- When defining a new `Plate` or `Route`.
- When implementing a custom `AppConfig` or `AgentError`.
- When building a new integration package for MontRS.

## 6. Deeper Documentation
- [Architecture Overview](../../docs/architecture/overview.md)
- [Plates](../../docs/core/plates.md) - The unit of composition.
- [Routing System](../../docs/core/router.md) - The unified Route trait.

## 7. Notes for Agents
- **Core Contract**: All significant framework behaviors are defined via traits in this package.
- **Unified Route**: The `Route` trait combines params, loaders, actions, and views into a single, machine-readable unit.
- **Metadata Hook**: Use the `.description()` and `.metadata()` methods on any `Plate` or `Route` to understand its purpose.
- **Error Handling**: Look for `AgentError` implementations to get structured debugging context.
- **Deterministic**: Assume all core components are deterministic unless explicitly documented otherwise.
