# Renderer Package Invariants

## 1. Responsibility
`montrs-renderer` provides the rendering pipeline for MontRS desktop and mobile applications. Supports software (tiny_skia), GPU (wgpu), and compositor backends.

## 2. Invariants
- **Backend Abstraction**: The `Renderer` trait must abstract over all backends.
- **Frame-Based**: All rendering produces `Frame` objects.
- **Feature-Gated**: Backends are behind feature flags (`tiny-skia`, `wgpu`, `compositor`).

## 3. Boundary Definitions
- **In-Scope**: 2D/3D rendering, text rendering, SVG rasterization, compositor integration.
- **Out-of-Scope**: Window creation, input handling, application logic.

## 4. Agent Guidelines
- Use the `Renderer` trait for backend-agnostic rendering.
- Call `begin_frame()` / `finish()` for each frame.