# MontRS API Template

Backend-only API server built on MontRS.

## Features

- Axum HTTP server with health endpoint
- `montrs-auth` (email/password sign-in + sign-up)
- `montrs-services` for daemon management (api, postgres)

## Getting started

```bash
montrs services start     # start postgres + api
montrs serve              # run the API server
```

## Services

| Service | Command | Ready check |
|---------|---------|-------------|
| `api` | `cargo run --bin api` | HTTP :3000/health |
| `postgres` | docker postgres:16 | TCP :5432 |