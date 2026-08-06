# Testing in MontRS

MontRS provides a robust and comprehensive testing ecosystem designed to ensure your application is reliable, performant, and bug-free. The testing strategy is built on three pillars:

1.  **Unit Testing**: Fast, isolated tests for individual functions and components.
2.  **Integration Testing**: Tests that verify the interaction between multiple components (e.g., database, router, services).
3.  **End-to-End (E2E) Testing**: Full-stack tests that simulate real user interactions in a browser environment.

This guide covers how to leverage the `montrs-test` package and the `montrs` CLI to implement these testing strategies.

---

## 1. Unit Testing

Unit tests in MontRS follow standard Rust testing practices but are enhanced with the `montrs-test` library for better determinism and mocking.

### Running Unit Tests

You can run unit tests using the standard Cargo command or the MontRS CLI:

```bash
# Standard Rust way
cargo test

# MontRS CLI (offers enhanced reporting)
montrs test
```

The `montrs test` command supports generating JSON and JUnit reports for CI/CD integration:

```bash
montrs test --report junit --output test-results.xml
```

### Writing Unit Tests

MontRS provides a suite of production-grade utilities to improve Developer Experience (DevX), including fluent assertions, spies, and table-driven tests.

#### Fluent Assertions

Instead of standard `assert_eq!`, you can use `expect` for more readable tests.

```rust
use montrs_test::unit::expect;

#[test]
fn test_fluent_assertions() {
    expect(1 + 1).to_equal(2);
    expect(vec![1, 2, 3]).to_contain(&2);
    expect(true).to_be_true();
    
    // Negation
    expect(1).not().to_equal(2);
}
```

#### Spies and Mocks

Track function calls and verify interactions using `Spy` and `Mock`.

```rust
use montrs_test::unit::Spy;

#[test]
fn test_spy() {
    let spy = Spy::new();
    
    // Simulate a call
    spy.record();
    
    expect(spy.called()).to_be_true();
    expect(spy.call_count()).to_equal(1);
}
```

#### Table-Driven Tests

Easily define parameterized tests using the `table_test!` macro.

```rust
use montrs_test::table_test;

fn add(a: i32, b: i32) -> i32 { a + b }

table_test! {
    name: test_addition,
    func: add,
    cases: [
        (1, 1) => 2,
        (2, 2) => 4,
        (10, -5) => 5,
    ]
}
```

### Environment Mocking

Use `montrs_test::integration::TestEnv` to mock environment variables and configurations without affecting your global environment.

```rust
#[cfg(test)]
mod tests {
    use montrs_test::integration::TestEnv;
    use montrs_core::EnvConfig;

    #[test]
    fn test_config_loading() {
        let env = TestEnv::new();
        env.set("API_KEY", "test-secret-123");

        assert_eq!(env.get_var("API_KEY").unwrap(), "test-secret-123");
    }
}
```

---

## 2. Integration Testing

Integration tests verify that different parts of your application work together correctly. MontRS provides the `TestRuntime` to spin up a lightweight version of your app context.

### Using Fixtures

The `Fixture` trait allows you to define reusable setup and teardown logic for your tests.

```rust
use montrs_test::integration::{Fixture, run_fixture_test};

struct DbFixture {
    conn_string: String,
}

#[async_trait::async_trait]
impl Fixture for DbFixture {
    async fn setup() -> anyhow::Result<Self> {
        // Spin up a test DB or use an in-memory one
        Ok(Self { conn_string: "sqlite::memory:".to_string() })
    }

    async fn teardown(&mut self) -> anyhow::Result<()> {
        // Cleanup logic
        Ok(())
    }
}

#[tokio::test]
async fn test_database_interaction() {
    run_fixture_test::<DbFixture, _>(|fixture| async move {
        assert_eq!(fixture.conn_string, "sqlite::memory:");
        Ok(())
    }).await.unwrap();
}
```

---

## 3. End-to-End (E2E) Testing

MontRS provides a high-level E2E testing framework powered by Playwright (via the `playwright-rs` crate) and integrated into the `montrs` CLI.

### Architecture

The E2E plate is built around the `MontrsDriver`, a wrapper around the Playwright browser automation library. It handles:
- **Browser Lifecycle**: Automatically launching and closing browser instances (Chromium, Firefox, WebKit).
- **Configuration**: Auto-detecting settings from `montrs.toml` or environment variables (`MONTRS_SITE_URL`, `LEPTOS_SITE_ADDR`).
- **Orchestration**: When run via `montrs e2e`, it manages the full lifecycle of your application server (startup, readiness check, test execution, shutdown).

### Setup

To enable E2E testing in your project, add `montrs-test` with the `e2e` feature to your `Cargo.toml` (usually in a dedicated `e2e` crate or the `[dev-dependencies]` section):

```toml
[dependencies]
montrs-test = { version = "0.1.0", features = ["e2e"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

### Configuration

You can configure E2E settings in your `montrs.toml` file under the `[e2e]` section:

```toml
[e2e]
# browser = "chromium"
# headless = true
# timeout = 30000 # Not yet supported in montrs.toml, use env var or default
base_url = "http://localhost:3000"
```

Alternatively, you can override these at runtime using environment variables:
- `MONTRS_E2E_HEADLESS`: "true" or "false"
- `MONTRS_E2E_BROWSER`: "chromium", "firefox", or "webkit"
- `MONTRS_SITE_URL`: The base URL of the running application.

### Writing E2E Tests

E2E tests use the `MontrsDriver` to interact with your application.

```rust
use montrs_test::e2e::MontrsDriver;

#[tokio::test]
async fn test_home_page_loads() -> anyhow::Result<()> {
    // 1. Initialize the driver (launches browser, connects to context)
    let driver = MontrsDriver::new().await?;

    // 2. Navigate to a route (automatically resolves against base_url)
    driver.goto("/").await?;

    // 3. Perform assertions
    let title = driver.page.title().await?;
    assert!(title.contains("Welcome to MontRS"));

    // 4. Interact with elements
    driver.page.click("text=Get Started").await?;
    assert!(driver.url().contains("/docs"));

    // 5. Cleanup
    driver.close().await?;
    
    Ok(())
}
```

### Advanced: Extending MontrsDriver with Plugins

You can extend the functionality of `MontrsDriver` by implementing the `MontrsPlugin` trait. This is useful for shared logic like authentication, custom logging, or complex setup routines.

```rust
use montrs_test::e2e::{MontrsDriver, MontrsPlugin};

struct AuthPlugin {
    username: String,
}

#[async_trait::async_trait]
impl MontrsPlugin for AuthPlugin {
    async fn on_init(&self, driver: &MontrsDriver) -> anyhow::Result<()> {
        // Automatically log in when the driver starts
        driver.goto("/login").await?;
        driver.page.fill("input[name=username]", &self.username).await?;
        driver.page.click("button[type=submit]").await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_dashboard() -> anyhow::Result<()> {
    let driver = MontrsDriver::new().await?;
    
    // Apply the plugin
    driver.use_plugin(AuthPlugin { username: "admin".into() }).await?;
    
    // Now we are logged in
    driver.goto("/dashboard").await?;
    assert!(driver.url().contains("/dashboard"));
    
    driver.close().await?;
    Ok(())
}
```

### Running E2E Tests

```bash
# Run all E2E tests
montrs e2e

# Run in a specific browser
montrs e2e --browser firefox

# Run in headed mode (shows the browser)
montrs e2e --headless false
```

The `montrs e2e` command automatically:
1. Builds your application.
2. Starts the development server.
3. Runs the Playwright test suite against the live server.
4. Shuts down the server upon completion.

You can also run them as standard Rust tests if you manage the server yourself:

```bash
# Start your server first
cargo run & 
# Run tests
MONTRS_SITE_URL=http://localhost:3000 cargo test --package e2e
```

---

## CI/CD Integration

MontRS testing tools are designed for CI environments.

### GitHub Actions Example

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install Playwright Dependencies
        run: npx playwright install-deps
        
      - name: Run Unit Tests
        run: montrs test --report junit --output unit-results.xml

      - name: Run E2E Tests
        run: montrs e2e --headless
```
