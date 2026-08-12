# montrs-runtime — Invariants

## 1. Responsibility
Provide a general-purpose Rust runtime: ops, extensions, resource table, event loop, module loading, and memory primitives.

## 2. Invariants
- **TypeMap-based OpState**: Extension state is stored in a `TypeMap` (one value per type).
- **Extension-driven**: All functionality is added via `RuntimeExtension` — the core has no built-in ops.
- **Typed resources**: `ResourceTable` stores `Box<dyn Resource>` keyed by `ResourceId`.
- **Tokio event loop**: All async tasks run on tokio's runtime; `run()` is event-driven via `Notify` (no busy-wait).
- **Memory primitives**: `Arena` (bump allocation with CAS overflow safety), `TaggedValue` (tag-marker scheme), `BitField` (packed fields).
- **Single OpId counter**: All `OpDecl` constructors share one global `AtomicU16` — IDs never collide across sync/async variants.
- **Dependency-ordered lifecycle**: `init_all_states` / `start_all` / `stop_all` run in topological order (deps first; stop is reverse). Cycles return `RuntimeError::ExtensionCycle`.
- **Single tables**: `ResourceTable` and `EventLoop` live only in `OpState` (not duplicated as struct fields).
- **Structured errors**: All failures are `RuntimeError` with stable `RuntimeErrorKind` codes and `suggested_fixes`.
- **Resource close**: `Resource::close()` returns `Result<(), RuntimeError>`; table propagates errors.

## 3. Boundary
- **In-Scope**: Runtime struct, extensions, ops, resources, event loop, module loader, memory primitives, structured errors.
- **Out-of-Scope**: Application logic, routing, ORM, auth, web server, FS/Net/HTTP extensions (Phase 6 B1+).

## 4. Agent Guidelines
- Use `MontrsRuntime::new(options)?` then `init()?`.
- Register extensions via `RuntimeOptions::extensions`.
- Add ops with `OpDecl::new_sync` / `OpDecl::new_async`.
- Store extension state in `OpState` via `.put()` / `.get()`.
- Match errors on `err.kind()` / `err.code()`, not string equality.
