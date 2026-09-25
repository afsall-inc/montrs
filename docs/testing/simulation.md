# Agent Vision & 3D Simulation

MontRS lets AI agents reason about UI layouts, accessibility trees, visual
effects, and 3D scenes **by code** — without needing `computer-use` tools,
screen captures, or a GPU.

Everything in this module is pure Rust and runs in-process.

---

## 1. Agent vision (`montrs-test` feature `vision`)

`AgentVision::observe` packages accessibility, laid-out geometry, responsive
status, and a layout hash into a single JSON-serializable `Observation`:

```rust
use montrs_test::prelude::*;

let obs = AgentVision::observe(&html, &iphone_15());
obs.assert_responsive();
obs.assert_a11y("heading", "Dashboard");
obs.assert_a11y("button", "Save");

println!("{}", obs.to_json());
```

```json
{
  "device_id": "iphone-15",
  "width": 393.0,
  "height": 852.0,
  "a11y": [
    { "tag": "h1", "role": "heading", "name": "Dashboard" },
    { "tag": "button", "role": "button", "name": "Save" }
  ],
  "boxes": [
    { "tag": "h1", "class": null, "x": 0.0, "y": 0.0, "width": 393.0, "height": 36.0 }
  ],
  "violations": [],
  "layout_hash": 13917409214710293
}
```

An agent compares `layout_hash` across edits to detect unexpected layout drift.

---

## 2. 3D scene simulation (`montrs-test` feature `scene`)

Deterministic vector, matrix, projection, and bounding-box math with **no GPU**.
Agents can reason about 3D models and scenes numerically or verify projected
geometry.

```rust
use montrs_test::prelude::*;

let mut scene = Scene::new();
scene.add(Mesh::cube(2.0), Mat4::identity());

let camera = Camera::looking_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO);

// Assert the target projects to the center of an 800×600 viewport
let (x, y) = camera.project_pixel(Vec3::ZERO, 800, 600).unwrap();
assert!((x - 400.0).abs() < 1.0);
assert!((y - 300.0).abs() < 1.0);

// Rasterize a wireframe to an RGBA pixel buffer
let wireframe = scene.rasterize_wireframe(&camera, 128, 128);
assert!(wireframe.coverage() > 0.0);
```

`Wireframe::hash()` returns a deterministic 64-bit hash of the rasterized
output, suitable for snapshot regression testing.

---

## 3. CSS effect inspection

`EffectSim` inspects inline styles for visual effects that agents cannot
directly "see":

```rust
use montrs_test::prelude::*;

EffectSim::assert_has(&html, EffectKind::Shadow);
EffectSim::assert_has(&html, EffectKind::Gradient);
```

Recognises `Shadow`, `Blur`, `Gradient`, `Transform`, and `Opacity`.
