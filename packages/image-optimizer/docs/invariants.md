# Image Optimizer Package Invariants

## Responsibility
`montrs-image-optimizer` validates image requests and owns bounded server-side optimization policy.

## Invariants
- Every request is validated by `montrs-image-core`.
- Source files must be regular files under the configured root.
- File-size and dimension limits are mandatory.
- Cache names are filesystem-safe and deterministic.
- HTTP routing and UI rendering remain outside this package.
