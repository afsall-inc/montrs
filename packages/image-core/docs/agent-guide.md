# Agent Guide: montrs-image-core

## Core Concepts
Validated, serializable image request specifications.

### ImageSpec
- `source` — image path relative to root.
- `width`, `height` — optional target dimensions.
- `quality` — 1–100 (default 80).
- `format` — `Original`, `Webp`, `Png`, `Jpeg`.

### Key Functions
- `validate(max_dimension)` — validates dimensions, quality, and path safety.
- `cache_key()` — deterministic string for cache lookups.
- `resolve_under(root)` — resolves the source path under a root, rejecting absolute paths and `..`.

## Important Rules
- No filesystem, HTTP, or rendering logic.
- Paths are validated against directory traversal.
- Dimensions and quality are bounded at validation time.