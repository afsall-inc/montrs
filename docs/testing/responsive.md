# Responsive Testing (Simulated Devices)

MontRS treats responsiveness as a first-class, testable invariant. Rather than
hoping a layout works across devices, you can **prove** it in-process — down to
the pixel — without a browser.

The responsive engine lives in `montrs-test` behind the `layout` feature.

---

## 1. Breakpoints and Tailwind variants

`SimLayout` evaluates Tailwind v4 responsive variants against the viewport:

| Variant | Applies when |
|---------|--------------|
| `sm:` `md:` `lg:` `xl:` `2xl:` | width ≥ 640 / 768 / 1024 / 1280 / 1536px |
| `max-md:` etc. | width < breakpoint |
| `min-[600px]:` / `max-[600px]:` | arbitrary pixel bounds |
| `@md:` / `@[600px]:` | container query, uses a declared container width |

Override breakpoints for custom design systems:

```rust
use montrs_test::prelude::*;

let check = ResponsiveCheck::new(html)
    .with_breakpoints(Breakpoints { lg: 1100.0, ..Default::default() });
```

---

## 2. Simulated devices

The `devices` module ships a catalog covering phones, tablets, laptops, and
desktops, plus rotation and custom profiles.

```rust
use montrs_test::prelude::*;

for device in standard_devices() {
    println!("{} {}×{} @{}x",
        device.name, device.width, device.height, device.dpr);
}

// Build your own
let custom = DeviceProfile::custom("kiosk", "Kiosk", 1080.0, 1920.0);
```

`DeviceProfile::from_observed(width, height, dpr)` captures a real client so a
developer's own device can be tested and recorded.

---

## 3. Exact-pixel overflow detection

`ResponsiveCheck` finds the **exact width** at which layout breaks:

```rust
let check = ResponsiveCheck::new(html);

// Every violation across a pixel-exact sweep
for report in check.sweep(320.0..=1920.0, 1.0) {
    println!("{}", report.render());
}

// Fails with the exact offending width, rule, element, and pixel delta
check.assert_no_irresponsive_px(320.0..=1280.0);

// The smallest width at which the layout is clean
if let Some(w) = check.required_width(320.0..=1920.0) {
    println!("requires at least {w}px");
}
```

### Rules

Enabled by default:

- **Horizontal overflow** — a box's right edge exceeds the viewport.
- **Off-screen** — a box starts left of the viewport.
- **Clipped text** — text wider than a clipping box (estimated).

Opt-in (opinionated accessibility policy):

- **Tap target** — a `button`/`input` smaller than 44×44 px.

```rust
ResponsiveCheck::new(html).with_tap_targets().assert_responsive();
```

---

## 4. Ready-made presets

The harness bakes in a standard device matrix:

```rust
let harness = TestHarness::new(build_spec());
let defaults = harness.responsive_defaults();
defaults.assert_responsive(view.html());
```

---

## 5. `montrs-ui` is responsive by default

Core `montrs-ui` components are verified responsive out of the box; see
`packages/ui/tests/responsive.rs`, which renders each component and asserts
`assert_responsive()` across the standard device matrix in CI.

---

## 6. Documented limits

- Text metrics are a deterministic approximation, not a real font. Pixel-exact
  text flow will be provided by the future CDP backend.
- `SimLayout` models a documented subset of inline styles and Tailwind
  utilities (display, flex, gap, padding, margin, width/height, min/max).
