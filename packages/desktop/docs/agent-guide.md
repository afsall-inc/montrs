# Agent Guide: montrs-desktop

## Core Concepts
Desktop shell for MontRS applications — webview (wry) or native (winit + wgpu).

### Webview Mode
- Uses `wry` (WebView) for rendering HTML/JS applications.
- Suitable for most MontRS applications using `montrs-ui`.
- Provides native window decorations, menus, and system tray integration.

### Native Mode
- Uses `winit` + `wgpu` for pure native rendering.
- No browser/WebView dependency — renders directly to the GPU.
- Suitable for TUI-adjacent or custom-rendered applications.

### Configuration
- Window title, size, position, and decorations are configurable.
- Mode (webview/native) is selected via Cargo features.
- Platform-specific behavior is handled by the `PlatformAdapter` trait.

## Important Rules
- Webview mode requires a system WebView runtime (WebKit on macOS, WebView2 on Windows).
- Native mode requires GPU support via wgpu.
- Window configuration is set at startup via `DesktopConfig`.
- Application lifecycle events (open, close, focus) are handled through the adapter.