# montrs-motion

Animation library for MontRS — spring physics, tweening, gestures, and video creation.

Inspired by [Motion](https://motion.dev) (Framer Motion) and [Remotion](https://remotion.dev).

## Usage

```rust
use montrs_motion::*;

// Spring animation
let spring = Spring::new(100.0, 10.0, 1.0);
let value = spring.solve(0.5);

// Tween with easing
let tween = Tween::new(0.0, 100.0, 1.0).with_easing(Easing::BounceOut);
let v = tween.sample(0.5);

// Reactive motion value
let mv = use_motion_value(0.0);
mv.animate_to(100.0, 300.0, 20.0, 1.0);

// Animated component
view! { <Animated hover_scale=1.05>
    <div>"Hover me"</div>
</Animated> }

// Video creation
let mut video = VideoComposition::new(1920, 1080, 30, 90);
for frame in 0..90 {
    video.add_frame(Frame::new(frame, svg_string))?;
}
video.render("output.mp4")?;
```

## API

| Module | Types | Description |
|--------|-------|-------------|
| `spring` | `Spring` | Physics-based spring (stiffness, damping, mass, velocity) |
| `tween` | `Tween`, `Easing` | Time-based interpolation with 20 easing functions |
| `keyframes` | `Keyframes`, `Extrapolate` | Multi-keyframe interpolation |
| `value` | `MotionValue` | Reactive animation value with velocity tracking |
| `frame` | `FrameLoop` | requestAnimationFrame scheduler |
| `gesture` | `use_hover()`, `use_press()`, `use_pan()` | Gesture → Leptos signals |
| `svg` | `PathDrawAnimation`, `stroke_dash*` | SVG path animation helpers |
| `css` | `css_transition()`, `will_change()`, `gpu_transform()` | CSS transition builders |
| `animated` | `Animated` | Wrapper component (motion.div equivalent) |
| `video` | `VideoComposition`, `Frame` | Frame-by-frame video pipeline |

## Easing Functions

Linear, Ease, EaseIn, EaseOut, EaseInOut, QuadIn/Out/InOut, CubicIn/Out/InOut, SineIn/Out/InOut, BackIn/Out/InOut, ElasticIn/Out/InOut, BounceIn/Out/InOut, Bezier(x1,y1,x2,y2)

## Video

The video module requires the `video` feature flag. Frame rendering uses FFmpeg for final video stitching.

## GPU Acceleration

Use the `gpu` feature flag (requires `montrs-renderer`) for GPU-accelerated spring calculations and compositing. Without it, animations use CSS transitions with `will-change: transform, opacity` for browser-level GPU compositing.