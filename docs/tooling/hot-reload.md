# Hot Reload and Hot Patching

MontRS has a live development loop. This page explains what each part does, how
to turn it on, how it works internally, and what its limits are.

## Hot reload vs hot patch

- **Hot reload** updates what the app *renders* or *looks like* while it runs
  (CSS/assets and `view!` markup). It needs no Rust recompile.
- **Hot patch** replaces the app's *compiled Rust code* in the running process.
  The app keeps running — no restart, no lost connection.

| | Hot reload (CSS / `view!`) | Rust hot reload |
|---|---|---|
| Changes | markup, attributes, text, styles | function bodies, branches, constants, logic |
| Mechanism | diff source → patch the DOM / swap CSS | rebuild the app library → swap it into the running shell |
| Cargo rebuild | no | yes (incremental) |
| Reaches | the browser DOM, across all crates | the app + its workspace crates |
| State | preserved | process and registered in-memory state preserved (see below) |
| Latency | instant (ms) | ~seconds |
| Status | **on by default with `montrs serve`** | **opt-in** |

## Turning on Rust hot reload

`view!` and CSS hot reload are always active during `montrs serve`/`montrs watch`.
Nothing to configure.

Rust hot reload is opt-in. Enable it in `montrs.toml`:

```toml
[serve]
hotpatch = true
```

or per session with:

```bash
MONTRS_HOTPATCH=1 montrs serve
```

## How it works (dylib swap)

The app library is the hot-reload boundary:

1. **Build as a dylib.** With hot reload on, the app library is built as a
   `cdylib` that exports `montrs_app_entry` (created by
   `montrs_app_abi::export_app!`) behind a stable `repr(C)` vtable. Only plain C
   types cross the boundary.
2. **Host it.** A generic shell (`montrs-dev-shell`) owns the HTTP listener and
   renders each request by calling into the dylib.
3. **Swap on change.** On an edit the CLI rebuilds the library, copies it to a
   fresh filename, and points the shell's reload file at it. The shell loads the
   new library and swaps the vtable atomically — the listener, and the browser
   connection, never restart. Old libraries are leaked (Windows cannot safely
   unload a loaded DLL), bounded by reloads in a session.

This covers edits in the app crate and its **workspace library crates**
(`lib.rs`, `packages/*`), because the whole library is rebuilt and swapped.

## How to write the app

The normal entry (used when hot reload is off) is unchanged:

```rust
montrs_hotpatch::serve!(spec.router, || leptos::prelude::view! { <Shell /> });
```

For hot reload, add two things to the app **library**: a `mod hotpatch;` and the
dylib entry. `hotpatch.rs` is the convention file the framework wires
automatically:

```rust
mod hotpatch;

#[cfg(not(target_arch = "wasm32"))]
montrs_app_abi::export_app_with_hotpatch!(
    build_spec(),
    || leptos::prelude::view! { <Shell /> }
);
```

`hotpatch.rs` is small and nearly identical across apps — it holds the state
that should survive a reload and an optional per-request hook:

```rust
use montrs_app_abi::state;

static HITS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn before_render() { /* called before each render */ }
pub fn export_state() -> Vec<u8> { state::export() }
pub fn import_state(bytes: &[u8]) { state::import(bytes) }
```

Register any long-lived store with `state::register(name, load, save)` (see
[State across reloads](#state-across-reloads)). Prefer `state.rs` (or any other
file) for your own state management — `hotpatch.rs` is only the bridge. If you
want the hooks somewhere else, use the explicit-path form:

```rust
montrs_app_abi::export_app_with_state!(
    build_spec(),
    || leptos::prelude::view! { <Shell /> },
    crate::state::export_state,
    crate::state::import_state,
);
```

`export_app!` (no state) is still available.

## Project setup (already done in templates)

- `[profile.hot]` in the root `Cargo.toml` — the client profile (`release` plus
  `debug-assertions`), so the WASM client and the SSR server emit matching
  hot-reload markers.
- `.cargo/config.toml` pinning `LEPTOS_WATCH` — `leptos` reads it at compile
  time and cargo does not track it, so it must be constant.
- `montrs-app-abi` as a dependency — the ABI, `export_app_with_hotpatch!`, and
  the `state` registry.
- The `montrs-dev-shell` binary, built with the CLI (or on `PATH`).

## State across reloads

Each swapped-in library gets fresh globals, so any state the app kept **in
memory** would reset on a reload. MontRS preserves it: before swapping, the
shell asks the outgoing library for `export_state()`, and after loading the new
one, hands it back via `import_state()`.

Register each store in `hotpatch.rs` with `montrs_app_abi::state::register`:

```rust
state::register(
    "hits",
    || HITS.load(Ordering::Relaxed).to_le_bytes().to_vec(),
    |bytes| { /* restore */ },
);
```

The registry serializes all registered stores into one framed blob, so the glue
is the same for every app. Import is best-effort: an unknown name or a blob that
no longer matches the new code is ignored, so a schema change falls back to
fresh state instead of failing the reload. This keeps sessions and warm caches
alive across edits — a reload becomes a continuation rather than a reset.

State that lives outside the process (a real database, Redis, files) is
unaffected and needs no registration.

## Limitations

- **Browser (WASM) Rust hot reload is not finished.** Client-side changes are
  covered by `view!`/CSS hot reload; Rust hot reload applies to the native dev
  server. The wasm patch path (jump tables, PIC thin link) is in progress.
- **Debug builds only.** This is a development feature and is never part of a
  production build.
- **State transfer is opt-in.** State kept in memory survives a reload only if
  it is registered in `hotpatch.rs`; unregistered globals reset.

## Environment variables

| Variable | Purpose |
|---|---|
| `MONTRS_HOTPATCH` | Enable Rust hot reload (same as `[serve] hotpatch`) |
| `MONTRS_APP_DYLIB` | App library path; set on the shell by `serve` |
| `MONTRS_RELOAD_FILE` | File whose contents name the current app library; the shell reloads when it changes |
| `MONTRS_DEV_SHELL_PATH` | Override the `montrs-dev-shell` binary path |

## Crates

- `montrs-app-abi` — the stable C ABI and `export_app!`.
- `montrs-dev-shell` — the generic shell: listener, loader, swap.
- `montrs-hot-reload` — `view!` diffing and markup patches.
- `montrs-hotpatch` — app-facing facade (`serve!`).
- `montrs-dev-hotpatch` — capture/link/patch tooling; the base for the wasm path.

## Roadmap

- **WASM (browser) Rust hot reload remains unsupported.** Capture, object
  selection, patch linking, and broadcast wiring exist, but end-to-end patch
  application is not verified. The dev shell now proxies `/_dioxus` upgrades
  directly to the hotpatch hub rather than through the request-rendering ABI.
  Transport tests do not establish browser state preservation.
- **PIC compilation is a confirmed blocker.** Linking the website's captured
  WASM objects with the current shared-module flags fails with
  `R_WASM_TABLE_INDEX_SLEB` and `R_WASM_MEMORY_ADDR_SLEB` relocation errors
  requesting recompilation with PIC. Linker flags alone are insufficient.
- **Host compatibility still needs validation.** Captured objects reference
  pre-bindgen imports, while the browser loads the transformed `front_bg.wasm`.
  Patch generation must retain the running browser's baseline table rather than
  using the newly rebuilt bundle, resolve imports against that host, and avoid
  the full-page reload notifications currently sent after a rebuild.

## Troubleshooting

- **Hydration panic (`failed_to_cast_element`).** The SSR and WASM builds emitted
  different hot-reload markers. Ensure `.cargo/config.toml` pins `LEPTOS_WATCH`
  and that `[profile.hot]` exists; after changing either, a one-time
  `cargo clean -p leptos` is needed because cargo does not track the env var.
- **`montrs-dev-shell not found`.** Build it with
  `cargo build -p montrs-dev-shell`, or set `MONTRS_DEV_SHELL_PATH`.
- **`app cdylib not found`.** The app library must call
  `montrs_app_abi::export_app!`; the dylib is named after the package
  (`<package>.dll`).
- **An edit didn't show up.** Check the shell log for `reloaded app dylib …`.
  Changes to the SSR bin/shell restart the server; changes to the app library
  reload in place.
