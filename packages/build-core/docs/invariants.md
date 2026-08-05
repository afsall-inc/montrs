# Build-Core Package Invariants

## 1. Responsibility
`montrs-build-core` defines the `BuildPipeline` trait and `BuildConfig` types. It is the interface that `montrs-build-watch` and `montrs-build-serve` depend on, avoiding a dependency on the concrete `Pipeline` implementation.

## 2. Invariants
- **Zero Heavy Dependencies**: Must not depend on `axum`, `hyper`, `tower`, `notify`, or any runtime-specific crate.
- **Trait-Driven**: All build orchestration logic is expressed via the `BuildPipeline` trait.
- **Config-Only**: Contains only configuration types and the trait definition — no executable build logic.

## 3. Boundary Definitions
- **In-Scope**: `BuildPipeline` trait, `BuildStep` enum, `BuildConfig`, `find_workspace_target_dir`.
- **Out-of-Scope**: Concrete build logic (cargo invocations, wasm-bindgen, tailwind), file watching, dev server.

## 4. Agent Guidelines
- When adding a new build step, add a variant to `BuildStep` and a method to `BuildPipeline`.