# MontRS Website

The official MontRS website at [montrs.com](https://montrs.com).

Built with MontRS, `montrs-ui`, `montrs-icons`, and Tailwind CSS.

## Development

```bash
# Serve with hot-reload
montrs serve

# Or from the apps/website directory
cd apps/website
montrs serve
```

## Build

```bash
montrs build --release
```

## Structure

```
apps/website/
├── app/src/
│   ├── lib.rs          # App entry, AppConfig, main()
│   ├── routes.rs       # MontRS Route impls & WebsitePlate
│   ├── pages/
│   │   ├── home.rs     # Landing page
│   │   ├── icons.rs    # Icon showcase with search
│   │   ├── components.rs  # UI component showcase
│   │   └── blocks.rs   # Blocks gallery
│   ├── components/
│   │   ├── header.rs   # Site header
│   │   └── footer.rs   # Site footer
│   └── blocks/
│       ├── login.rs    # Login form blocks
│       ├── headers.rs  # Header blocks
│       ├── footers.rs  # Footer blocks
│       └── faq.rs      # FAQ blocks
├── server/src/main.rs  # Axum SSR server
├── style/main.css      # Tailwind CSS with theme variables
├── tailwind.toml       # Tailwind config (no JS!)
├── components.json     # UI theme config
└── Cargo.toml          # Workspace member
```

## Architecture

The website uses MontRS patterns:

- **Plates**: `WebsitePlate` registers all routes
- **Routes**: Each page is a `view_route!` with a `RouteView`
- **Router**: `RouterOutlet<MyConfig>` renders the matched page
- **AppSpec**: `AppSpec::new().with_plate().mount_with_router()`

No Leptos Router imports in application code — the MontRS Router wraps it internally.