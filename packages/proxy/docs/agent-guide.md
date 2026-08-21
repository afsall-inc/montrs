# Agent Guide: montrs-proxy

## Core Concepts
Reverse proxy routing for local development — maps `<slug>.localhost` to configured ports.

### Routing
- Routes are defined in `montrs.toml` under the `[proxy]` section.
- Each route maps a slug (e.g., `api`) to a port (e.g., `3001`).
- Requests to `api.localhost` are proxied to `localhost:3001`.

### Configuration
- Routes are deterministic and declared in the project metadata.
- SSL/TLS is handled at the proxy level when enabled.
- Custom headers can be injected per route.

### Health Checks
- The proxy can verify upstream health before routing.
- Unhealthy upstreams are automatically skipped.
- Health check intervals are configurable.

## Important Rules
- Routing is deterministic based on configuration.
- Only localhost routing is supported (development use only).
- SSL/TLS is optional and configured per-route.
- Logging is available for debugging proxy routes.