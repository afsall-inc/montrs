//! End-to-End (E2E) testing plate for MontRS.

//! This plate provides a wrapper around `playwright-rs` to facilitate writing
//! E2E tests for MontRS applications. It includes configuration management,
//! test driver orchestration, and helper assertions.
//!
//! The E2E plate is built around the [`MontrsDriver`] struct, which manages the
//! Playwright browser instance, context, and page. It abstracts away the
//! complexities of managing browser lifecycles in tests.
//!
//! # Example
//!
//! ```rust
//! use montrs_test::e2e::MontrsDriver;
//!
//! #[tokio::test]
//! async fn test_homepage() -> anyhow::Result<()> {
//!     let driver = MontrsDriver::new().await?;
//!     driver.goto("http://localhost:3000").await?;
//!     driver.assert_text("h1", "Welcome to MontRS").await?;
//!     Ok(())
//! }
//! ```

use playwright::Playwright;
use playwright::api::{Browser, BrowserContext, BrowserType, Page};
use std::env;
use std::time::Duration;

// Re-export playwright so users don't need to add it separately if they don't want to
pub use playwright;

/// Configuration for the E2E test session.
///
/// This struct controls how the browser is launched and how the test driver behaves.
/// It automatically resolves defaults from environment variables.
///
/// # Environment Variables
///
/// - `MONTRS_SITE_URL`: The base URL of the application under test.
/// - `LEPTOS_SITE_ADDR`: Alternative source for the base URL (e.g. `127.0.0.1:8080`).
/// - `MONTRS_E2E_HEADLESS`: Set to `false` to run in headful mode (default: `true`).
#[derive(Debug, Clone)]
pub struct E2EConfig {
    /// Whether to run the browser in headless mode.
    pub headless: bool,
    /// The base URL for the application.
    pub base_url: String,
    /// The default timeout for operations in milliseconds.
    pub timeout: u32,
    /// The browser engine to use (chromium, firefox, webkit).
    pub browser: String,
}

impl Default for E2EConfig {
    fn default() -> Self {
        let base_url = env::var("MONTRS_SITE_URL")
            .or_else(|_| env::var("LEPTOS_SITE_ADDR").map(|addr| format!("http://{}", addr)))
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

        Self {
            headless: env::var("MONTRS_E2E_HEADLESS")
                .map(|v| v != "false")
                .unwrap_or(true),
            base_url,
            timeout: 30000,
            browser: env::var("MONTRS_E2E_BROWSER").unwrap_or_else(|_| "chromium".to_string()),
        }
    }
}

/// A trait for extending MontrsDriver functionality.
///
/// Implement this trait to create reusable plugins for your E2E tests,
/// such as authentication helpers or custom logging.
#[async_trait::async_trait]
pub trait MontrsPlugin {
    /// Called immediately after the driver is initialized.
    ///
    /// Use this hook to perform setup actions like logging in or mocking network requests.
    async fn on_init(&self, driver: &MontrsDriver) -> anyhow::Result<()>;
}

/// The main driver for MontRS E2E tests.
///
/// Wraps Playwright to provide an ergonomic, framework-aware testing experience.
/// It manages the lifecycle of the browser, context, and page.
pub struct MontrsDriver {
    /// The underlying Playwright instance.
    pub playwright: Playwright,
    /// The active browser instance.
    pub browser: Browser,
    /// The isolated browser context for this driver.
    pub context: BrowserContext,
    /// The main page object for interacting with the web content.
    pub page: Page,
    /// The active configuration.
    pub config: E2EConfig,
}

impl MontrsDriver {
    /// Launches a new browser instance and prepares a page for testing.
    ///
    /// Uses the default configuration resolved from environment variables.
    pub async fn new() -> anyhow::Result<Self> {
        Self::with_config(E2EConfig::default()).await
    }

    /// Launches with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use for this session.
    pub async fn with_config(config: E2EConfig) -> anyhow::Result<Self> {
        let playwright = Playwright::initialize().await?;
        playwright.prepare()?; // Install browsers if needed

        let browser_type = match config.browser.as_str() {
            "firefox" => playwright.firefox(),
            "webkit" => playwright.webkit(),
            _ => playwright.chromium(),
        };

        let launcher = browser_type.launcher();

        let browser = launcher
            .headless(config.headless)
            .timeout(config.timeout as f64)
            .launch()
            .await?;

        let context = browser.context_builder().build().await?;
        let page = context.new_page().await?;

        Ok(Self {
            playwright,
            browser,
            context,
            page,
            config,
        })
    }

    /// Register and run a plugin.
    ///
    /// # Arguments
    ///
    /// * `plugin` - The plugin to execute.
    pub async fn use_plugin<P: MontrsPlugin>(&self, plugin: P) -> anyhow::Result<&Self> {
        plugin.on_init(self).await?;
        Ok(self)
    }

    /// Navigates to a path relative to the configured base URL.
    ///
    /// If an absolute URL is provided (starting with `http://` or `https://`),
    /// it is used as-is. Otherwise, it is appended to `config.base_url`.
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path or absolute URL.
    pub async fn goto(&self, path: &str) -> anyhow::Result<()> {
        let url = if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            // Ensure proper slash handling
            let base = self.config.base_url.trim_end_matches('/');
            let path = path.trim_start_matches('/');
            format!("{}/{}", base, path)
        };

        self.page.goto_builder(&url).goto().await?;
        Ok(())
    }

    /// Returns the current URL of the page.
    pub fn url(&self) -> String {
        self.page.url().unwrap_or_default()
    }

    /// Reloads the current page.
    pub async fn reload(&self) -> anyhow::Result<()> {
        self.page.reload_builder().reload().await?;
        Ok(())
    }

    /// Takes a screenshot and saves it to the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to save the screenshot to.
    pub async fn screenshot(&self, path: &str) -> anyhow::Result<()> {
        self.page
            .screenshot_builder()
            .path(std::path::Path::new(path))
            .screenshot()
            .await?;
        Ok(())
    }

    /// Closes the browser and context.
    ///
    /// This should be called at the end of the test to ensure resources are released.
    pub async fn close(&self) -> anyhow::Result<()> {
        self.context.close().await?;
        self.browser.close().await?;
        Ok(())
    }
}

/// Helper assertions for MontRS applications.
///
/// These functions provide common verification steps for E2E tests.
pub mod assertions {
    use playwright::api::Page;

    /// Asserts that the page title contains the expected text.
    ///
    /// # Arguments
    ///
    /// * `page` - The Playwright Page object.
    /// * `text` - The text to search for in the title.
    pub async fn assert_title_contains(page: &Page, text: &str) -> anyhow::Result<()> {
        let title = page.title().await?;
        if !title.contains(text) {
            anyhow::bail!(
                "Expected title to contain '{}', but found '{}'",
                text,
                title
            );
        }
        Ok(())
    }

    /// Asserts that an element exists on the page.
    ///
    /// # Arguments
    ///
    /// * `page` - The Playwright Page object.
    /// * `selector` - The CSS selector of the element.
    pub async fn assert_element_exists(page: &Page, selector: &str) -> anyhow::Result<()> {
        match page.query_selector(selector).await? {
            Some(_) => Ok(()),
            None => anyhow::bail!("Element '{}' not found", selector),
        }
    }
}
