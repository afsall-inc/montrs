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
| State | preserved | process preserved; in-app state resets (see below) |
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

Add the dylib entry in the app **library** (the templates already do this):

```rust
#[cfg(not(target_arch = "wasm32"))]
montrs_app_abi::export_app!(build_spec(), || leptos::prelude::view! { <Shell /> });
```

## Project setup (already done in templates)

- `[profile.hot]` in the root `Cargo.toml` — the client profile (`release` plus
  `debug-assertions`), so the WASM client and the SSR server emit matching
  hot-reload markers.
- `.cargo/config.toml` pinning `LEPTOS_WATCH` — `leptos` reads it at compile
  time and cargo does not track it, so it must be constant.
- `montrs-app-abi` as a native dependency — the ABI and `export_app!`.
- The `montrs-dev-shell` binary, built with the CLI (or on `PATH`).

## State resets on reload

Each swapped-in library gets fresh globals, so in-dylib app state (caches,
counters, open connections) is discarded on every reload. Requests are treated
as stateless, which is fine for typical server-rendered pages.

### Future: state-transfer hook

A later version can preserve long-lived state across reloads: add optional
`export_state() -> bytes` / `import_state(bytes)` hooks to the ABI, and have the
shell carry those bytes from the outgoing library to the incoming one. This is
worth doing because it keeps sessions and warm caches alive across edits,
turning every reload into a continuation instead of a reset — the difference
between "the server restarted" and "the code changed." It requires the app to
declare a serializable state type, so it stays opt-in.

## Limitations

- **Browser (WASM) Rust hot reload is not finished.** Client-side changes are
  covered by `view!`/CSS hot reload; Rust hot reload applies to the native dev
  server. The wasm patch path (jump tables, PIC thin link) is in progress.
- **Debug builds only.** This is a development feature and is never part of a
  production build.
- **State resets** on each reload (above).

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

- **WASM (browser) Rust hot reload.** The jump-table semantics are in place
  (`wasm_function_table_indices` / `build_wasm_jump_table`). Remaining: capture
  and replay changed wasm crates into objects, link a PIC patch module with
  `wasm-ld`, serve the patch `.wasm`, and apply it in-browser.

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
