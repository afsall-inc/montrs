# Dev-Hotpatch Package Invariants

## 1. Responsibility
`montrs-dev-hotpatch` is the Rust hot-patching dev toolchain: it captures the
workspace's `rustc`/link invocations, links the hot-patchable ("fat") binary and
the patch shared library, builds jump tables, hosts the dev hub, and provides the
native client plus the rustc/link wrapper shims.

## 2. Invariants
- **Dev-only**: never a runtime dependency of a shipped application. Only the CLI
  and the `montrs-hotpatch` facade may depend on it.
- **Externally tagged protocol**: `DevserverMsg`/`ClientMsg` serialize with
  external tagging so a populated `subsecond_types::AddressMap` round-trips and
  Leptos's devtools client accepts the payload. Never re-introduce internal
  tagging.
- **Direct symbol addresses**: the fat link and the patch link must disable
  incremental linking (`/INCREMENTAL:NO`); an ILT thunk for `main` breaks the
  rebase that `subsecond::apply_patch` performs.
- **Symbols need a PDB**: the fat link forces full debug info and a PDB; the run
  copy falls back to the `deps` PDB. The patch builder cannot index a binary
  without one.
- **Best-effort capture**: a wrapper or capture failure must never break a user's
  build.

## 3. Boundary Definitions
- **In-Scope**: invocation capture, fat/patch linking, jump-table construction,
  the dev hub, the native client, debug binaries.
- **Out-of-Scope**: the application-facing API, build orchestration, file
  watching.

## 4. Agent Guidelines
- `build_patch` is the single entrypoint for producing a patch; the shims are the
  only place tip objects can be captured (rustc deletes them after linking).
- Keep wire types in `lib.rs`/`server.rs`; keep linker argument assembly pure and
  unit-tested.
