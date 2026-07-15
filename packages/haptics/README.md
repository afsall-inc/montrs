# montrs-haptics

Cross-platform haptic feedback for web, desktop, and mobile.

## Usage
```rust
use montrs_haptics::{HapticsProvider, HapticsConfig, HapticsTarget, ImpactStyle};

let config = HapticsConfig {
    enabled: true,
    target: HapticsTarget::Desktop,
};
let provider = montrs_haptics::create_haptics_provider(&config);
if provider.is_supported() {
    provider.impact(ImpactStyle::Medium);
    provider.selection_changed();
}
```

## Features
- `web`: Web Vibration API via wasm-bindgen
- `desktop`: OS-native APIs (temporary until montrs-desktop engine)
- `mobile`: Platform-native bridge stubs

## Architecture

See `docs/haptics/overview.md` for the full design rationale, target detection flow,
and migration path when the desktop engine (Task 5) lands.
