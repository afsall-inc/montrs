# montrs-lockfile — Agent Guide

## Overview
Records locked tool versions so installations are deterministic across machines. The `montrs.lock` file is auto-generated and committed to version control.

## Key Concepts
- **MontrsLock**: The full lockfile with format version, tools, and config sources.
- **LockfileTool**: A single locked version with backend, options, and platform-specific info.
- **PlatformInfo**: Download URL, SHA256 checksum, and binary names for a specific platform.
- **LockFile**: File-based mutex for serializing concurrent installs.

## Agent Usage
- Build a lockfile with `MontrsLock::new()` + `set_tool()`.
- Persist with `write_lockfile(path, &lock)`.
- Load with `read_lockfile(path)` or `parse_lockfile(content)`.
- Check platform with `MontrsLock::current_platform()`.

## Local Invariants
Read `docs/invariants.md` before modifying.