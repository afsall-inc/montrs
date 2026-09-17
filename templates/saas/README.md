# MontRS SaaS Template

Full SaaS starter: auth, organizations, admin, API keys, services.

## Getting started

```bash
montrs services start     # start postgres + redis + api
montrs run seed           # seed data
montrs serve              # run API + web
```

## Auth plugins

- Email/password
- Two-factor (TOTP)
- Organizations + RBAC
- Admin panel
- API keys

## Hot reload

`montrs serve` hot-reloads `view!` and CSS changes. The `[profile.hot]` profile
in `Cargo.toml` (release optimization plus `debug-assertions`) and the pinned
`LEPTOS_WATCH` in `.cargo/config.toml` keep the SSR server and WASM client
emitting matching hot-reload markers — keep both files intact.