# montrs-services

Service supervisor for MontRS — daemon management with developer experience.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-services` manages background services (daemons) defined in the `[services]` section of `montrs.toml`. It handles process lifecycle, ready checks, retry logic, lifecycle hooks, cron scheduling, and file-watch restarts.

## 2. What problems it solves
- **Dev service management**: Start/stop Postgres, Redis, APIs, and workers without Docker Compose.
- **Ready checks**: Know when a service is actually ready (HTTP, port, output regex).
- **Failure recovery**: Automatic retry with exponential backoff.

## 3. What it intentionally does NOT do
- **Container orchestration**: It's a process supervisor, not Kubernetes.
- **Log analysis**: It captures output but delegates storage to `montrs-log`.

## 4. How it fits into the MontRS system
Layer 2, depends on `montrs-log`, `montrs-env`, `montrs-build-watch`. Driven by the `montrs services` CLI command and exposed as MCP tools.

## 5. When a user should reach for this package
- Defining dev services for a MontRS project.
- Auto-starting services when entering a project directory (shell hook).
- Building a service dashboard (TUI or web).

## 6. Quick start
```toml
[services.redis]
run = "redis-server --port 6379"
auto = "start-stop"
ready_port = { port = 6379 }
```

## 7. Deeper Documentation
- [Invariants](docs/invariants.md)
- [Supervisor](src/supervisor.rs)
- [Config](src/config.rs)