# Haptics Package — Agent Guide

## Overview
`montrs-haptics` provides cross-platform haptic feedback for MontRS applications. Supports web (Navigator.vibrate), desktop (SDL2), and mobile (Android/iOS native) backends.

## Key Concepts
- **HapticsProvider trait**: Platform-specific haptic feedback implementation.
- **ImpactStyle**: `Light`, `Medium`, `Heavy`, `Selection`, `Notification`.
- **NoopHapticsProvider**: Fallback when haptics are disabled or unsupported.

## Agent Usage
- Use `HapticsProvider::impact(ImpactStyle)` to trigger haptic feedback.
- Use `HapticsProvider::notification()` for notification feedback.
- Configure via `HapticsConfig`.

## Local Invariants
Read `docs/invariants.md` before modifying.