# Agent Guide: montrs-services

## Core Concepts
Service supervisor for daemon management, ready checks, retry, hooks, and cron scheduling.

### Service Configuration
- `run` — command to execute.
- `auto` — `start-stop`, `always`, `manual`.
- `ready_port` / `ready_http` — health check strategies.
- `depends` — service dependencies.

### Key Operations
- `start`, `stop`, `restart` — lifecycle management.
- `status` — check service health.
- `logs` — view service output.

## Important Rules
- Services are managed as child processes.
- Dependencies are started before dependent services.
- Health checks use port or HTTP readiness probes.