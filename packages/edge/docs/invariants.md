# Edge Package Invariants

## 1. Responsibility
`montrs-edge` provides an edge runtime adapter for MontRS. It implements `PlatformAdapter` from `montrs-platform` and provides a lightweight request handler compatible with the `fetch` event model used by Cloudflare Workers, Deno, and similar environments.

## 2. Invariants
- **PlatformAdapter Implementation**: Must implement `PlatformAdapter` from `montrs-platform`.
- **Zero Heavy Dependencies**: Must not depend on `axum`, `hyper`, `tower`, or any server-specific crate.
- **Fetch-Compatible**: The `EdgeRequest`/`EdgeResponse` model mirrors the `fetch` event API.
- **Feature-Gated Platforms**: Cloudflare and Deno integrations are behind feature flags.

## 3. Boundary Definitions
- **In-Scope**: `EdgeAdapter`, `EdgeRequest`, `EdgeResponse`, `handle_edge_request`, Cloudflare/Deno stubs.
- **Out-of-Scope**: Full runtime implementations, WASM bindings, deployment tooling.

## 4. Agent Guidelines
- New edge platforms should be added as feature-gated modules.
- The `handle_edge_request` function is the universal entry point — platform-specific modules convert to/from it.