# montrs-desktop

Native desktop support for MontRS applications.

**Target Audiences:** Application Developers, Framework Contributors.

## 1. What this package is
`montrs-desktop` provides the `DesktopAdapter` (`PlatformAdapter` implementation) plus webview and native window runners built on `wry` and `winit`.

## 2. What problems it solves
- **One codebase, many shells**: Run a MontRS web app as a desktop window.
- **Platform lifecycle**: Window creation, title, and URL handling.

## 3. What it intentionally does NOT do
- **Rendering**: Delegates to the platform webview / windowing backend.
- **Mobile**: Android/iOS is `montrs-mobile`.

## 4. How it fits into the MontRS system
Layer 3, implements the `PlatformAdapter` trait from `montrs-platform` for `Target::Desktop`.

## 5. When a user should reach for this package
- Shipping a MontRS app as a native desktop application.

## 6. Deeper Documentation
- [Invariants](docs/invariants.md)