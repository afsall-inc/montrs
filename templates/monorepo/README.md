# MontRS Workspace Template

A flexible monorepo structure for building scalable web applications with [MontRS](https://github.com/afsall-labs/montrs) and [Leptos](https://github.com/leptos-rs/leptos).

## Structure

```
my-workspace/
├── apps/           # Your applications
│   └── web/        # Main web app (Leptos)
├── packages/       # Shared libraries
│   └── ui/         # Shared UI components
├── Cargo.toml      # Workspace root
├── montrs.toml     # MontRS
 configuration
└── README.md
```

## Getting Started

```bash
# Development with hot-reload
montrs watch

# Production build
montrs build --release

# Run tests
montrs run test
```

## Adding New Apps/Packages

### Add a new app:
```bash
cd apps
cargo new my-new-app
```

### Add a new package:
```bash
cd packages
cargo new --lib my-shared-lib
```

Update `Cargo.toml` dependencies in your apps to use shared packages:
```toml
[dependencies]
ui = { path = "../packages/ui" }
```

## Tailwind Support

This workspace includes `tailwind-fuse` for type-safe Tailwind classes:
- `TwClass` and `TwVariant` macros
- `tw_merge!` for intelligent class merging
- VSCode intellisense via `.vscode/settings.json`

## Flexibility

This structure is **not prescriptive**. You can:
- Add more apps (mobile, CLI, etc.)
- Create any shared packages you need
- Organize however fits your project

## Hot Reload

`montrs watch` runs a hot-reloading dev loop: editing a `view! { ... }` block or
`style/main.css` updates the page in place. This needs two things that ship with
the template:

- `[profile.hot]` in the workspace `Cargo.toml` — the dev client profile
  (release optimization plus `debug-assertions`).
- `.cargo/config.toml`, which pins `LEPTOS_WATCH` so the SSR server and WASM
  client always emit matching hot-reload markers (cargo does not track that env
  var, so keeping it constant avoids mismatched builds and hydration failures).
