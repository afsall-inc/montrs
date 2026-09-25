// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The deterministic test harness: one object that owns the app spec, a mocked
//! environment, a controllable clock, and a seeded RNG.

use crate::{
    integration::{Fixture, TestEnv},
    kernel::{
        clock::{Clock, TestClock},
        rng::{Rng, TestRng},
    },
};
use montrs_core::{AppConfig, AppSpec, Owner, provide_context};
use std::{sync::Arc, time::Duration};

/// An application (or test double) that can be tested without mocks.
///
/// Implement this for your real app type so it acts as its own mock:
///
/// ```rust,ignore
/// struct MyApp;
/// impl TestApp for MyApp {
///     type Config = MyConfig;
///     fn spec() -> AppSpec<Self::Config> { crate::build_spec() }
/// }
///
/// let harness = TestHarness::for_app::<MyApp>();
/// ```
pub trait TestApp {
    /// The app's configuration type.
    type Config: AppConfig;

    /// Build the application spec (the real one).
    fn spec() -> AppSpec<Self::Config>;
}

/// The baked-in responsive preset: standard devices plus default breakpoints.
#[cfg(feature = "layout")]
#[derive(Debug, Clone)]
pub struct ResponsiveDefaults {
    /// Devices checked by default.
    pub devices: Vec<crate::devices::DeviceProfile>,
    /// Breakpoints used for responsive resolution.
    pub breakpoints: crate::layout::Breakpoints,
}

#[cfg(feature = "layout")]
impl ResponsiveDefaults {
    /// Start a responsive check for `html` with these defaults applied.
    pub fn check(&self, html: &str) -> crate::responsive::ResponsiveCheck {
        crate::responsive::ResponsiveCheck::new(html)
            .with_breakpoints(self.breakpoints)
    }

    /// Assert `html` is responsive on every default device.
    #[track_caller]
    pub fn assert_responsive(&self, html: &str) {
        self.check(html).assert_responsive_on(&self.devices);
    }
}

/// A configured, deterministic context for testing a MontRS application.
///
/// ```rust,ignore
/// let harness = TestHarness::new(build_spec())
///     .seed(42)
///     .with_clock(Arc::new(TestClock::new()));
/// harness.execute(|spec| { /* … */ });
/// ```
pub struct TestHarness<C: AppConfig> {
    /// The application spec under test.
    pub spec: AppSpec<C>,
    /// The mocked environment.
    pub env: TestEnv,
    /// The injected clock (deterministic by default).
    pub clock: Arc<dyn Clock>,
    /// The injected RNG (seeded by default).
    pub rng: Arc<dyn Rng>,
}

impl<C: AppConfig> TestHarness<C> {
    /// Create a harness for an application implementing [`TestApp`].
    pub fn for_app<A: TestApp<Config = C>>() -> Self {
        Self::new(A::spec())
    }

    /// Create a harness with a [`TestClock`] and a fixed-seed [`TestRng`].
    pub fn new(spec: AppSpec<C>) -> Self {
        Self {
            spec,
            env: TestEnv::new(),
            clock: Arc::new(TestClock::new()),
            rng: Arc::new(TestRng::default()),
        }
    }

    /// Use a specific clock (e.g. [`SystemClock`] for real-time tests).
    pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }

    /// Use a specific RNG.
    pub fn with_rng(mut self, rng: Arc<dyn Rng>) -> Self {
        self.rng = rng;
        self
    }

    /// Seed the built-in [`TestRng`] for reproducibility.
    pub fn seed(mut self, seed: u64) -> Self {
        self.rng = Arc::new(TestRng::seed(seed));
        self
    }

    /// Set an environment variable visible only to this harness.
    pub fn with_env(self, key: &str, value: &str) -> Self {
        self.env.set(key, value);
        self
    }

    /// Advance the clock if it is a [`TestClock`]; a no-op otherwise.
    pub fn advance(&self, delta: Duration) {
        if let Some(test_clock) = self.as_test_clock() {
            test_clock.advance(delta);
        }
    }

    /// Advance the clock by milliseconds if it is a [`TestClock`].
    pub fn advance_ms(&self, ms: u64) {
        self.advance(Duration::from_millis(ms));
    }

    fn as_test_clock(&self) -> Option<&TestClock> {
        self.clock.as_any().downcast_ref::<TestClock>()
    }

    /// Run a closure inside the Leptos reactive context with the app spec
    /// provided, mirroring how the real app is mounted.
    pub fn execute<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AppSpec<C>) -> R,
    {
        let owner = Owner::new();
        owner.with(|| {
            provide_context(self.spec.clone());
            f(&self.spec)
        })
    }

    /// Run a fixture around this harness. Teardown always runs, even when the
    /// test fails (the fixture's error is returned only if the test itself
    /// succeeded).
    pub async fn with_fixture<F, T, Fut>(
        &self,
        fixture: F,
        test: T,
    ) -> anyhow::Result<()>
    where
        F: Fixture + Send + Sync,
        F::Context: Send,
        T: FnOnce(&Self, &mut F::Context) -> Fut + Send,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
    {
        let mut context = fixture.setup().await?;
        let result = test(self, &mut context).await;
        let teardown = fixture.teardown(&mut context).await;
        result.and(teardown)
    }

    /// The baked-in responsive preset (standard devices + default breakpoints).
    ///
    /// ```rust,ignore
    /// let devices = harness.responsive_defaults();
    /// devices.assert_responsive(view.html());
    /// ```
    #[cfg(feature = "layout")]
    pub fn responsive_defaults(&self) -> ResponsiveDefaults {
        ResponsiveDefaults {
            devices: crate::devices::standard_devices(),
            breakpoints: crate::layout::Breakpoints::default(),
        }
    }
}

/// E2E convenience: launch a browser driver for this harness's environment.
#[cfg(feature = "e2e")]
impl<C: AppConfig> TestHarness<C> {
    pub async fn driver(&self) -> anyhow::Result<crate::e2e::MontrsDriver> {
        crate::e2e::MontrsDriver::new().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use montrs_core::{AppConfig, AppSpec, EnvConfig, env::EnvError};

    #[derive(Clone)]
    struct Env;
    impl EnvConfig for Env {
        fn get_var(&self, _key: &str) -> Result<String, EnvError> {
            Err(EnvError::MissingKey(String::new()))
        }
    }

    #[derive(Clone)]
    struct Cfg;
    impl AppConfig for Cfg {
        type Error = std::io::Error;
        type Env = Env;
    }

    #[test]
    fn harness_defaults_are_deterministic() {
        let harness = TestHarness::new(AppSpec::new(Cfg, Env));
        let a = harness.rng.next_u64();

        let fresh = TestHarness::new(AppSpec::new(Cfg, Env));
        let b = fresh.rng.next_u64();
        assert_eq!(a, b, "default RNG must be seeded deterministically");
    }

    #[test]
    fn seed_changes_sequence() {
        let h1 = TestHarness::new(AppSpec::new(Cfg, Env)).seed(1);
        let h2 = TestHarness::new(AppSpec::new(Cfg, Env)).seed(2);
        assert_ne!(h1.rng.next_u64(), h2.rng.next_u64());
    }

    #[test]
    fn advance_moves_test_clock() {
        let harness = TestHarness::new(AppSpec::new(Cfg, Env));
        harness.advance_ms(500);
        assert_eq!(harness.clock.now(), Duration::from_millis(500));
    }

    #[test]
    fn env_is_isolated() {
        let harness =
            TestHarness::new(AppSpec::new(Cfg, Env)).with_env("K", "v");
        use montrs_core::EnvConfig;
        assert_eq!(harness.env.get_var("K").unwrap(), "v");
        assert!(harness.env.get_var("MISSING").is_err());
    }
}
