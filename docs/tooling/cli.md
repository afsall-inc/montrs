# montrs CLI Reference

The `montrs` CLI is the primary tool for managing MontRS applications.

## General Flags

- `--release`: Build artifacts in release mode.
- `--hot-reload`: Enable partial hot-reloading.
- `--verbose`, `-v`: Increase logging verbosity.
- `--features`: Specify features to use during compilation.

## Commands

### `new`
Scaffold a new MontRS project from a template.
```bash
montrs new <name> [--template <template>]
```

### `build`
Build the project for production.
```bash
montrs build
```

### `serve`
Start the development server with hot-reloading.
```bash
montrs serve
```

### `bench`
Run performance benchmarks.

**Arguments:**
- `target`: (Optional) The target file, directory, or filter.

**Flags:**
- `--iterations <N>`: Number of benchmark iterations (default: 100).
- `--warmup <N>`: Number of warmup runs (default: 10).
- `--timeout <SECONDS>`: Maximum execution time for each benchmark.
- `--filter <STRING>`: Filter benchmarks by name.
- `--json-output <PATH>`: Export results to a JSON file.
- `--generate-weights <PATH>`: Generate a Rust file with weight constants (Substrate-style).
- `--simple`: **Native Mode**. Benchmarks a file/binary directly without project overhead. Requires `target`.

**Examples:**
```bash
# Standard project benchmarks
montrs bench

# Native benchmark of a script
montrs bench --simple scripts/process_data.rs
```

### `fmt`
Format the project's Rust and view! code.

**Arguments:**
- `path`: (Optional) The file or directory to format (default: `.`).

**Flags:**
- `--check`: Verifies if files are formatted without modifying them.
- `--verbose`: Show detailed output.

**Examples:**
```bash
# Format everything
montrs fmt

# Check formatting in CI
montrs fmt --check
```

### `test`
Run project tests (Unit, Integration, E2E).
```bash
montrs test [--filter <name>] [--report <format>]
```

### `spec`
Generate a machine-readable specification of the project.
```bash
montrs spec [--format <json|yaml|txt>]
```
This command refreshes the `.agent/agent.json` file used by agents.

### `run`
Run custom tasks defined in `montrs.toml`.
```bash
montrs run <task_name>
```

### `sketch`
Generate a single-file, explicit "sketch" of a MontRS component. This is the first step in the **Scaffolded Explicit** workflow.
```bash
montrs sketch <name> [--kind <plate|route|app>]
```

### `expand`
Expand a `.sketch.rs` file into a full project structure with explicit imports and manual registration.
```bash
montrs expand <path_to_sketch>
```

### `generate`
Generate boilerplate for plates and routes. This is the preferred way to add new components to your application to maintain the **Productive Explicitness** principle.

**Subcommands:**

- **`plate <name>`**: Generates a new `Plate` implementation in `src/plates/`.
- **`route <path> --plate <name>`**: Generates a new unified `Route` implementation (Params, Loader, Action, View) within the specified plate's directory.

**Examples:**
```bash
# Create a new Auth plate
montrs generate plate Auth

# Add a login route to the Auth plate
montrs generate route /login --plate Auth
```

## 🤖 Agent-first CLI

MontRS CLI is built to be a primary communication channel between the developer and agents:

- **Error Capturing**: When a command fails, the CLI generates a versioned `errorfile.json` in `.agent/errorfiles/`.
- **Context Awareness**: The CLI knows the state of your project through the `.agent` folder, allowing it to provide smarter error messages and suggested fixes.

### `agent list-errors`
List all tracked errors and their status.
```bash
montrs agent list-errors [--status <active|resolved>] [--json]
```

### `agent diff`
Show a diagnostic diff for an error file.
```bash
montrs agent diff <path>
```

### `agent resolve`
Resolve a specific tracked error by ID.
```bash
montrs agent resolve <id> [--message <msg>]
```

### `agent check`
Validate structural correctness and project invariants.
```bash
montrs agent check [--path <path>] [--json]
```

### `agent doctor`
Assess project health and agent-readiness.
```bash
montrs agent doctor [--package <pkg>] [--json]
```

### `agent snapshot`
Regenerate the agent snapshot (`agent.json`).
```bash
montrs agent snapshot [--format <json|yaml|txt>]
```

### `agent skills`
List and inspect available skills.
```bash
montrs agent skills [--name <skill-name>]
```

### `agent prdoc`
Manage PR documentation (prdoc.md).
```bash
montrs agent prdoc show [--path <prdoc.md>]
montrs agent prdoc validate [--path <prdoc.md>]
montrs agent prdoc generate [--pr <number>] [--from-diff <path>] [--from-commits <range>] [--embed] [--output <path>] [--force]
```

### `agent changelog`
Generate changelogs and manage version bumps from prdocs.
```bash
montrs agent changelog generate [--from <tag>] [--to <tag>] [--output <path>]
montrs agent changelog bump [--current <version>] [--from <range>] [--dry-run]
montrs agent changelog verify [--from <range>]
```

### `agent rules`
Manage agent rules and IDE integration.
```bash
montrs agent rules list
montrs agent rules export --target <trae|cursor>
```

### `watch`
Watch for changes and rebuild automatically.
```bash
montrs watch
```

### `upgrade`
Upgrade the montrs CLI to the latest version.
```bash
montrs upgrade
```
