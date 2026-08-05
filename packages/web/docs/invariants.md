# Web Package Invariants

## 1. Responsibility
`montrs-web` implements `PlatformAdapter` from `montrs-platform` for browser/WASM targets. It uses `web-sys` and `wasm-bindgen` for DOM and browser API access.

## 2. Invariants
- **PlatformAdapter Implementation**: Must implement `PlatformAdapter` from `montrs-platform`.
- **WASM-First**: The primary target is `wasm32-unknown-unknown`. Native compilation is allowed for development/testing but methods are no-ops.
- **No Leptos Dependency**: This package does not depend on Leptos — it only uses raw `web-sys` bindings.

## 3. Boundary Definitions
- **In-Scope**: Browser URL navigation, document title, window metadata.
- **Out-of-Scope**: Leptos integration, SSR, routing, rendering.

## 4. Agent Guidelines
- WASM-specific code should be gated with `#[cfg(target_arch = "wasm32")]`.
- Default methods should be no-ops on non-WASM targets.