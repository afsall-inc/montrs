# Agent Guide: montrs-test

This guide helps agents write and run tests in MontRS.

## Core Concepts

### 1. TestHarness (start here)
One object owns the app spec, mocked env, clock, and RNG. Prefer it over the raw
`TestHarness`.

```rust
use montrs_test::prelude::*;

let harness = TestHarness::new(build_spec()).seed(42).with_env("MODE", "test");
harness.advance_ms(500); // virtual time only moves when you say so
```

### 2. TestEnv
Mock environment variables to test different configurations.
```rust
let env = TestEnv::new();
env.set("KEY", "VALUE");
```

### 3. Fixture Trait
Define setup and teardown logic for complex integration tests.

### 4. In-process backends (feature-gated)
- `http` — `TestClient` renders requests through the router without a socket;
  `harness.load(path)` / `harness.act(path, input)` run loaders/actions directly.
- `db` — `sqlite_memory()` (real SQL), `RecordingDb` (records + delegates),
  `MockDb` (execute expectations). `MockDb` cannot build typed rows.
- `sim-dom` — `ComponentTest` renders a `view!` to HTML and queries it by CSS
  selector (text, classes, attributes, ARIA roles).
- `layout` — `SimLayout::compute(html, Viewport)` evaluates the box model and
  asserts `assert_no_horizontal_overflow()`.
- `motion` — `MotionTest` steps springs/tweens/keyframes over a virtual timeline.

### 5. E2E Testing
Uses Playwright for browser automation (requires a browser; not hermetic).
- **Agent Recommendation**: Use `MontrsDriver` for full user-journey flows, and
  the hermetic backends above for everything else.

### 6. Assertions
Use `expect(value).to_equal(expected)` for fluent, agent-readable assertions.

## Agent Usage Patterns

### Generating Tests
When asked to add tests for a new feature:
1. Prefer the hermetic backends: `http` for APIs/routes, `sim-dom` + `layout`
   for UI, `db` for persistence, `motion` for animation, the kernel for time/RNG.
2. Use `TestEnv`/`TestHarness::with_env` for configuration mocking.
3. Use `run_fixture_test` (or `SqliteFixture`) when a resource needs setup/teardown.
4. Only reach for the `e2e` feature when a real browser interaction is required.

### Feature Selection Cheat Sheet
| Need | Feature | Entry point |
|------|---------|-------------|
| API / loader / action | `http` | `TestClient`, `harness.load`/`act` |
| Database | `db` | `sqlite_memory`, `RecordingDb`, `MockDb` |
| UI structure / a11y | `sim-dom` | `ComponentTest` |
| Layout / overflow | `layout` | `SimLayout`, `Viewport` |
| Animation | `motion` | `MotionTest` |
| Time / randomness | (kernel) | `TestClock`, `TestRng` |

### Debugging Failures
If a test fails with `TEST_EXPECTATION`, compare the actual and expected values
provided in the `AgentError`.
