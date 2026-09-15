# MontRS Hotpatch

Runtime facade that gives applications Rust hot-patching with no hot-patch code
of their own.

Serve an app through the `serve!` macro; it installs the native patch client and
wraps the root render in a `subsecond` cutover. A no-op in release builds, and
active only when `montrs serve` exposes a hot-patch socket.

```rust
montrs_hotpatch::serve!(
    spec.router,
    || view! { <Shell /> }
)
.unwrap();
```

Enable it with `[serve] hotpatch = true` in `montrs.toml`. See
[docs/tooling/hot-reload.md](../../docs/tooling/hot-reload.md).
