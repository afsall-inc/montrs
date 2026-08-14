# montrs-icons

Lucide icons for MontRS applications. 1600+ icons grouped into 42 category features for selective compilation.

## Usage

```rust
// Preferred: per-icon convenience components
use montrs_icons::SearchIcon;
view! { <SearchIcon class="text-blue-500" /> }

// Generic component with the glyph! macro
use montrs_icons::{glyph, Icon};
view! { <Icon glyph=glyph!(Search) /> }

// Full path (verbose)
use montrs_icons::{Icon, Glyph};
view! { <Icon glyph=Glyph::Search /> }

// With Tailwind classes
view! { <Icon glyph=glyph!(Heart) class="text-red-500" size="32" /> }

// Custom inline SVG
use montrs_icons::CustomIcon;
view! { <CustomIcon svg=r#"<path d="M12 2L2 7l10 5 10-5-10-5z" />"# /> }
```

## Features

Select only the categories you need:

```toml
montrs-icons = { path = "../icons", default-features = false, features = ["arrows", "navigation", "design"] }
```

Available categories: accessibility, account, animals, arrows, buildings, charts, communication, connectivity, cursors, design, development, devices, emoji, files, finance, food_beverage, gaming, home, layout, mail, math, medical, multimedia, nature, navigation, notifications, people, photography, science, seasons, security, shapes, shopping, social, sports, sustainability, text, time, tools, transportation, travel, weather.

## API

| Component | Description |
|-----------|-------------|
| `Icon` | Generic icon component (`glyph`, `class`, `size`, `fill`, `stroke`, `stroke_width`) |
| `CustomIcon` | Render arbitrary SVG inline (`svg`, `class`, `size`, `fill`, `stroke`, `stroke_width`) |
| `SearchIcon` | Per-icon convenience component (one per icon, e.g. `HeartIcon`, `ArrowRightIcon`) |
| `glyph!` | Macro — shorthand for `Glyph::Search` → `glyph!(Search)` |

| Glyph method | Returns | Description |
|-------------|---------|-------------|
| `svg()` | `&'static str` | Inner SVG markup |
| `name()` | `&'static str` | PascalCase name |
| `kebab_name()` | `String` | kebab-case name |
| `by_name(name)` | `Option<Glyph>` | Lookup by PascalCase or kebab-case |
| `find(filter)` | `Vec<Glyph>` | Search by name, tag, or category |
| `count()` | `usize` | Total icon count |
| `related(limit)` | `Vec<Glyph>` | Related icons by tag overlap |
| `tags()` | iterator | Icon tags |
| `categories()` | iterator | Icon categories |
| `all_categories()` | `&BTreeMap<String, u16>` | All categories with counts |