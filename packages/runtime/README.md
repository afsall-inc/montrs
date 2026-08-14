# montrs-runtime

General-purpose Rust runtime with ops, extensions, resource table, event loop, and memory optimization.

## Architecture

- **MontrsRuntime** — main runtime struct managing extensions, ops, and event loop
- **OpState** — TypeMap for extension state (stores one value per type)
- **RuntimeExtension** — pluggable extension with ops, state init, and lifecycle hooks
- **OpDecl** — typed operation declaration (sync/async, with/without JSON input)
- **ResourceTable** — typed handles identified by ResourceId
- **EventLoop** — tokio-based async task management
- **ModuleLoader** — trait for loading Rust/WASM modules
- **Arena** — O(1) bump allocator for high-throughput temporary data
- **TaggedValue** — NaN-boxed 64-bit value representation (int, float, bool, ptr, null)
- **BitField** — packed multi-field structs in a single u64

## MontRS Extension

The `montrs_ext` module provides a MontRS-specific runtime extension with ops for:
- `montrs.ping` — health check
- `montrs.resource_count` — count of resources in the resource table
- `montrs.sleep_ms` — async sleep (for testing)