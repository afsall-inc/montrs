# Build-Watch Package Invariants

## 1. Responsibility
`montrs-build-watch` provides file system watching for MontRS projects. It depends on the `BuildPipeline` trait (not the concrete implementation) so it can be used independently of the full build pipeline.

## 2. Invariants
- **Trait-Only Dependency**: Must depend on `montrs-build-core` for the trait, not on `montrs-build` for the concrete pipeline.
- **Debounced Events**: Change events must be debounced (300ms) to avoid redundant rebuilds during save operations.
- **Cross-Platform**: Uses `notify` for platform-native file watching.

## 3. Boundary Definitions
- **In-Scope**: File watching, debouncing, rebuild trigger.
- **Out-of-Scope**: Build logic, dev server, HTTP serving.

## 4. Agent Guidelines
- The `watch_directory` function is the low-level primitive; `watch_and_rebuild` is the convenience wrapper.