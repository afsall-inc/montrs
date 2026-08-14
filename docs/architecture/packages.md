# Package Boundaries & Responsibilities

MontRS is organized as a modular workspace. Each package has a specific responsibility and strictly defined boundaries to ensure modularity and ease of maintenance.

---

## 📦 `montrs-core`
- **Responsibility**: Foundational traits (`Plate`, `Loader`, `Action`), routing engine, and `AppSpec` definition.
- **Key Components**: `Router`, `Context`, `AppSpec`.
- **Boundary**: It is strictly IO-agnostic. It defines the "Grammar" of how MontRS apps are built.
- **When to modify**: When you need to change how routing works or add new fundamental capabilities to the framework.

## 📦 `montrs-cli`
- **Responsibility**: Scaffolding (`new`), orchestration (`build`, `serve`), and task management.
- **Key Components**: `Config`, `TaskRunner`, `ProjectScaffolder`.
- **Boundary**: It is the "Orchestrator." It depends on `core` and `agent` to understand the project state but does not contain business logic.
- **When to modify**: When adding new CLI commands or improving the developer experience (DX).

## 📦 `montrs-agent`
- **Responsibility**: Agent-first logic, snapshot generation (`agent.json`), and versioned error tracking.
- **Key Components**: `AgentManager`, `ErrorRecord`, `ToolScanner`.
- **Boundary**: Acts as a "Sidecar." It scans the codebase (using `core` metadata) to produce machine-optimized context.
- **When to modify**: When improving agent discoverability or changing the `agent.json` structure.

## 📦 `montrs-orm`
- **Responsibility**: Database abstraction, SQL execution, and row mapping.
- **Key Components**: `Database`, `Transaction`, `FromRow`.
- **Boundary**: Handles all persistent data interactions. It provides a unified API that abstracts away the specific database driver (SQLite/Postgres).
- **When to modify**: When adding support for a new database backend or improving the query builder.

## 📦 `montrs-validator`
- **Responsibility**: Declarative validation and metadata generation via proc-macros.
- **Key Components**: `#[derive(Validator)]`, `Validator`.
- **Boundary**: Defines the "Contract" for data structures. It is used by both `core` (for routing) and `orm` (for mapping).
- **When to modify**: When adding new validation rules or expanding metadata capture.

## 📦 `montrs-test`
- **Responsibility**: Deterministic test runtime, fixtures, and E2E drivers.
- **Key Components**: `TestRuntime`, `FixtureManager`.
- **Boundary**: Provides the "Validation Infrastructure." It allows testing of `Loader` and `Action` logic without needing a real network or database.
- **When to modify**: When improving the testability of the framework or adding new mocking capabilities.

## 📦 `montrs-haptics`
- **Responsibility**: Cross-platform haptic feedback for web, desktop, and mobile.
- **Key Components**: `HapticsProvider` trait, `ImpactStyle`, `HapticsConfig`, platform-specific providers.
- **Boundary**: Provides a unified `HapticsProvider` trait with feature-gated platform backends. Web uses Vibration API; desktop uses OS-native calls as temporary fallback; mobile is stubbed until native bridges land.
- **When to modify**: When adding a new platform target, changing the trait interface, or implementing the desktop engine integration.

## 📦 `montrs-icons`
- **Responsibility**: 1600+ Lucide icons as Leptos components, grouped into 42 category features.
- **Key Components**: `Glyph` enum, `Icon` generic component, per-icon convenience components (e.g. `SearchIcon`), `CustomIcon` for inline SVG.
- **Boundary**: Pure icon rendering and metadata (search, categories, tags). No animation, picker UI, or icon design tools.
- **When to modify**: When adding new icons from upstream Lucide, improving search, or adding rendering features.

## 📦 `montrs-ui`
- **Responsibility**: UI component library with Tailwind CSS macros, shadcn-inspired theming system, and type-safe variant components.
- **Key Components**: `clx!`/`void!` macros, `variants!` macro, `cn!()` utility, `ThemeProvider`, `components.json` config, theme generator.
- **Boundary**: Pure Rust macros and components. No JavaScript, no Objective-C, no external runtime dependencies. Relies on `montrs-icons` for icon rendering.
- **When to modify**: When adding new UI macros, updating the theming system, or adding new component patterns.

## 📦 `montrs-platform`
- **Responsibility**: Platform abstraction — `Target` enum, `PlatformAdapter` trait, `NoopPlatformAdapter`.
- **Key Components**: `Target`, `PlatformAdapter`, `NoopPlatformAdapter`, `NativeMenuItem`.
- **Boundary**: Layer-0 package with zero MontRS dependencies. Used by `core` for target identification and by `desktop`/`mobile` for platform-specific behavior.
- **When to modify**: When adding a new target variant or platform capability.

## 📦 `montrs-build-core`
- **Responsibility**: `BuildPipeline` trait and `BuildConfig` types — the interface for the build system.
- **Key Components**: `BuildPipeline`, `BuildStep`, `BuildConfig`, `find_workspace_target_dir`.
- **Boundary**: Trait-only, no heavy dependencies (no axum, no notify). Used by `build-watch` and `build-serve` to avoid depending on the concrete Pipeline.
- **When to modify**: When adding new build steps or changing the build interface.

## 📦 `montrs-build-watch`
- **Responsibility**: File system watcher with debounced rebuild triggers.
- **Key Components**: `watch_directory`, `watch_and_rebuild`.
- **Boundary**: Depends on `montrs-build-core` for the `BuildPipeline` trait. Uses `notify` for cross-platform file watching.
- **When to modify**: When changing the watch strategy or debounce behavior.

## 📦 `montrs-build-serve`
- **Responsibility**: HTTP dev server for static file serving.
- **Key Components**: `ServeConfig`, `serve_static`, `serve_with_callback`.
- **Boundary**: Depends on `montrs-build-core` for configuration types. Uses `axum` + `tower-http` for serving.
- **When to modify**: When adding server features (live reload, proxy, HTTPS).

## 📦 `montrs-env`
- **Responsibility**: Environment variable management — parse `[env]` from `montrs.toml`, Tera rendering, `.env` loading, apply to process.
- **Key Components**: `EnvDirective`, `Environment`, `EnvDiff`, `parse_env_section`, `resolve_environment`, `apply_environment`.
- **Boundary**: Layer 2. Handles parsing, rendering, and applying env vars. No file watching or secret management.
- **When to modify**: When adding new env directive types or changing the resolution order.

## 📦 `montrs-sigstore`
- **Responsibility**: Signature verification — cosign, SLSA, GitHub attestations.
- **Key Components**: `AttestationClient`, `GitHubSource`, `AttestationSource` trait, `verify_github_attestation`, `verify_cosign_signature`, `verify_slsa_provenance`.
- **Boundary**: Layer 2. Verification only — no signing, key generation, or certificate management.
- **When to modify**: When adding new verification methods or attestation sources.

## 📦 `montrs-registry`
- **Responsibility**: Tool registry — baked and floating metadata for version management.
- **Key Components**: `Registry`, `RegistryTool`, `BAKED_REGISTRY`, `load_registry_from_dir`, `fetch_floating_registry`.
- **Boundary**: Layer 2. Tool metadata only — no installation or version resolution.
- **When to modify**: When adding new registry tool entries or changing the registry format.

## 📦 `montrs-plugin`
- **Responsibility**: Plugin system — asdf-compatible tool plugin management.
- **Key Components**: `Plugin` trait, `PluginRegistry`, `PluginType`, `PluginSource`, `install_git_plugin`, `uninstall_plugin`.
- **Boundary**: Layer 2. Plugin lifecycle only — no tool version management or binary execution.
- **When to modify**: When adding new plugin types or install methods.

## 📦 `montrs-tui`
- **Responsibility**: Terminal UI library — buffer, rendering, events, widgets.
- **Key Components**: `Buffer`, `CliRenderer`, `EventBus`, `Renderable` trait, `TuiAdapter`, 21 renderable components.
- **Boundary**: Layer 3. Pure Rust ANSI terminal output — no external dependencies like ncurses.
- **When to modify**: When adding new widgets, rendering features, or event types.

## 📦 `montrs-runtime`
- **Responsibility**: General-purpose Rust runtime — ops, extensions, resource table, event loop, memory primitives.
- **Key Components**: `MontrsRuntime`, `RuntimeExtension`, `OpDecl`, `OpState`, `ResourceTable`, `EventLoop`, `ModuleLoader`, `Arena`, `TaggedValue`.
- **Boundary**: Layer 0. Extensions add ops and state. No application logic.
- **When to modify**: When adding new runtime features, extension types, or memory optimization primitives.

---

## How Packages Interact

Every package in the workspace maintains its own **[Local Invariants](file:///packages/*/docs/invariants.md)**. These documents define the specific "rules of engagement" and architectural boundaries for that package, providing immediate context for both human contributors and AI agents.

MontRS follows a **Dependency Inversion** pattern. `montrs-core` defines the traits, and other packages (like `orm` or `validator`) provide implementations or tools that work with those traits.

### Dependency Hierarchy

Packages are organized into layers. A package at layer N may depend on packages at layer N or below, but never on packages at a higher layer.

| Layer | Packages | Rules |
|---|---|---|
| **0 (Core)** | `core`, `validator`, `platform`, `runtime` | No montrs-* deps (except platform → core re-export) |
| **1 (Foundation)** | `utils`, `metadata`, `agentignore`, `runner`, `env`, `sigstore`, `registry`, `plugin` | Only core/validator/platform |
| **2 (Feature)** | `agent`, `orm`, `fmt`, `bench`, `prdoc`, `haptics`, `motion`, `icons`, `ui`, `test`, `build-core`, `build-watch`, `build-serve` | Only layers 0-1 |
| **3 (Shell)** | `cli`, `desktop`, `mobile`, `renderer`, `build`, `montrs`, `tui` | Any layer |

### Dependency Flow

1.  **CLI** reads **Config** and **Core** to understand the app.
2.  **Core** uses **Validator** to validate data at the boundaries.
3.  **Plates** use **ORM** to persist data.
4.  **UI** uses **Icons** for icon rendering.
5.  **Agent** scans everything to produce the **Spec Snapshot**.
6.  **Build** orchestrates **Build-core** (pipeline trait), **Build-watch** (file watcher), and **Build-serve** (dev server).

---

## 🛠️ Adding New Packages

If you are a contributor looking to add a new package to the MontRS workspace, you **must** follow the guidelines in the **[Packages Contribution Guide](../community/packages-contribution.md)**. 

Key requirements include:
- Defining clear boundaries.
- Ensuring Agent-first compatibility.
- Updating this document with the new package's responsibility.
