# Agent Guide: montrs-icons

Agents can use this package to find, render, and manipulate icons in MontRS applications.

## Key Types

- **`Glyph`** — Enum of all 1600+ icons. Use `Glyph::by_name("Search")` or `Glyph::find("arrow")` to look up.
- **`Icon`** — Generic component. `<Icon glyph=Glyph::Search class="w-4 h-4" />`
- **`CustomIcon`** — For brand logos or custom SVG. `<CustomIcon svg=... />`
- **Per-icon components** — E.g. `<SearchIcon />`, `<HeartIcon />`, `<ArrowRightIcon />`

## Common Patterns

```rust
// Reactive icon based on state
<Icon glyph=Signal::derive(move || {
    if dark_mode.get() { Glyph::Moon } else { Glyph::Sun }
}) />

// Find icons by category
let nav_icons = Glyph::find("navigation");

// Get related icons
let related = Glyph::ArrowRight.related(5);
```

## @agent-tool
- `Icon` — Generic icon renderer
- `CustomIcon` — Arbitrary SVG renderer
- `Glyph` — Icon enum with search/lookup
- All per-icon convenience components (e.g. `SearchIcon`)