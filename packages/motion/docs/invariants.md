# montrs-motion Invariants

## What It Enforces
- All animation primitives are pure Rust
- Spring physics uses Newton's method for accurate duration-based springs
- Easing functions match standard CSS easing curves exactly
- `MotionValue` is a reactive wrapper around Leptos signals with velocity tracking
- GPU acceleration is opt-in via the `gpu` feature flag (requires `montrs-renderer`)
- Video creation is opt-in via the `video` feature flag

## Rules
- `Spring::solve(t)` must be monotonic for fixed `t` values
- `Easing::apply(0.0)` must return `0.0` and `Easing::apply(1.0)` must return `1.0`
- `MotionValue` must track velocity accurately for gesture-based interactions
- `FrameLoop` must not block the main thread
- Gesture hooks must work with Leptos event system
- SVG path animation helpers must produce valid CSS `stroke-dasharray`/`stroke-dashoffset` values

## Boundary
- **In-Scope**: Spring physics, tweening, keyframes, reactive values, frame loop, gestures, SVG animation, CSS transitions, animated components, video creation
- **Out-of-Scope**: Layout animation (like Framer Motion's `layout` prop), drag-to-reorder, 3D transforms, physics engine (like Box2D)