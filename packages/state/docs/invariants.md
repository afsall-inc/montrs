# State Package Invariants

## Responsibility
`montrs-state` provides deterministic, platform-independent stores and typed state machines.

## Invariants
- The core package must not depend on the DOM, browser APIs, or a renderer.
- State transitions are explicit and deterministic.
- Invalid transitions return structured errors; they must not panic.
- History is bounded by configuration.
- Leptos integration is optional and must not define the core model.

## Out of scope
- Browser DevTools
- JavaScript code generation
- Network clients
- Renderer-specific state
