# montrs-runtime — Invariants

## 1. Responsibility
Provide a general-purpose Rust runtime: ops, extensions, resource table, event loop, module loading, and memory primitives.

## 2. Invariants
- **No JS/V8**: This is a pure Rust runtime. No JavaScript engine, no Node.js.
- **TypeMap-based OpState**: Extension state is stored in a `TypeMap` (one value per type).
- **Extension-driven**: All functionality is added via `RuntimeExtension` — the core has no built-in ops.
- **Typed resources**: `ResourceTable` stores `Box<dyn Resource>` keyed by `ResourceId`.
- **Tokio event loop**: All async tasks run on tokio's runtime.
- **Memory primitives only**: `Arena` (bump allocation), `TaggedValue` (NaN-boxed u64), `BitField` (packed fields). No garbage collector.

## 3. Boundary
- **In-Scope**: Runtime struct, extensions, ops, resources, event loop, module loader, memory primitives.
- **Out-of-Scope**: Application logic, routing, ORM, auth, web server.

## 4. Agent Guidelines
- Use `MontrsRuntime::new(options)` to create a runtime.
- Register extensions via `RuntimeOptions::extensions`.
- Add ops with `OpDecl::new_sync` / `OpDecl::new_async`.
- Store extension state in `OpState` via `.put()` / `.get()`.