# montrs-renderer

Low-level rendering abstractions for MontRS.

**Target Audiences:** Framework Contributors.

## 1. What this package is
`montrs-renderer` defines the `Renderer` trait and the geometry primitives (`Rect`, `Point`, `Color`, `Path`, `Paint`, `Stroke`, `Quad`, `GlyphRun`, `Image`, `Svg`, `Frame`) used by rendering backends.

## 2. What problems it solves
- **Backend agnosticism**: A stable trait so wgpu and tiny-skia backends can be swapped.
- **Shared primitives**: Common types for 2D and text rendering.

## 3. What it intentionally does NOT do
- **Window management**: That's `montrs-desktop` / `montrs-mobile`.
- **High-level UI**: Components live in `montrs-ui`.

## 4. How it fits into the MontRS system
Layer 3, the interface consumed by desktop/mobile shells.

## 5. When a user should reach for this package
- Implementing a custom rendering backend.
- Working on the render pipeline.

## 6. Deeper Documentation
- [Invariants](docs/invariants.md)