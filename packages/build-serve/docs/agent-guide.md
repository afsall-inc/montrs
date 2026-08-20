# Agent Guide: montrs-build-serve

## Core Concepts
HTTP dev server for static file serving with optional live reload.

### Server Configuration
- `ServeConfig` configures the address, port, and static file root.
- The server is built on Axum and Tower for HTTP handling.
- Static files are served with appropriate cache headers.

### Live Reload
- When enabled, the server injects a WebSocket-based reload script.
- File changes detected by `build-watch` trigger a reload signal.
- The reload script is injected into HTML responses automatically.

### Integration
- Used by `montrs serve` for development workflows.
- Composes with `build-watch` for automatic rebuilds.
- Can be extended with custom middleware via Tower.

## Important Rules
- The dev server is for development only — not for production use.
- Live reload is optional and disabled by default in production builds.
- Static file serving respects the configured root directory.
- Custom middleware can be added via Tower layers.