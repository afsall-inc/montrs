# Mobile Package — Agent Guide

## Overview
`montrs-mobile` provides the mobile platform shell for MontRS. It implements `PlatformAdapter` from `montrs-platform` for Android and iOS targets.

## Key Concepts
- **MobileAdapter**: Implements `PlatformAdapter` for mobile targets.
- **Platform-Specific**: Behavior differs between Android and iOS via `Target::MobileAndroid` / `Target::MobileIos`.

## Agent Usage
- Use `MobileAdapter::new(target)` to create the adapter.
- `open_url` opens URLs via Android Intents or iOS `UIApplication`.

## Local Invariants
Read `docs/invariants.md` before modifying.