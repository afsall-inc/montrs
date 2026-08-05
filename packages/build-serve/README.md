# MontRS Build-Serve

Dev server for MontRS projects.

Serves static files from the site root using `axum` + `tower-http`.

```rust
use montrs_build_serve::{serve_static, ServeConfig};
```