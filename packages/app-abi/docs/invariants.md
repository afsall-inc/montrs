# App-ABI Package Invariants

## 1. Responsibility
`montrs-app-abi` defines the stable C ABI between the MontRS dev shell and a
hot-swappable application `cdylib`, plus the `export_app!` macro that emits the
entry symbol in an app library.

## 2. Invariants
- **C types only**: everything crossing the boundary is `#[repr(C)]` and
  FFI-safe. No Rust or framework types cross it, so the app and shell rebuild
  independently.
- **Dependency-free**: this crate has no dependencies (`allowed_deps = []`) so
  the app `cdylib` stays light and fast to rebuild.
- **Versioned**: any layout or semantic change bumps `ABI_VERSION`; the shell
  refuses a mismatch.
- **Fixed entry symbol**: apps export `montrs_app_entry`.

## 3. Boundary Definitions
- **In-Scope**: the vtable, request/response types, the entry symbol, and the
  `export_app!` macro.
- **Out-of-Scope**: HTTP serving, loading/swapping libraries, watching files.

## 4. Agent Guidelines
- `export_app!` expands in the app crate and references `montrs-core` at the
  call site; do not make this crate depend on it.
