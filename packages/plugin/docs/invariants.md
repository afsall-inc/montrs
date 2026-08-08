# montrs-plugin — Invariants

## 1. Responsibility
Manage tool plugins (asdf-compatible). Install, update, uninstall, and list plugins.

## 2. Invariants
- **Git-based**: Primary install method is `git clone --depth 1`.
- **Local copy**: `install_local_plugin` copies directories.
- **Plugin path**: Installed under `~/.local/share/montrs/plugins/<name>`.
- **Idempotent**: Installing an already-installed plugin returns an error.

## 3. Boundary
- **In-Scope**: Plugin install, uninstall, update, listing, source detection.
- **Out-of-Scope**: Tool version resolution, binary management, registry lookup.

## 4. Agent Guidelines
- Use `PluginRegistry::new()` for the default plugin directory.
- Use `install_git_plugin()` for remote git plugins.
- Use `uninstall_plugin()` to remove a plugin.