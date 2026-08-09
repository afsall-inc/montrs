# montrs-tool — Invariants

## 1. Responsibility
Install, list, and manage tool versions through pluggable backends.

## 2. Invariants
- **Backend trait**: All installation goes through `ToolBackend` — no direct install logic in the manager.
- **Registry-driven**: Tool lookup uses `montrs-registry` metadata.
- **Versioned installs**: Each version installs to `install_dir/<tool>/<version>`.
- **Shims**: Binaries are exposed via shims in a shared directory.
- **Checksummed**: Downloaded artifacts have SHA256 digests computed.

## 3. Boundary
- **In-Scope**: Backend abstraction, install/uninstall/list, download, extraction, shims.
- **Out-of-Scope**: Shell activation, environment setup, task execution.

## 4. Agent Guidelines
- Use `ToolManager::new()` for the default setup.
- Use `ToolRequest::parse("name@version")` for requests.
- Use `create_backend()` to instantiate a backend directly.