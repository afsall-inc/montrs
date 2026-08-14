# Renderer Package — Agent Guide

## Overview
`montrs-renderer` provides the rendering pipeline for MontRS desktop and mobile applications. Supports software (tiny_skia), GPU (wgpu), and compositor backends.

## Key Concepts
- **Renderer trait**: `begin_frame()`, `clear()`, `draw_quad()`, `draw_text()`, `finish()`.
- **Frame**: The output of a render pass — contains pixel data.
- **Backends**: Feature-gated (`tiny-skia`, `wgpu`, `compositor`).

## Agent Usage
- Use the `Renderer` trait for backend-agnostic rendering.
- Call `begin_frame()` before drawing, `finish()` after.
- Use `Viewport` for coordinate transformation.

## Local Invariants
Read `docs/invariants.md` before modifying.