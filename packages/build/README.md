# montrs-build

Native Rust build pipeline for MontRS applications. Replaces `cargo-leptos` entirely.

## What it does

- Reads `montrs.toml` for project metadata
- Builds the server binary with `cargo build`
- Builds the WASM frontend with `cargo build --target wasm32-unknown-unknown`
- Runs Tailwind CSS v4 CLI (native binary, no JS)
- Copies assets to the site root
- Watches files for changes with auto-rebuild
- Serves the site with a lightweight Axum dev server

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
5. **`copy_wasm_package()`** — Copies the WASM package to the site directory