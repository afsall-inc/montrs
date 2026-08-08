# montrs-plugin

Plugin system for MontRS — asdf-compatible tool plugins.

## Features

- **Plugin trait**: `name`, `plugin_type`, `plugin_path`, `list_versions`, `install`, `uninstall`
- **PluginType**: `Asdf` (git-based), `Vfox` (Lua-based)
- **PluginSource**: `Git`, `Local`, `Zip`
- **PluginRegistry**: Track installed plugins, list installed
- **git-based install**: `install_git_plugin`, `update_git_plugin`
- **local copy**: `install_local_plugin`
- **removal**: `uninstall_plugin`

## Usage

```rust
use montrs_plugin::{PluginRegistry, install_git_plugin};

let registry = PluginRegistry::new();
install_git_plugin(&registry, "rust", "https://github.com/asdf-community/asdf-rust.git").await?;
```