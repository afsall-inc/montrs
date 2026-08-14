# montrs-registry

Tool registry for MontRS — tool metadata for version management.

## Features

- **Baked registry**: Compiled from `registry/*.toml` at build time via `include_str!`
- **Floating registry**: Downloadable from `montrs.com` with TTL-based caching
- **Tool lookup**: `get()`, `has()`, `search()` by name/description/binary
- **Platform-aware backend selection**: `best_backend()` picks the best backend for the current OS
- **Registry file format**: TOML files with `backends`, `bins`, `description`, `detect`, `idiomatic_files`, `aliases`, `platform` overrides

## Usage

```rust
use montrs_registry::BAKED_REGISTRY;

let tool = BAKED_REGISTRY.get("rust").unwrap();
println!("{}: {}", tool.name, tool.description);
```