# MontRS Web

Web platform adapter for MontRS. Implements `PlatformAdapter` from `montrs-platform` for browser/WASM targets.

```rust
use montrs_web::WebAdapter;
use montrs_platform::PlatformAdapter;

let adapter = WebAdapter::new();
adapter.set_title("My App");
```