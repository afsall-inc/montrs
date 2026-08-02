# Agent Guide: montrs-metadata

Agents use this package to understand and manipulate MontRS project metadata.

## Key Types

- **`MontrsMetadata`** — Full project metadata read from `montrs.toml`
- **`ProjectMeta`** — Project identity (name, version, description)
- **`ServeMeta`** — Build/serve config (bin-package, lib-package, site-addr, etc.)
- **`BuildMeta`** — Build options (release, target)

## Common Patterns

```rust
use montrs_metadata::MontrsMetadata;

// Load metadata
let meta = MontrsMetadata::from_file("montrs.toml")?;

// Generate cargo-leptos compatible metadata
let section = meta.to_leptos_metadata_section();
println!("{}", serde_json::to_string_pretty(&section)?);
```

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
lib-features = ["hydrate"]
```

## @agent-tool
- `MontrsMetadata::from_file()` — Load metadata from `montrs.toml`
- `MontrsMetadata::to_leptos_metadata_section()` — Generate leptos-compatible JSON