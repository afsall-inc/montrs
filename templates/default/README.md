# MontRS Default Template

This is a starter template for building web applications with [MontRS](https://github.com/afsall-labs/montrs) and [Leptos](https://github.com/leptos-rs/leptos).

## Tailwind Support

This template comes with full Tailwind CSS support powered by `montrs` and `tailwind-fuse`.

### tailwind-fuse Integration

We use [`tailwind-fuse`](https://github.com/gaucho-labs/tailwind-fuse) to provide type-safe Tailwind classes and intelligent class merging.

- **`TwClass`**: Macro to create components with variant props.
- **`TwVariant`**: Macro to define enum-based variants (like `primary`, `secondary`, `outline`).
- **`tw_merge!`**: Macro to merge classes while handling conflicts (e.g., `p-4` overrides `px-2`).

### Configuration Options

You can configure Tailwind in two ways:

1.  **Tailwind v4 (Recommended)**:
    - Use pure CSS for configuration in your input file (e.g., `style/input.css`).
    - No `tailwind.config.js` or `tailwind.toml` required.
    - `montrs` detects this mode automatically if no config file is present.

2.  **tailwind.toml**:
    - Create a `tailwind.toml` file in your project root for a pure-Rust configuration experience.
    - `montrs` automatically generates a temporary `tailwind.config.js` for you during build/watch.

    ```toml
    content = ["src/**/*.rs", "*.html"]
    
    [theme.extend.colors]
    brand = "#ff00ea"

    [merge]
    prefix = "tw-"
    separator = ":"
    ```

## Running the Project

```bash
montrs watch
```

## Hot Reload

`montrs serve` / `montrs watch` run a hot-reloading dev loop:

- Editing a `view! { ... }` block or `style/main.css` updates the page in place.
- The dev client is built with the `[profile.hot]` profile in `Cargo.toml`
  (release optimization plus `debug-assertions`), so it stays fast while still
  emitting the same hot-reload markers as the dev SSR server.
- `.cargo/config.toml` pins `LEPTOS_WATCH`, which `leptos` reads at compile time
  to decide whether to emit those markers. Keep both `[profile.hot]` and that
  file: without them the SSR and WASM builds disagree and hydration fails.
