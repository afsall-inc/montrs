# montrs-proxy

Reverse proxy for MontRS local development.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-proxy` routes `<slug>.localhost` hostnames to local service ports, with optional TLS via self-signed certificates. It's the bridge between `montrs-services` daemons and your browser.

## 2. What problems it solves
- **Port management**: `api.localhost` → `127.0.0.1:3001` without remembering ports.
- **TLS in dev**: Self-signed cert generation for HTTPS-ready local development.
- **Fallback routing**: A default target port for unmatched hosts.

## 3. What it intentionally does NOT do
- **Production reverse proxy**: Use a real gateway for production.
- **Load balancing**: Single target per slug only.

## 4. How it fits into the MontRS system
Layer 3, standalone. Works alongside `montrs-services` (proxy to daemon ports).

## 5. When a user should reach for this package
- Running multiple dev services with memorable hostnames.
- Testing HTTPS locally.

## 6. Quick start
```rust
let proxy = ProxyServer::new(ProxyConfig {
    routes: vec![RouteEntry {
        slug: "api".into(), target_port: 3001, use_tls: false,
    }],
    ..Default::default()
});
proxy.serve().await?;
```

## 7. Deeper Documentation
- [Invariants](docs/invariants.md)
- [Server](src/server.rs)
- [TLS](src/tls.rs)