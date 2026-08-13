# montrs-runner

Custom task runner for MontRS projects.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-runner` parses and executes tasks defined in the `[tasks]` section of `montrs.toml`. It supports dependencies, parallel execution, templating, and workspace-aware task discovery.

## 2. What problems it solves
- **Consistent commands**: `montrs run fmt`, `montrs run test` across the workspace.
- **Dependency ordering**: Tasks run in topological order with parallel branches.
- **Reusable config**: Task definitions live in `montrs.toml`, not shell history.

## 3. What it intentionally does NOT do
- **Process supervision**: Long-running daemons are `montrs-services`.
- **Tool versioning**: That's `montrs-tool`.

## 4. How it fits into the MontRS system
Layer 2, depends on `montrs-utils`, `montrs-fmt`, `montrs-metadata`. Backs the `montrs run` / `montrs tasks` CLI commands.

## 5. When a user should reach for this package
- Defining project tasks in `montrs.toml`.
- Running CI-style task chains.

## 6. Quick start
```toml
[tasks.test-all]
command = "cargo test --workspace"
dependencies = ["lint", "fmt"]
```

## 7. Deeper Documentation
- [Invariants](docs/invariants.md)
- [Scheduler](src/scheduler.rs)
- [Parser](src/parser.rs)