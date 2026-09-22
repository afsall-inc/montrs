# Test Package Invariants

## 1. Responsibility
`montrs-test` provides the infrastructure for deterministic unit, integration, and e2e testing, plus the hermetic "test fabric" that makes every MontRS layer testable in-process.

## 2. Invariants
- **Determinism**: Tests must be reproducible. Any non-deterministic behavior (time, random) must be mockable via `TestHarness` (`TestClock`, `TestRng`).
- **Isolation**: Tests should not leak state between runs.
- **Hermetic by default**: The fabric must not require a network socket, browser, database server, or child process. Optional backends (browser/CDP, Playwright) are feature-gated and opt-in.
- **Agent-Verifiable**: Testing utilities should provide clear, machine-readable output for agents to verify their own changes.
- **Additive core changes**: Test-facing core APIs (e.g. `SsrApp::render_request`, `Router::route`) must be backward compatible.

## 3. Boundary Definitions
- **In-Scope**: Mocking traits, test runners, assertion libraries, in-process HTTP/DOM/layout/motion harnesses, e2e orchestration.
- **Out-of-Scope**: Implementation of business logic being tested.

## 4. Documented Limits (do not regress silently)
- `ComponentTest` renders to HTML: structure/text/classes/ARIA are hermetic, live event dispatch is not (use the browser/CDP backend or `wasm-bindgen-test`).
- `SimLayout` models a subset of inline styles + Tailwind utilities and ignores text metrics.
- `MockDb` cannot fabricate typed rows because `DbBackend::query` is generic over `T: FromRow`; use `sqlite_memory` for row assertions.

