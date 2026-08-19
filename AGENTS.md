# MontRS — Agent Guide

This repo *is* the MontRS framework, not a user app. Always operate in **Framework Contributor** mode.

## Agentic Loop (use this first)

MontRS has a native agent system. Start every session with:

```bash
montrs agent list-errors        # check tracked errors
montrs agent doctor             # health check
```

Then follow the workflow for your task:

| If you're... | Use this workflow doc | Key commands |
|-------------|----------------------|--------------|
| Fixing a bug | `docs/agent/workflows/fixing-errors.md` | `list-errors` → `agent diff` → fix → `agent check` |
| Adding a feature | `docs/agent/workflows/adding-features.md` | Read invariants → implement → `agent check` |
| Restructuring | `docs/agent/workflows/restructuring.md` | `agent check` → refactor → `agent check` |
| Starting new | `docs/agent/workflows/new-projects.md` | Use templates, `montrs new` |

After any change: `montrs agent check` then `montrs agent snapshot` to regenerate `.agent/agent.json`.

## Data Files (read these)

- `.agent/agent.json` — project spec: plates, routes, tools, **package invariants**
- `.agent/tools.json` — CLI commands as agent-callable functions
- `.agent/errorfiles/` — versioned error captures with `suggested_fixes`
- `.agent/rules/` — persona rules: `app-developer.md`, `framework-contributor.md`
- `skills/<name>/skill.toml` — multi-step agent workflows

**Invariants**: Read scoped rules from `agent.json → packages[].invariants` for the package you're editing. Don't read all invariants by default.

## Metadata Conventions

- `@agent-tool` comment on agent-callable functions
- `@agent-skill` comment on multi-step capability definitions
- `description()` on every trait impl (Plate, Route, Loader, Action)
- `AgentError` trait with stable error codes and `suggested_fixes`

## Architecture

48 workspace packages under `packages/`:

| Package | Role |
|---------|------|
| `agent` | Sidecar: snapshots, error tracking, tool curation, PRDoc. **No LLM inference**. |
| `agentignore` | `.agentignore` patterns + IDE export. |
| `auth` | Authentication system (email/password, OAuth, 2FA, sessions, RBAC). |
| `bench` | Statistical benchmarking. |
| `build` | Build pipeline facade (re-exports sub-packages). |
| `build-core` | BuildPipeline trait + BuildConfig. |
| `build-serve` | Dev server (axum static file serving). |
| `build-watch` | File watcher with debounced rebuild (notify). |
| `cli` | Binary entrypoint (`montrs` command), delegates to core/agent. |
| `command` | Typed command registry + deterministic prefix search (command palettes). |
| `content` | Typed Markdown content collections with deterministic ordering. |
| `core` | Foundational traits (Plate, Route, AppSpec, AgentError). **Dep on platform only.** |
| `deps` | Dependency freshness checking. |
| `desktop` | Native desktop (wry webview, winit+wgpu window). |
| `env` | Environment variable parsing + `.env` loading + Tera templates. |
| `fmt` | Custom formatter for Rust + `view!` macros. |
| `haptics` | Cross-platform haptic feedback. |
| `hotkeys-core` | Platform-independent shortcut parsing/matching. |
| `hotkeys-web` | Browser/WASM hotkey adapter. |
| `i18n` | Internationalization with macros, plurals, formatting, scoping. |
| `icons` | 1600+ Lucide icons as Leptos components. |
| `image-core` | Validated, serializable image request specs. |
| `image-optimizer` | Bounded server-side image optimization policy. |
| `lockfile` | Deterministic tool version locking. |
| `log` | Structured log store with retention, streaming, rotate. |
| `metadata` | `montrs.toml` single source of truth (all sections incl. services/proxy). |
| `mobile` | Mobile platform adapter (Android/iOS shells). |
| `montrs` | Facade crate — re-exports. Minimal logic. |
| `motion` | Spring, tween, keyframes, gestures, SVG/CSS animation. |
| `orm` | SQL-first, backend-agnostic DB abstraction. |
| `platform` | Target enum, PlatformAdapter trait. **No deps on other packages.** |
| `plugin` | Tool plugin system (asdf/vfox-compatible). |
| `prdoc` | PR doc parser/generator/changelog. |
| `proxy` | Reverse proxy routing `<slug>.localhost` to ports. |
| `registry` | Tool registry (baked + floating). |
| `renderer` | Renderer trait + geometry primitives (wgpu/tiny-skia backends). |
| `runner` | Custom task runner config. |
| `runtime` | Native Rust runtime (Deno-inspired ops, memory-optimized). |
| `services` | Service supervisor (daemon management, ready checks, retry, hooks, cron). |
| `shell` | Shell integration (bash/zsh/fish/pwsh) + shims. |
| `sigstore` | GitHub attestation, cosign, SLSA verification. |
| `state` | Deterministic stores, selectors, typed state machines, history. |
| `table-core` | Headless table state + row models (stable column/row IDs). |
| `test` | Deterministic TestRuntime, fixtures, E2E orchestration (Playwright). |
| `tool` | Tool version manager (5 backends: core, cargo, github, http, ubi). |
| `tui` | Full terminal UI library (21 renderables, keymap, plugins, audio, ssh, qr, 3d). |
| `ui` | shadcn-inspired component library + theme system + toaster. |
| `utils` | Generic pure functions. |
| `validator` | Proc-macros (`#[derive(Validator)]`), compile-time validation. |
| `web` | Web platform adapter (WASM browser bindings). |

Entrypoints: `packages/cli/src/bin/montrs.rs`, `packages/montrs/src/lib.rs`, `packages/core/src/lib.rs`.

## Toolchain

- **Rust**: `nightly-2026-02-18` (pinned in `rust-toolchain.toml`, CI enforces)
- **Target**: `wasm32-unknown-unknown` required
- **Cargo**: edition 2024, resolver "2"

## Developer Commands

```bash
mise run ci           # fmt → clippy → test (CI order, do NOT reorder)
mise run fmt          # cargo fmt --all
mise run clippy       # cargo clippy --workspace -- -D warnings
mise run test         # cargo test --workspace
mise run build        # cargo build --workspace
mise run dev          # cargo run --package montrs-cli -- serve
mise run bench        # cargo run --package montrs-cli -- bench

# Single-package
cargo test -p montrs-agent
cargo clippy -p montrs-core -- -D warnings
```

**Required order** (CI enforces): `fmt (--check)` → `clippy -D warnings` → `test` → `build --release`

## Casing (enforced by montrs-fmt for `view!`)

| Item | Convention | Example |
|------|-----------|---------|
| Components | PascalCase | `<UserProfile />` |
| Attributes | kebab-case | `on-click`, `class-name` |
| Rust vars | snake_case | `user_name` |
| Files | kebab-case | `user-profile.rs` |

## Testing

- `cargo test --workspace` for unit/integration tests
- E2E: `packages/test`, `#[cfg(feature = "e2e")]`, Playwright
- Use `TestRuntime` for deterministic in-process Loader/Action tests
- All tests must be hermetic, deterministic, and isolated

## PRDoc

Structured PR docs at `prdoc.md` (root). Agent commands:

```bash
montrs agent prdoc validate       # check schema
montrs agent prdoc generate       # auto-generate from PR context
montrs agent prdoc show           # view as JSON
```

## Skills

Composable workflows in `skills/<name>/skill.toml`. List with `montrs agent skills list`, load with `--name <name>`.

## MCP

```bash
montrs mcp serve                  # starts MCP server for agent tool calls
```

MCP tools: `get_project_snapshot`, `agent_list_errors`, `agent_diff`, `agent_check`, `list_router_structure`.

## Gotchas

- `.agentignore` controls agent scanning (separate from `.gitignore`)
- `montrs fmt` enforces view! macro casing — run before committing
- Build in `--release` for deployment; artifacts at `target/release/`
- Always implement `description()` on traits — it feeds agent.json
- This is **not** a user app — `montrs new`, `generate`, `serve` target framework dev, not app building
