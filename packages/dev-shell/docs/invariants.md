# Dev-Shell Package Invariants

## 1. Responsibility
`montrs-dev-shell` is the generic, dev-only HTTP shell that hosts a
hot-swappable MontRS app `cdylib`. It owns the listener, renders each request by
calling into the current dylib through `montrs-app-abi`, and swaps the app in
place when the CLI points it at a freshly built dylib.

## 2. Invariants
- **Listener never restarts**: reloading swaps the loaded app; the HTTP listener
  and in-flight connections are never dropped.
- **Old libraries are leaked, never unloaded**: Windows cannot safely unload a
  loaded DLL; leak is bounded by reloads in a dev session.
- **ABI-checked**: refuse to load a dylib whose `ABI_VERSION` differs.
- **Dependency-light**: depends only on `montrs-app-abi` (plus HTTP/runtime
  crates). It must not pull the app's or the build tooling's dependencies.
- **Dev-only**: not part of a production build.

## 3. Boundary Definitions
- **In-Scope**: loading/swapping the dylib, HTTP serving, the reload channel.
- **Out-of-Scope**: building the app, watching source files, hot-patching.

## 4. Agent Guidelines
- Reload is triggered either by `POST /__montrs/reload` or by the
  `MONTRS_RELOAD_FILE` watcher; keep both working.
