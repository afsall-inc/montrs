# Build Package Invariants

## 1. Responsibility
`montrs-build` is the facade crate for the MontRS build system. It re-exports `montrs-build-core`, `montrs-build-watch`, and `montrs-build-serve`, and provides the concrete `Pipeline` struct implementing `BuildPipeline`.

## 2. Invariants
- **Facade Only**: Must re-export from sub-packages, not duplicate logic.
- **Concrete Pipeline**: The `Pipeline` struct lives here, not in `build-core`.
- **CI Order**: Tasks must follow the CI order: fmt → clippy → test → build.

## 3. Boundary Definitions
- **In-Scope**: Build orchestration, cargo invocation, tailwind processing, asset copying.
- **Out-of-Scope**: File watching, dev server, build trait definitions.

## 4. Agent Guidelines
- Use `Pipeline::from_root()` to construct the pipeline.
- Always call `build_all()` for a full build.