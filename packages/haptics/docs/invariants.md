# Haptics Invariants

## What It Enforces
- Haptic feedback is abstracted behind the HapticsProvider trait
- Platform selection is done at runtime via HapticsTarget enum
- Each platform implementation is behind a Cargo feature gate

## Rules
- Always check is_supported() before calling vibrate()
- Never hardcode platform-specific haptic patterns in application logic
- Use ImpactStyle for standard interactions, not raw duration values
