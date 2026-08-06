# Runner Package Invariants

## 1. Responsibility
`montrs-runner` provides the task runner for MontRS. It parses task definitions from `montrs.toml` and executes them with dependency resolution.

## 2. Invariants
- **Task Config**: Supports `Simple(String)` and `Detailed` task definitions.
- **Dependency Resolution**: Tasks must be executed in topological order based on `dependencies`.
- **No Mise Dependency**: Must not require `mise` to be installed — internal runner is primary.

## 3. Boundary Definitions
- **In-Scope**: Task parsing, execution, dependency resolution, listing.
- **Out-of-Scope**: Tool version management, environment variables, shell integration.

## 4. Agent Guidelines
- Use `TaskRunner::new()` to create a runner from parsed tasks.
- Call `run(task_name)` to execute a task and its dependencies.