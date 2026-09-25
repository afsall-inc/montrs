# Incremental Builds

MontRS builds are **incremental by construction**. Every build step is keyed by a
content hash of its inputs, so a second build that changes nothing re-runs
nothing — and a change that only touches one step re-runs only that step.

No JavaScript, no Node, no daemon: just Rust and a directory under
`.montrs/cache/`.

---

## 1. How it works

A build step declares its **inputs** and **outputs**:

| Step | Inputs | Outputs |
|------|--------|---------|
| `wasm` | app sources, `Cargo.toml`, `Cargo.lock`, `montrs.toml`, `PROFILE`/`LEPTOS_WATCH`/`RUSTFLAGS`, workspace `packages/*` sources + manifests, `rustc` version | `target/wasm32-unknown-unknown/<profile>/<pkg>.wasm` |
| `bindgen` | the WASM artifact, `wasm-bindgen` version | `pkg/front.js`, `pkg/front_bg.wasm` |
| `tailwind` | input stylesheet, scanned source trees, `tailwindcss` version | `site/main.css` |
| `assets` | the assets directory contents | copied files under `site/` |
| `index-html` | project name, output name | `site/index.html` |
| `server` | app + workspace sources, `rustc` version, profile | `target/<profile>/<pkg>-ssr` |

The key is `namespace + SHA-256(inputs)`. A recorded manifest stores, for each
output, its relative path, length, and content hash. A step is skipped when the
key is unchanged **and** every output still matches its fingerprint.

When inputs change, the key changes and the step runs. When outputs are deleted
(e.g. `cargo clean`), the fingerprints no longer match and the step runs.

> **Rule:** the cache always misses when unsure. It never serves a stale artifact.

---

## 2. Independent steps run in parallel

`tailwind` and `assets` have no dependency on the Rust build, so they run
concurrently with the `wasm → bindgen → server` chain. `cargo` serializes itself
through its own build-directory lock, so the Rust steps stay ordered while the
CSS and asset steps overlap them.

---

## 3. Controlling the cache

```bash
# Normal build: uses the cache.
montrs build

# Force a full rebuild, ignoring the cache.
MONTRS_NO_CACHE=1 montrs build
```

The cache lives at `.montrs/cache/` and is ignored by both git and
`.agentignore`. Delete it to reclaim space or to reset all fingerprints.

---

## 4. Typical effect

| Scenario | What re-runs |
|----------|--------------|
| Nothing changed | nothing (all steps hit) |
| Edit a Tailwind stylesheet | `tailwind` only |
| Edit a component's `view!` | `view!` hot reload patches the DOM; no cargo step |
| Edit server logic | `wasm`? no → `bindgen`? no → `server` only (via `build_server_only` in serve) |
| Add a dependency | `wasm`, `bindgen`, `server` |

---

## 5. Documented limits

- The `tailwind` step approximates Tailwind's scan set with the input stylesheet,
  the app `src`, and `packages/ui/src` + `packages/icons/src`. If your CSS
  `@source`s additional trees, add them to the step inputs (or disable the cache
  with `MONTRS_NO_CACHE=1`).
- The cache is **local**. A remote/shared cache backend is possible via the
  `montrs-cache` API but is not shipped yet.
