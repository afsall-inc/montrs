# Agent Guide: montrs-motion

Agents can use this package to add animations to MontRS applications.

## Key Types

- **`Spring`** — Physics-based spring. `Spring::new(stiffness, damping, mass).solve(t)`
- **`Tween`** — Time-based interpolation. `Tween::new(from, to, duration).with_easing(Easing::EaseOut)`
- **`Keyframes`** — Multi-keyframe interpolation. `Keyframes::new(inputs, outputs).with_easings(...)`
- **`MotionValue`** — Reactive animation value. `use_motion_value(initial)`, `.animate_to(target, ...)`
- **`FrameLoop`** — rAF scheduler. `FrameLoop::on_frame(callback)`
- **`Easing`** — 20 easing functions. `Easing::EaseOut`, `Easing::BounceIn`, etc.
- **`Animated`** — Wrapper component. `<Animated hover_scale=1.05>...</Animated>`
- **`VideoComposition`** — Video creation. `VideoComposition::new(w, h, fps, duration)`

## Common Patterns

```rust
use montrs_motion::*;

// Spring animation
let spring = Spring::new(100.0, 10.0, 1.0).with_range(0.0, 100.0);
let value = spring.solve(0.5); // value at 0.5s

// Tween with easing
let tween = Tween::new(0.0, 1.0, 1.0).with_easing(Easing::BounceOut);
let v = tween.sample(0.75); // 0.75s into animation

// Reactive motion value
let mv = use_motion_value(0.0);
mv.animate_to(100.0, 300.0, 20.0, 1.0);

// Hover gesture
let (on_enter, on_leave, is_hovered) = use_hover();

// Animated component
view! { <Animated hover_scale=1.05 hover_opacity=0.9>
    <div>"Hover me"</div>
</Animated> }
```

## @agent-tool
- `Spring` — Spring physics simulation
- `Tween` — Time-based interpolation
- `MotionValue` — Reactive animation value
- `FrameLoop` — Frame loop scheduler
- `use_hover()`, `use_press()`, `use_pan()` — Gesture hooks
- `Animated` — Animated component wrapper
- `VideoComposition` — Video creation pipeline