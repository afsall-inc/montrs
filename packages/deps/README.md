# montrs-deps

Dependency management for MontRS. Processes the `[deps]` section of `montrs.toml` and provides freshness checking between lockfiles and build outputs.

## Features

- **`[deps]` parsing**: `cargo:ripgrep@14.0.0`, `git-submodule:repo.git`, `npm:react`, etc.
- **Freshness checking**: Compares source hashes against stored state
- **Provider resolution**: Cargo, git-submodule, npm, and custom
- **State persistence**: Saves source hashes to `.montrs/deps/state.json`

## Usage

```toml
# montrs.toml
[deps]
cargo:ripgrep = "14.0.0"
npm:react = { auto = true }
git-submodule:https://github.com/org/repo.git = "main"
```

```rust
use montrs_deps::DepsManager;

let mut manager = DepsManager::new(&project_root);
manager.load_from_config(&config.deps);
let resolved = manager.check_freshness("cargo:ripgrep")?;
```