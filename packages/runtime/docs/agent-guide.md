# montrs-runtime — Agent Guide

## Overview
General-purpose Rust runtime. Extensions register ops that can be called synchronously or asynchronously with access to the shared OpState.

## Key Concepts
- **MontrsRuntime**: Create with `RuntimeOptions`, then `init()`. Call ops with `op_sync()` / `op_async()`.
- **RuntimeExtension**: `builder("name").ops(vec![...]).init_state(|s| ...).build()`
- **OpDecl**: `new_sync`, `new_async`, `new_sync_with_input`, `new_async_with_input`
- **OpState / TypeMap**: `state.put(my_value)`, `state.get::<MyType>()`
- **ResourceTable**: `table.add(Box::new(my_resource))`, `table.get(id)`
- **EventLoop**: `runtime.event_loop.spawn("name", future)`
- **Arena**: Fast bump allocation for temporary data

## Agent Usage
- Register a custom op: `OpDecl::new_sync("my.op", |state| { ... })`
- Store shared state: `state.put(MyConfig::default())`
- Spawn tasks: `runtime.event_loop.spawn("task", async { ... })`

## Local Invariants
Read `docs/invariants.md` before modifying.