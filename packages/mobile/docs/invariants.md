# Mobile Package Invariants

## 1. Responsibility
`montrs-mobile` provides the mobile platform shell for MontRS. It implements `PlatformAdapter` from `montrs-platform` and provides `run_webview` / `run_native` entry points.

## 2. Invariants
- **PlatformAdapter Implementation**: Must implement `PlatformAdapter` from `montrs-platform`.
- **Feature-Gated Backends**: `webview` (wry) and `native` (winit + wgpu) are behind feature flags.
- **Stub-Ready**: Mobile platforms are not yet fully supported. Methods should have sensible no-op defaults.
- **No Framework Logic**: Contains no application logic — only platform shell code.

## 3. Boundary Definitions
- **In-Scope**: Mobile window creation, webview hosting, native rendering loop, platform adapter.
- **Out-of-Scope**: Application logic, routing, build pipeline, agent metadata.

## 4. Agent Guidelines
- When adding a new mobile capability, add a method to `PlatformAdapter` first, then implement it here.
- Android and iOS native bridges will be added as separate feature flags.