# montrs-test

Deterministic testing utilities for the MontRS ecosystem.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-test` provides the infrastructure for writing robust unit, integration, and end-to-end tests. It emphasizes determinism, allowing developers to boot their entire application spec in-process for fast and reliable verification.

## 2. What problems it solves
- **Flaky Tests**: By providing a deterministic `TestRuntime`, it eliminates "it works on my machine" issues caused by timing or environment variance.
- **Complex Setup**: The `Fixture` system automates the setup and teardown of external resources like databases or file systems.
- **E2E Overhead**: Integrated Playwright support via `MontrsDriver` simplifies browser automation for full-stack tests.

## 3. What it intentionally does NOT do
- **Test Execution**: It does not replace `cargo test`; it provides the tools used *within* your tests.
- **Static Analysis**: It does not check code for bugs without running it (use `clippy` for that).
- **Code Coverage**: It does not generate coverage reports (use `cargo-tarpaulin` or similar).

## 4. How it fits into the MontRS system
It is the **validation layer**. It uses the `AppSpec` from `montrs-core` to spin up isolated environments for testing.

## 5. When a user should reach for this package
- When writing unit tests for a `Loader` or `Action`.
- When building an integration test that requires a mock database or environment.
- When creating end-to-end user journey tests using a browser.

## 6. Deeper Documentation
- [Testing Philosophy](../../docs/testing/index.md)
- [Using the TestRuntime](../../docs/testing/index.md#test-runtime)
- [E2E with MontrsDriver](../../docs/testing/index.md#e2e-testing)
- [Table-Driven Testing](../../docs/testing/index.md#table-driven-tests)

## 7. Notes for Agents
- **Deterministic Assertions**: Use `expect(...)` for fluent, human-readable assertions in generated tests.
- **Test Generation**: Refer to the `Plate` and `AppSpec` metadata to understand what inputs and outputs need to be tested.
- **Error Handling**: Look for `TestError` with `AgentError` metadata if a test fixture or driver fails.
- **Isolaton**: Always use `TestEnv` and `run_fixture_test` to ensure that tests do not leak state to the host system.

## E2E Testing Usage

Enable the `e2e` feature in `Cargo.toml`. You may also want to add `playwright` if you need direct access to its types:

```bash
cargo add montrs-test --features e2e
# Optional: Add playwright-rs for direct type usage
cargo add playwright-rs
```

Then use `MontrsDriver` in your tests:

```rust
use montrs_test::e2e::MontrsDriver;

#[tokio::test]
async fn test_home_page() -> anyhow::Result<()> {
    let driver = MontrsDriver::new().await?;
    driver.goto("/").await?;
    let title = driver.page.title().await?;
    assert!(title.contains("MontRS"));
    driver.close().await?;
    Ok(())
}
```

## Deterministic Test Fabric

Everything below runs **in-process**: no server, no browser, no database, no wall clock.

| Feature | Adds | What you test |
|---------|------|---------------|
| (kernel) | `Clock`, `TestClock`, `Rng`, `TestRng`, `TestHarness` | Deterministic time and randomness |
| `http` | `TestClient`, `TestResponse`, harness `load`/`act` | Backend, APIs, loaders, actions |
| `db` | `sqlite_memory`, `SqliteFixture`, `RecordingDb`, `MockDb` | SQL behaviour and call expectations |
| `sim-dom` | `ComponentTest` | Component structure, text, classes, ARIA |
| `layout` | `SimLayout`, `Viewport` | Box-model layout and **horizontal overflow** |
| `motion` | `MotionTest` | Springs, tweens, keyframes over a virtual timeline |

### Kernel

```rust
use montrs_test::prelude::*;

let harness = TestHarness::new(build_spec()).seed(42).with_env("MODE", "test");
harness.advance_ms(500); // time only moves when you say so
```

### APIs and loaders (no server)

```rust
let client = TestClient::new(&spec, || view! { <App /> })?;
client.get("/health").unwrap().assert_status(200);

let data = harness.load("/users/:id").await?;   // runs the loader in-process
harness.act("/users", json!({ "name": "Ada" })).await?;
```

### Database (no server)

```rust
let db = sqlite_memory()?;                       // real SQL, in memory
let rec = RecordingDb::new(sqlite_memory()?);    // records + delegates
assert!(rec.ran("INSERT INTO users"));
```

### Components and overflow (no browser)

```rust
let view = ComponentTest::render(|| view! { <Card /> });
view.assert_role("button", "Save");
view.assert_class("div", "card");

SimLayout::compute(view.html(), Viewport::width(320))
    .assert_no_horizontal_overflow();
```

### Motion (deterministic timeline)

```rust
let mut motion = MotionTest::new(Spring::new(100.0, 10.0, 1.0).with_range(0.0, 1.0));
motion.step_ms(16);
motion.assert_settles_within_ms(600, 0.01);
```

### Notes and limits

- `ComponentTest` renders to HTML: structure, text, classes, and ARIA are
  covered, but live event dispatch requires a DOM (use the browser/CDP backend,
  or `wasm-bindgen-test`).
- `SimLayout` models a documented subset of inline styles and Tailwind
  utilities and ignores text metrics. Use the CDP backend for pixel-accurate
  checks.
- `MockDb` answers `execute` and records `query`; it cannot fabricate typed rows
  because `DbBackend::query` is generic over `T: FromRow`. Use `sqlite_memory`
  for row-level assertions.


```
