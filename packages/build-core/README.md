# MontRS Build-Core

Build pipeline trait and configuration for MontRS.

Defines the `BuildPipeline` trait that `build-watch` and `build-serve` depend on, avoiding a dependency on the concrete `Pipeline` implementation.

```rust
use montrs_build_core::BuildPipeline;
```