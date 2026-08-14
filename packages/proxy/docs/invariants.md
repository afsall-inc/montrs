# montrs-proxy — Invariants

- **Layer**: 3 (standalone, no deps on other montrs packages)
- **Routing**: `<slug>.localhost` → `127.0.0.1:<port>`
- **Server**: Axum-based with reqwest forwarding
- **TLS**: Self-signed cert generation via rcgen
- **Fallback**: Optional fallback port for unmatched routes
- **Port resolution**: Bump on conflict (not yet implemented)
- **mDNS**: LAN mode discovery (not yet implemented)