# Agent Guide: montrs-image-optimizer

## Core Concepts
Bounded server-side image optimization policy.

### OptimizerConfig
- `root` — allowed source directory.
- `max_dimension` — maximum width/height in pixels.
- `max_file_size` — maximum source file size in bytes.

### Key Functions
- `validate_spec(spec)` — validates the spec against config, checks file existence and size.
- `cache_path(spec, cache_root)` — deterministic filesystem-safe cache path.

## Important Rules
- Every request is validated by `montrs-image-core`.
- Source files must be regular files under the configured root.
- File-size and dimension limits are mandatory.
- HTTP routing and UI rendering remain outside this package.