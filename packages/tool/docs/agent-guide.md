# montrs-tool — Agent Guide

## Overview
Manages tool versions. Installs from GitHub releases, crates.io, or HTTP URLs. Creates shims so tools are available on PATH.

## Key Concepts
- **ToolBackend trait**: Plugable installer for different source types.
- **ToolManager**: Orchestrates installs, lookups, and shim creation.
- **ToolRequest**: `name@version` string parser.
- **BackendType**: Core, Cargo, GitHub, HTTP, UBI, etc.

## Agent Usage
- `ToolManager::new()` then `manager.install(&ToolRequest::parse("tool@1.0"))` to install.
- `manager.list_installed("tool")` to see installed versions.
- `manager.list_remote("tool")` to see available versions.
- `manager.create_shim("tool", "bin-name", "1.0")` to create a shim.

## Local Invariants
Read `docs/invariants.md` before modifying.