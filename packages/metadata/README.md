# montrs-metadata

MontRS project metadata abstraction. Reads `montrs.toml` and provides all configuration needed for building, serving, and deploying MontRS applications.

## What it does

- Replaces the need for `[[workspace.metadata.leptos]]` or `[package.metadata.leptos]` in `Cargo.toml`
- Auto-detects `bin-package` and `lib-package` from Cargo workspace
- Auto-detects project name from `Cargo.toml`
- Generates cargo-leptos compatible metadata for legacy compatibility

## Example `montrs.toml`

```toml
[project]
name = "my-app"
version = "0.1.0"

[serve]
site-addr = "0.0.0.0:3000"
reload-port = 3001
tailwind-input-file = "style/main.css"
bin-package = "server"
lib-package = "app"
site-root = "target/site"
site-pkg-dir = "pkg"
lib-features = ["hydrate"]
lib-default-features = false
```

## Usage

```rust
use montrs_metadata::MontrsMetadata;

let meta = MontrsMetadata::from_file("montrs.toml")?;
println!("Project: {}", meta.project.name.unwrap_or("unknown"));
println!("Server: {}", meta.serve.bin_package.unwrap_or("auto"));
```

## Fields

| Section | Field | Default | Description |
|---------|-------|---------|-------------|
| `project` | `name` | auto | Project name |
| `project` | `version` | — | Project version |
| `project` | `description` | — | Project description |
| `serve` | `bin-package` | auto | Binary package to build |
| `serve` | `lib-package` | auto | Library package for WASM |
| `serve` | `site-addr` | `0.0.0.0:3000` | Dev server address |
| `serve` | `reload-port` | `3001` | Live reload port |
| `serve` | `tailwind-input-file` | — | Tailwind CSS input file |
| `serve` | `site-root` | `target/site` | Output directory |
| `serve` | `site-pkg-dir` | `pkg` | WASM package directory |
| `serve` | `lib-features` | `[]` | WASM build features |
| `serve` | `bin-features` | `[]` | Server build features |
| `serve` | `lib-default-features` | `true` | Use default features for WASM |