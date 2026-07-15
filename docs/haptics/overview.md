# Haptics Architecture

MontRS haptics provides cross-platform tactile feedback through a unified `HapticsProvider` trait. This document explains the design rationale, platform detection flow, fallback strategy, and migration path.

---

## Architecture

```
┌──────────┐     ┌──────────────────┐     ┌──────────────────────┐
│  App     │ ──▶ │  HapticsPlate    │ ──▶ │  HapticsProvider     │
│  /View   │     │  (dependency     │     │  (trait object)      │
│          │     │   injection)     │     │                      │
└──────────┘     └──────────────────┘     └──────────────────────┘
                                                   │
                          ┌────────────────────────┼────────────────────────┐
                          ▼                        ▼                        ▼
                  ┌──────────────┐        ┌──────────────┐        ┌──────────────┐
                  │  Web         │        │  Desktop     │        │  Mobile      │
                  │  Vibration   │        │  OS-native   │        │  (stubs)     │
                  │  API         │        │  calls       │        │              │
                  └──────────────┘        └──────────────┘        └──────────────┘
```

## Design Rationale

### Why a trait?

`HapticsProvider` is a trait so that:

1. **Pluggable**: Apps can swap providers at runtime (e.g., test mock vs. real device).
2. **Mockable**: Tests inject a `NoopHapticsProvider` or a recording mock.
3. **Detectable**: `is_supported()` lets callers gracefully degrade when haptics are unavailable.
4. **Platform-agnostic**: The trait knows nothing about `Navigator`, `Vibrator`, or `UIImpactFeedbackGenerator`.

### Why `HapticsConfig`?

The `create_haptics_provider()` factory accepts `&HapticsConfig` so that:

- `enabled: false` returns `NoopHapticsProvider` — zero overhead, no platform code runs.
- `target` selects the platform backend at runtime (not compile-time), enabling a single binary to run on multiple targets.

## Target Detection Flow

```
create_haptics_provider(&config)
         │
         ├── config.enabled == false ──▶ NoopHapticsProvider
         │
         └── match config.target
                  │
                  ├── HapticsTarget::Web ──▶ #[cfg(feature = "web")]
                  │                               ├── yes ──▶ WebHapticsProvider
                  │                               └── no  ──▶ DesktopHapticsProvider (fallback)
                  │
                  ├── HapticsTarget::Desktop ──▶ DesktopHapticsProvider
                  │
                  └── HapticsTarget::Mobile ──▶ MobileHapticsProvider
```

## Platform Status

| Target | Backend | Status | Implementation |
|--------|---------|--------|----------------|
| Web | Navigator.vibrate() | ✅ Complete | `web-sys` + `wasm-bindgen` |
| Desktop (Windows) | MessageBeep | 🔶 Temporary | WinAPI FFI in `desktop.rs` |
| Desktop (macOS) | NSHapticFeedbackManager | 🔶 Stub | Awaiting `montrs-desktop` engine |
| Desktop (Linux) | N/A | 🔶 No-op | No universal API without engine |
| Mobile (Android) | JNI bridge | 🔶 Stub | Awaiting `montrs-desktop` engine |
| Mobile (iOS) | UIImpactFeedbackGenerator | 🔶 Stub | Awaiting `montrs-desktop` engine |

### Expected Behavior per Target

- **Web**: Real haptic feedback via Vibration API. Works in Chrome/Firefox on mobile and supported desktop browsers. `impact()` maps to duration-based vibration.
- **Desktop**: Temporary OS-native calls (Windows beep) are *audible/tactile* approximations, not real haptics. Will be replaced by the `montrs-desktop` engine.
- **Mobile**: All operations are currently no-ops. The stubs document the native API to call (`Vibrator`, `UIImpactFeedbackGenerator`).

## Fallback Strategy

1. **Call `is_supported()` first** — always check before calling haptic operations.
2. **`NoopHapticsProvider`** — returned when `enabled: false`; all methods are no-ops, `is_supported()` returns `false`.
3. **Graceful degradation** — if haptics are unavailable, the app should continue silently (vibration is a UX enhancement, not a requirement).

## Migration Path

When `montrs-desktop` (Task 5) lands:

1. `DesktopHapticsProvider::vibrate()` → delegates to `DesktopEngine::haptic_event()`
2. The engine dispatches to platform-specific backends (DirectInput on Windows, CoreHaptics on macOS, evdev on Linux)
3. The raw FFI (`MessageBeep`) and fallbacks are removed
4. `MobileHapticsProvider` gains real JNI/FFI implementations targeting the engine's mobile runtime

## Feature Gates

| Feature | Enables |
|---------|---------|
| `web` | `WebHapticsProvider` (wasm-bindgen + web-sys) |
| `desktop` | OS-native desktop calls (currently no extra deps) |
| `mobile` | Mobile stubs (no native deps yet) |

Forwarded through the `montrs` facade as `haptics-web`, `haptics-desktop`, `haptics-mobile`.

## Casing & Style

- Types: `PascalCase` (e.g., `ImpactStyle::Light`)
- Functions: `snake_case` (e.g., `create_haptics_provider`)
- Constants: `SCREAMING_SNAKE_CASE`
- Files: `kebab-case` (e.g., `desktop.rs`, `invariants.md`)
