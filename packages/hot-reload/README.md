# MontRS Hot-Reload

`view!` hot reload for MontRS.

Parses `view!` macros out of Rust source and diffs them against a baseline to
produce markup patches the dev overlay applies to the live DOM without a
rebuild.

```rust
use montrs_hot_reload::ViewPatcher;
```

See [docs/tooling/hot-reload.md](../../docs/tooling/hot-reload.md).
