# montrs-plugin — Agent Guide

## Overview
Manages tool plugins — the backends that know how to install and manage specific tools. Supports asdf-style git plugins.

## Key Concepts
- **Plugin trait**: Core interface for any tool plugin.
- **PluginRegistry**: Tracks where plugins are installed.
- **PluginType**: `Asdf` (git-based) or `Vfox` (Lua-based).
- **PluginSource**: `Git`, `Local`, or `Zip`.

## Agent Usage
- `install_git_plugin(&registry, "name", "git-url")` to install a new plugin.
- `uninstall_plugin(&registry, "name")` to remove one.
- `registry.list_installed()` to see all plugins.

## Local Invariants
Read `docs/invariants.md` before modifying.