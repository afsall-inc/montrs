//! Integration testing plate for MontRS.
//!
//! This plate provides tools for testing interactions between components
//! and managing test environments.
//!
//! It includes:
//! - [`TestEnv`]: For mocking environment variables.
//! - [`TestRuntime`]: For executing app logic in a controlled context.
//! - [`Fixture`]: For managing test setup and teardown.
//!
//! # Example
//!
//! ```rust
//! use montrs_test::integration::{TestEnv, TestRuntime};
//! use montrs_core::{AppSpec, EnvConfig};
//!
//! // Mock the environment
//! let env = TestEnv::new();
//! env.set("DATABASE_URL", "postgres://localhost:5432/test");
//!
//! // Create a runtime (assuming you have a spec)
//! // let runtime = TestRuntime::new(spec);
//! ```

use async_trait::async_trait;
use montrs_core::EnvConfig;
use montrs_core::env::EnvError;
use montrs_core::{AppConfig, AppSpec};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[cfg(feature = "e2e")]
use crate::e2e;

/// A mock environment configuration provider for testing.
///
/// `TestEnv` allows you to programmaticallly set environment variables that are
/// visible only within the test context, avoiding pollution of the real process environment.
///
/// # Example
///
/// ```rust
/// use montrs_test::integration::TestEnv;
/// use montrs_core::EnvConfig;
///
/// let env = TestEnv::new();
/// env.set("API_KEY", "test-secret");
///
/// assert_eq!(env.get_var("API_KEY").unwrap(), "test-secret");
/// ```
pub struct TestEnv {
    vars: Arc<RwLock<HashMap<String, String>>>,
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl TestEnv {
    /// Creates a new, empty `TestEnv`.
    pub fn new() -> Self {
        Self {
            vars: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Sets an environment variable for the test session.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable.
    /// * `value` - The value to assign.
    pub fn set(&self, key: &str, value: &str) {
        let mut vars = self.vars.write().unwrap();
        vars.insert(key.to_string(), value.to_string());
    }
}

impl EnvConfig for TestEnv {
    /// Retrieves a variable from the test environment.
    ///
    /// Returns `EnvError::MissingKey` if the variable is not set.
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        let vars = self.vars.read().unwrap();
        vars.get(key)
            .cloned()
            .ok_or_else(|| EnvError::MissingKey(key.to_string()))
    }
}

/// A trait for defining test fixtures with setup and teardown logic.
///
/// Implement this trait to manage resources that are needed for a test case,
/// such as database connections, temporary files, or mock servers.
///
/// # Example
///
/// ```rust
/// use montrs_test::integration::Fixture;
/// use async_trait::async_trait;
///
/// struct TempFileFixture;
///
/// #[async_trait]
/// impl Fixture for TempFileFixture {
///     type Context = std::path::PathBuf;
///
///     async fn setup(&self) -> anyhow::Result<Self::Context> {
///         Ok(std::env::temp_dir().join("test_file"))
///     }
///
///     async fn teardown(&self, path: &mut Self::Context) -> anyhow::Result<()> {
///         if path.exists() {
///             std::fs::remove_file(path)?;
///         }
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Fixture {
    /// The type of data produced by setup and used by the test.
    type Context;

    /// Sets up the fixture environment.
    ///
    /// This method is called before the test execution. It should return
    /// the context data needed for the test.
    async fn setup(&self) -> anyhow::Result<Self::Context>;

    /// Cleans up the fixture environment.
    ///
    /// This method is called after the test execution, regardless of whether
    /// the test passed or failed.
    async fn teardown(&self, _context: &mut Self::Context) -> anyhow::Result<()> {
        Ok(())
    }
}

/// Runs a test with the given fixture.
///
/// This helper ensures that `setup` is called before the test,
/// and `teardown` is called after, even if the test fails.
///
/// # Arguments
///
/// * `fixture` - The fixture implementation to use.
/// * `test` - The test closure, which receives a mutable reference to the fixture context.
///
/// # Example
///
/// ```rust
/// use montrs_test::integration::{Fixture, run_fixture_test};
/// use async_trait::async_trait;
///
/// struct MyFixture;
///
/// #[async_trait]
/// impl Fixture for MyFixture {
///     type Context = String;
///     async fn setup(&self) -> anyhow::Result<Self::Context> {
///         Ok("hello".to_string())
///     }
/// }
///
/// #[tokio::test]
/// async fn my_test() -> anyhow::Result<()> {
///     run_fixture_test(MyFixture, |ctx| async move {
///         assert_eq!(ctx, "hello");
///         Ok(())
///     }).await
/// }
/// ```
pub async fn run_fixture_test<F, T, Fut>(fixture: F, test: T) -> anyhow::Result<()>
where
    F: Fixture + Send + Sync,
    F::Context: Send,
    T: FnOnce(&mut F::Context) -> Fut + Send,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    let mut context = fixture.setup().await?;

    let result = test(&mut context).await;

    // Always run teardown
    let teardown_result = fixture.teardown(&mut context).await;

    // Return the test result first, or the teardown error if test succeeded but teardown failed
    result.and(teardown_result)
}

/// A specialized runtime for executing MontRS components in a test context.
///
/// `TestRuntime` wraps an `AppSpec` and provides facilities to execute closures
/// against that specification. This is useful for integration tests where you need
/// a fully configured application state.
pub struct TestRuntime<C: AppConfig> {
    /// The application specification being tested.
    pub spec: AppSpec<C>,
}

impl<C: AppConfig> TestRuntime<C> {
    /// Creates a new `TestRuntime` with the provided `AppSpec`.
    pub fn new(spec: AppSpec<C>) -> Self {
        Self { spec }
    }

    /// Executes a closure within the test runtime context.
    ///
    /// This method allows you to run code that requires access to the application
    /// configuration and environment.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a reference to `AppSpec` and returns a result.
    pub async fn execute<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AppSpec<C>) -> R,
    {
        // In a real implementation, this would set up the reactive context,
        // potentially a tokio task local for the runtime, etc.
        f(&self.spec)
    }
}

#[cfg(feature = "e2e")]
impl<C: AppConfig> TestRuntime<C> {
    /// Creates a new E2E driver instance.
    ///
    /// This provides a convenient way to access the E2E capabilities from within
    /// a test runtime context. It automatically connects to the environment
    /// configured for the test.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let runtime = TestRuntime::new(spec);
    /// let driver = runtime.driver().await?;
    /// driver.goto("/").await?;
    /// ```
    pub async fn driver(&self) -> anyhow::Result<e2e::MontrsDriver> {
        e2e::MontrsDriver::new().await
    }
}
