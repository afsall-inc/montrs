# Image Core Package Invariants

## Responsibility
`montrs-image-core` defines validated, serializable image requests shared by UI and server implementations.

## Invariants
- Image specifications are deterministic and serializable.
- Dimensions, quality, and source paths are validated before processing.
- Source paths must remain under the configured image root.
- The core contains no filesystem decoding, HTTP serving, DOM, or renderer logic.
