# montrs-tool

Tool version manager for MontRS. Installs, lists, and manages tool versions with multiple backend sources.

## Features

- **Backends**: Core, GitHub releases, Cargo, HTTP, UBI
- **Install**: Download and extract tarballs from GitHub releases or HTTP URLs
- **List**: List installed versions and remote versions
- **Uninstall**: Remove specific versions
- **Shim creation**: Create executable shims in a shared directory
- **Registry integration**: Uses `montrs-registry` for tool metadata
- **Checksum verification**: SHA256 digest computation for downloaded artifacts

## Backend Types

| Backend | Registry Prefix | Description |
|---------|----------------|-------------|
| Core | `core:` | Built-in installer (rustup, nvm, etc.) |
| Cargo | `cargo:` | `cargo install` from crates.io |
| GitHub | `github:` | GitHub releases download |
| HTTP | `http:` | Arbitrary HTTP URL |
| UBI | `ubi:` | Universal Binary Installer format |

## Usage

```rust
use montrs_tool::{ToolManager, ToolRequest};

let tm = ToolManager::new();
let version = tm.install(&ToolRequest::parse("ripgrep@14.0.0")).await?;
```