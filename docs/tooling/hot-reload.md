# Hot Reload and Hot Patching

MontRS has a live development loop. This page explains what each part does, how
to turn it on, how it works internally, and what its limits are.

## Hot reload vs hot patch

They are not the same thing:

- **Hot reload** updates what the app *renders* or *looks like* while it runs.
  It covers CSS/asset swaps and `view!` (markup) changes. It needs no Rust
  recompile.
- **Hot patch** replaces the app's *compiled Rust code* in the running process.
  It recompiles only what changed, links it into a shared library, and redirects
  function calls through a jump table at a cutover point. The process keeps
  running — no restart, no lost state.

| | Hot reload (CSS / `view!`) | Hot patch (Rust logic) |
|---|---|---|
| Changes | markup, attributes, text, styles | function bodies, branches, constants, logic |
| Mechanism | diff source → patch the DOM / swap CSS | recompile → thin-link a patch DLL → load → jump table |
| Cargo rebuild | no | yes (incremental rustc + link) |
| Reaches | the browser DOM, across all crates | the running process, **tip crate only** |
| State | preserved | preserved (same PID, in-flight state kept) |
| Latency | instant (ms) | ~seconds |
| Status | **on by default with `montrs serve`** | **experimental, opt-in** |

## Turning it on

`view!` and CSS hot reload are always active during `montrs serve`/`montrs watch`.
Nothing to configure.

Rust hot-patching is opt-in. Enable it in `montrs.toml`:

```toml
[serve]
hotpatch = true
```

or per session with environment variables:

```bash
MONTRS_HOTPATCH=1 MONTRS_HOTPATCH_PATCH=1 montrs serve
```

Both forms do the same thing: fat-link the server, build a patch on every Rust
edit, and deliver it to the running app.

## How to write the app

No hot-patch code belongs in an application. Serve the app through the
`montrs_hotpatch::serve!` macro (the templates and website already do). The
macro installs the native patch client, wraps the render in a hot-patch cutover,
and calls `montrs_core::serve::montrs_serve`:

```rust
#[cfg(feature = "ssr")]
fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let spec = website::build_spec();
    montrs_hotpatch::serve!(
        spec.router,
        || leptos::prelude::view! { <Shell /> }
    )
    .unwrap();
}
```

The root can be a closure or a named `fn`. A named `fn` is preferable for
hot-patching because it gives the cutover a stable symbol:

```rust
#[inline(never)]
fn root() -> impl leptos::prelude::IntoView {
    leptos::prelude::view! { <Shell /> }
}

montrs_hotpatch::serve!(spec.router, root).unwrap();
```

In a release build the cutover is a no-op, and the client only starts if the dev
server exposes a socket — so the macro is safe to keep in shipped code.

## How Rust hot-patching works

1. **Capture.** With hot-patching enabled, `montrs serve` sets
   `RUSTC_WORKSPACE_WRAPPER` to a shim that records each workspace crate's
   `rustc` invocation, and sets the platform linker to a shim that, for the
   **tip** link, links a "fat" binary: it exports `main`, disables high-entropy
   VA and incremental linking, forces a full PDB, and saves the tip's object
   files.
2. **Edit.** A source change triggers a rebuild. The server runs from a
   *sibling copy* (`app-ssr.run.exe`) so cargo can relink the real binary without
   hitting the Windows "running executable" lock — the process keeps serving.
3. **Patch.** The freshly linked tip objects plus generated *undefined-symbol
   stubs* are linked into a small shared library. A jump table maps each symbol's
   address in the running binary to its new address in the patch.
4. **Broadcast.** The dev server's hub sends the jump table over the hot-patch
   socket.
5. **Apply.** The native client in the running app calls
   `subsecond::apply_patch`, which loads the library and rebases the table by the
   ASLR slide.
6. **Cutover.** Every request re-enters the render through `subsecond::call`. If
   the cutover's function is in the table, the patched version runs; otherwise the
   original code runs.

## Project setup (already done in templates)

A hot-patchable app needs three things, all shipped in the templates:

- `[profile.hot]` in the root `Cargo.toml` — the client profile (`release`
  settings plus `debug-assertions`), so the WASM client and the SSR server emit
  matching hot-reload markers.
- `.cargo/config.toml` pinning `LEPTOS_WATCH` — the `leptos` crate reads it at
  compile time, and cargo does not track it, so it must be constant.
- `montrs-hotpatch` as a native dependency — the runtime facade and `serve!`.

## Status and rationale

Rust hot-patching is **experimental and opt-in** (`[serve] hotpatch = true`) and
stays that way until it can patch workspace crates, for two reasons:

- **Cost.** It fat-links the server and forces a large PDB (hundreds of MB), plus
  a relink and patch link per edit. Enabling it by default would slow every dev
  loop even when no Rust logic changed.
- **Surprise.** Only the tip crate is patchable, so a lib/dependency edit still
  needs a full rebuild. Defaulting it would make behavior look inconsistent.

View/CSS hot reload is always on and needs no flag. Revisit the default once
workspace-crate patching lands.

## Native workspace reload (dylib swap)

Tip-crate hot-patching covers edits in the bin crate. Native **workspace** edits
(a library or dependency crate) use dylib swap instead:

- the app library is built as a `cdylib` that exports `montrs_app_entry` (created
  by `montrs_hotpatch::export_app!`) behind the stable `montrs-app-abi` C ABI;
- a generic shell (`montrs-dev-shell`) owns the HTTP listener and renders each
  request by calling into the dylib;
- on a change the CLI rebuilds the dylib, copies it to a fresh filename, and
  POSTs its path to the shell, which loads it and swaps the vtable atomically.
  The listener never restarts, so the page keeps its connection.

Everything crossing the boundary is plain C types, so the app and shell are
rebuilt independently.

### State resets on reload

Each swapped-in dylib gets fresh globals, so in-dylib app state (caches,
counters, open connections) is discarded on every reload. v1 treats SSR requests
as stateless, which is fine for typical server-rendered pages.

### Future: state-transfer hook

A later version can preserve long-lived state across reloads: add optional
`export_state() -> bytes` / `import_state(bytes)` hooks to the ABI, and have the
shell carry those bytes from the outgoing dylib to the incoming one. This is
worth doing because it keeps sessions and warm caches alive across edits,
turning every reload into a continuation instead of a reset — the difference
between "the server restarted" and "the code changed." It requires the app to
declare a serializable state type, so it stays opt-in.

## Limitations

- **Tip crate only.** The patch contains only the crate with `main.rs`. Edits to
  a library or dependency crate (for example `lib.rs`) are *not* patched: the dev
  server detects this from the changed crates and falls back to a full rebuild +
  restart, so the change still appears (the server just restarts). Keep the code
  you iterate on in the bin crate to avoid the restart. An experimental
  `MONTRS_HOTPATCH_WORKSPACE=1` also links the changed workspace crates into the
  patch, but it does not reliably redirect calls that were inlined into the tip,
  so it is off by default and not yet supported.
- **Struct layout and statics.** Subsecond does not support hot-reloading structs
  that change layout, and globals/statics/thread-locals have caveats (renames look
  like new globals; static initializers do not re-run; thread-locals in the tip
  crate reset). Frameworks "re-instance" state to work around this; MontRS
  currently does not. On native, `ifunc_count` is `0` — the jump table is applied
  by loading the patch DLL and rebasing addresses; `ifunc_count` only matters for
  the (unimplemented) wasm path.
- **Debug builds only.** `subsecond::call` is compiled out when `debug_assertions`
  is off, and hot-patching is a development feature. Never ship it.
- **Cost.** Hot-patching fattens the server link, forces a large PDB (hundreds of
  MB), and adds a relink + patch link per edit. That is why it is off by default.

## Environment variables

| Variable | Purpose |
|---|---|
| `MONTRS_HOTPATCH` | Enable capture + fat link (same as `[serve] hotpatch`) |
| `MONTRS_HOTPATCH_PATCH` | Also build and broadcast a patch on each edit |
| `MONTRS_HOTPATCH_ADDR` | Address of the hub; set on the app by `serve` |
| `MONTRS_HOTPATCH_DIR` | Capture directory (default `target/montrs-hotpatch`) |
| `MONTRS_HOTPATCH_PROBE` | Log the cutover key and jump-table membership |
| `MONTRS_HOTPATCH_WORKSPACE` | Experimental: also link changed workspace crates into the patch (unreliable; off by default) |
| `MONTRS_REAL_LINKER` / `MONTRS_HOTPATCH_FLAVOR` | Real linker + flavour used by the shims |
| `MONTRS_HOTPATCH_TIP_OUT` / `MONTRS_HOTPATCH_WORKSPACE` | Tip binary + workspace target for the shims |

## Crates

- `montrs-hot-reload` — diffs `view!` macros and produces markup patches.
- `montrs-dev-hotpatch` — capture, fat link, patch linker, jump-table builder,
  the dev hub, the native client, and the wrapper/debug binaries.
- `montrs-hotpatch` — the runtime facade: `serve!`, the cutover, and the client
  bootstrap. This is the only crate an app references.
- `packages/cli` — the `serve`/`watch` supervisor that ties it together.

## Roadmap

- **Workspace-crate patching.** Experimental (`MONTRS_HOTPATCH_WORKSPACE=1`):
  the changed workspace crates' objects are linked into the patch, but calls
  inlined into the tip are not redirected, so it is off by default. Solving this
  needs codegen-level call-site routing.
- **WASM (browser) Rust hot-patching.** Not implemented. The jump-table
  foundation exists (`wasm_function_symbols`, `build_wasm_jump_table`), but the
  patch path is unfinished: it needs base-module preparation, a `wasm-ld` PIC
  thin link, and GOT/ifunc resolution before `subsecond::apply_patch` can
  instantiate a patch module in the browser. Until then, client-side changes are
  covered by `view!`/CSS hot reload, and Rust patches apply on the server (tip
  crate only).

## Troubleshooting

- **Hydration panic (`failed_to_cast_element`).** The SSR and WASM builds emitted
  different hot-reload markers. Ensure `.cargo/config.toml` pins `LEPTOS_WATCH`
  and that `[profile.hot]` exists; if you changed either, a one-time
  `cargo clean -p leptos` is needed because cargo does not track the env var.
- **`could not open … .pdb`.** The patch builder reads symbols from a PDB. The fat
  link forces `/DEBUG:FULL` and the run copy falls back to the `deps/<crate>.pdb`
  when cargo does not copy it. If it still fails, the binary may have no debug
  info.
- **Crash on the first patched call (`0xC0000409`).** The patch exported `main` as
  an incremental-link thunk whose address did not match the PDB RVA the table
  rebases against. Both links pass `/INCREMENTAL:NO`; if you see this, check that
  the linker shim is the current build.
- **Patch applies but nothing changes.** The edit was in a dependency/lib crate,
  which the patch does not contain (tip-crate limit). Move the change to the bin
  crate or do a full rebuild. Run with `MONTRS_HOTPATCH_PROBE=1` — the client logs
  whether the jump table contains the cutover key.
