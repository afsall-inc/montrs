# Hotpatch Package Invariants

## 1. Responsibility
`montrs-hotpatch` is the application-facing hot-patch runtime. It exposes the
`serve!` macro, the render cutover, and the native client bootstrap so apps need
no hot-patch code of their own.

## 2. Invariants
- **Zero app-facing hot-patch code**: apps serve through `serve!`; they must not
  reference `subsecond` or the client directly.
- **No-op outside the dev server**: in release builds the cutover runs the
  closure unchanged, and the client starts only when the dev server exposes a
  socket (`MONTRS_HOTPATCH_ADDR`). The macro is safe to keep in shipped code.
- **Tip-crate expansion**: the macro must expand `subsecond::call` in the
  *calling* crate so the cutover's monomorphized code lands in the tip crate's
  objects — otherwise the patch cannot contain it and the redirect never hits.
- **Dependency boundary**: depends only on `montrs-core` and
  `montrs-dev-hotpatch`.

## 3. Boundary Definitions
- **In-Scope**: `serve!`, the cutover helper, the client bootstrap, the opt-in
  probe.
- **Out-of-Scope**: patch building, linking, capture, transport.

## 4. Agent Guidelines
- Keep the macro body minimal; anything hidden behind a helper function lands in
  this crate and is not patchable.
- `install_client_from_env` is idempotent and best-effort.
