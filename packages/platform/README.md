# MontRS Platform

Layer-0 platform abstraction for MontRS.

Provides `Target` enum, `PlatformAdapter` trait, and no-op adapter for environments without native platform support.

```rust
use montrs_platform::{Target, PlatformAdapter, NoopPlatformAdapter};

let adapter = NoopPlatformAdapter::new(Target::Server);
assert_eq!(adapter.target(), Target::Server);
```