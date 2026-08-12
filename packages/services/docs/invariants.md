# montrs-services — Invariants

- **Layer**: 2 (depends on env, log, build-watch)
- **Config**: Parsed from `montrs.toml [services]` section
- **ServiceId**: Namespace/name with safe-path encoding
- **Supervisor**: Background process manager with retry loop
- **Ready checks**: Delay, output regex, HTTP, TCP port, custom command
- **Retry**: Configurable count, delay, exponential backoff
- **Hooks**: on_ready, on_fail, on_retry, on_stop, on_exit
- **State file**: TOML at `~/.local/state/montrs/services/montrs-services.toml`
- **CLI commands**: `montrs services list|start|stop|restart|status|logs`
- **All methods must be Send + Sync** (for tokio::spawn usage)
- **No reqwest dependency** — use tokio TcpStream for HTTP checks