# montrs-ui

UI component library for MontRS — Tailwind CSS macros, shadcn-inspired theming, and type-safe component system.

## Installation

```bash
montrs ui init
```

Or with `montrs new`:
```bash
montrs new my-app --ui
```

## Theming

MontRS UI uses a shadcn-inspired theming system. The `components.json` file at your project root controls the theme:

```json
{
  "$schema": "https://montrs.com/schema.json",
  "style": "default",
  "tailwind": {
    "css": "style/main.css",
    "toml": "tailwind.toml",
    "base_color": "neutral",
    "css_variables": true
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui"
  },
  "icon_library": "montrs"
}
```

### Base Colors

| Name | Description |
|------|-------------|
| `neutral` | Pure gray, no hue shift |
| `stone` | Warm, yellowish-gray |
| `zinc` | Cool, bluish-gray |
| `mauve` | Purple-ish gray |
| `olive` | Green-ish gray |
| `mist` | Blue-ish gray |
| `taupe` | Warm brown-gray |

### Accent Colors

amber, blue, cyan, emerald, green, indigo, lime, orange, pink, purple, red, rose, sky, teal, violet, yellow

## Usage

```rust
use montrs_ui::prelude::*;

// Theme provider wraps your app
#[component]
fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <Header />
            <main>
                <RouterOutlet<MyConfig> />
            </main>
        </ThemeProvider>
    }
}

// Dark mode toggle
let theme = use_theme();
toggle_theme(); // light -> dark -> system -> light
```

### Macros

```rust
use montrs_ui::prelude::*;

// cn!() — merge Tailwind classes (like shadcn's cn())
cn!("px-4 py-2", "bg-red-500")
cn!("px-4", Some("bg-red-500"))
cn!("px-4", cond.then_some("text-sm"))

// clx!() — create a component with base classes
clx! {Card, div, "rounded-lg p-4", "bg-sky-500"}

// void!() — self-closing component
void! {MyInput, input, "px-3 py-2 border rounded"}

// variants!() — type-safe variant/size components
variants! {
    Badge {
        base: "inline-flex items-center font-semibold rounded-md",
        variants: {
            variant: {
                Default: "bg-primary text-primary-foreground",
                Secondary: "bg-secondary text-secondary-foreground",
            },
            size: {
                Default: "px-2.5 py-0.5 text-xs",
                Sm: "px-1.5 py-0.5 text-[10px]",
            }
        },
        component: {
            element: span
        }
    }
}
```

## Dependencies

- `montrs-icons` (re-exported for convenience)
- `tw_merge` — Tailwind CSS class merging
- `leptos` — Reactive UI framework