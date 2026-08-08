# montrs-lockfile

Deterministic tool version locking for MontRS. Records pre-resolved tool versions, download URLs, checksums, and platform-specific info so installs are reproducible across machines.

## Features

- **Lockfile format**: TOML-based with `[[tools.name]]` arrays for multiple versions
- **Platform-specific info**: Per-platform download URLs, checksums, and binary names
- **Content hashing**: SHA256 for cache invalidation
- **File locking**: `fslock`-based blocking and non-blocking locks
- **Write/read/parse**: Full round-trip support with generated header annotation

## Usage

```rust
use montrs_lockfile::{MontrsLock, LockfileTool, write_lockfile, read_lockfile, lockfile_path_for_root};

let mut lock = MontrsLock::new();
lock.set_tool("rust", LockfileTool {
    version: "1.84.0".to_string(),
    backend: Some("core:rust".to_string()),
    options: BTreeMap::new(),
    platforms: BTreeMap::new(),
});
write_lockfile(&lockfile_path_for_root(&project_root), &lock)?;
let read = read_lockfile(&lockfile_path_for_root(&project_root))?;
```