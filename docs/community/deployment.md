# Deploying MontRS Applications

MontRS applications compile to native binaries that serve both server-rendered
HTML and optimized client assets. This guide explains how to build, configure,
and run MontRS in production.

---

## 1. Development vs. Production Mode

MontRS enforces a clean separation between **development** and **production**:

| Behavior | `development` (`dev`) | `production` (`prod`) |
|----------|-----------------------|-----------------------|
| Dev overlay & console | Injected into HTML | **Omitted completely** |
| Hot-reload socket (`/_dioxus`) | Active | **Disabled** |
| Live reload script | Injected | **Omitted** |
| Cache headers (`/pkg/*`, `/main.css`) | `no-store, no-cache` | `public, max-age=31536000, immutable` |
| Cache headers (HTML) | `no-store, no-cache` | `no-cache` |
| Compilation profile | `dev` / `hot` | `--release` |

### Setting the mode

Use the `MONTRS_MODE` environment variable:

```bash
export MONTRS_MODE=production   # or 'prod'
```

Alternatively, set the default mode declaratively in `montrs.toml`:

```toml
[deploy]
mode = "production"    # development | production (aliases: dev | prod)
target = "ssr"         # ssr | static | desktop | mobile
```

### Precedence

1. **CLI Flag**: `montrs serve` always forces `development`. `montrs build` defaults to `production`. Pass `montrs build --dev` to deliberately produce a development build.
2. **Environment Variable**: `MONTRS_MODE=production` or `MONTRS_MODE=development`.
3. **Configuration**: `montrs.toml [deploy] mode`.
4. **Fallback**: Release binaries infer `production`; debug binaries infer `development`.

> **Safety Guardrail:** Running `montrs build` when the resolved mode is `development` will fail unless the `--dev` flag is explicitly passed. This prevents accidentally building and shipping the dev overlay to production.

---

## 2. Building for Production

Run the production build:

```bash
montrs build
```

This generates:
- The compiled SSR binary: `target/release/<pkg>-ssr` (`.exe` on Windows).
- The client assets & hydration bundle: `target/site/` (e.g. `main.css`, `pkg/front.js`, `pkg/front_bg.wasm`, and assets).

---

## 3. Running in Production

To run the application, provide both the server binary and the `target/site` directory:

```bash
export MONTRS_MODE=production
export MONTRS_SITE_ADDR=0.0.0.0:3000
export MONTRS_SITE_ROOT=./site
export MONTRS_SITE_PKG_DIR=pkg

./target/release/my-app-ssr
```

### Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `MONTRS_MODE` | `production` (in release) | Mode selection (`development` or `production`) |
| `MONTRS_SITE_ADDR` | `0.0.0.0:3000` | Address and port to bind |
| `MONTRS_SITE_ROOT` | `target/site` | Filesystem path to client assets |
| `MONTRS_SITE_PKG_DIR` | `pkg` | Subdirectory containing WASM bundle |
| `MONTRS_OUTPUT_NAME` | (from `montrs.toml`) | Base name of the client WASM bundle |

---

## 4. Production Dockerfile

Use a multi-stage build with the pinned Rust toolchain:

```dockerfile
FROM rustlang/rust:nightly-2026-02-18 AS builder

WORKDIR /app
COPY . .

# Install MontRS CLI and toolchain dependencies
RUN cargo install --path packages/cli
RUN montrs install

# Build optimized production bundle
RUN montrs build

FROM debian:bookworm-slim AS runner

WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the server executable and static site files
COPY --from=builder /app/target/release/website-ssr /app/server
COPY --from=builder /app/target/site /app/site

ENV MONTRS_MODE=production
ENV MONTRS_SITE_ADDR=0.0.0.0:3000
ENV MONTRS_SITE_ROOT=/app/site
ENV MONTRS_SITE_PKG_DIR=pkg

EXPOSE 3000
CMD ["/app/server"]
```

---

## 5. Reverse Proxy Configuration (Caddy / Nginx)

For production deployments, place MontRS behind a reverse proxy to terminate TLS and forward requests:

### Caddyfile
```caddy
example.com {
    reverse_proxy 127.0.0.1:3000
}
```
