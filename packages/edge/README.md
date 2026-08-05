# MontRS Edge

Edge runtime adapter for MontRS. Provides `EdgeAdapter` (PlatformAdapter) and a lightweight `fetch`-compatible request handler.

```rust
use montrs_edge::{EdgeAdapter, handle_edge_request, EdgeRequest};
use montrs_platform::PlatformAdapter;

let adapter = EdgeAdapter::new();
assert!(adapter.target().is_web());
```