# montrs-build

Facade crate for the MontRS build system. Re-exports `montrs-build-core`, `montrs-build-watch`, and `montrs-build-serve` for convenience, and provides the concrete `Pipeline` struct.

## Sub-packages

| Package | Description |
|---------|-------------|
| `montrs-build-core` | `BuildPipeline` trait + `BuildConfig` types |
| `montrs-build-watch` | File watcher with debounced rebuild (notify) |
| `montrs-build-serve` | Dev server (axum static file serving) |
| `montrs-build` (this) | Facade — re-exports all three + concrete `Pipeline` |

## Usage

```rust
use montrs_build::Pipeline;

let pipeline = Pipeline::from_root(std::path::Path::new("."))?;
pipeline.build_all()?;
```

## CLI

These are used by the `montrs` CLI internally:

```bash
montrs build    # build everything
montrs serve    # build + serve
montrs watch    # build + watch for changes
```

## Pipeline Steps

1. **`build_server()`** — Compiles the server binary
2. **`build_frontend()`** — Compiles the WASM frontend
3. **`process_tailwind()`** — Processes Tailwind CSS via `tailwindcss` CLI
4. **`copy_assets()`** — Copies static assets to the site root
5. **`generate_index_html()`** — Generates the index.html entry point