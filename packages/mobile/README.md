# montrs-mobile

Mobile platform support for MontRS applications.

**Target Audiences:** Application Developers, Framework Contributors.

## 1. What this package is
`montrs-mobile` provides the `MobileAdapter` (`PlatformAdapter` implementation) for Android and iOS targets.

## 2. What problems it solves
- **Platform abstraction**: A uniform interface for `Target::Mobile` apps.
- **Native integration points**: Webview and native runners for mobile shells.

## 3. What it intentionally does NOT do
- **Build tooling**: Android/iOS packaging is outside this package.
- **Native UI**: Uses webviews for rendering.

## 4. How it fits into the MontRS system
Layer 3, implements `PlatformAdapter` from `montrs-platform` for `Target::Mobile`.

## 5. When a user should reach for this package
- Targeting mobile devices from a MontRS app.

## 6. Deeper Documentation
- [Invariants](docs/invariants.md)