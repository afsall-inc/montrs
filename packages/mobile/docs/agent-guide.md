# Agent Guide: montrs-mobile

## Core Concepts
Mobile platform adapter for Android and iOS shells.

### Platform Abstraction
- `MobileAdapter` trait provides platform-specific implementations.
- Android uses a WebView-based shell.
- iOS uses a WKWebView-based shell.

### Configuration
- Mobile-specific settings are defined in `montrs.toml`.
- App name, icons, splash screen, and permissions are configurable.
- Build targets are selected via Cargo features.

## Important Rules
- Mobile builds target `wasm32-unknown-unknown` for the web layer.
- Native shell code is separate from the Rust application.
- Platform-specific features should be gated behind feature flags.