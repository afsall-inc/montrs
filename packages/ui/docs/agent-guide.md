# UI Package — Agent Guide

## Overview
`montrs-ui` provides pre-built Leptos components for MontRS applications. Includes buttons, inputs, dialogs, navigation, and layout components.

## Key Concepts
- **Components**: Reusable Leptos components with Tailwind CSS styling.
- **Variants**: Component styling variants defined via the `variants!` macro.
- **ThemeProvider**: Dark/light/system theme context provider.

## Agent Usage
- Import components from `montrs_ui::prelude::*`.
- Use `ThemeProvider` at the app root for theme support.
- Follow the `cn!` class merging pattern for custom styling.

## Local Invariants
Read `docs/invariants.md` before modifying.