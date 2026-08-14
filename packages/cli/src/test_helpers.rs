//! Test helpers for montrs-cli.

use crate::*;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn default_cli() -> MontrsCli {
    MontrsCli {
        command: Commands::Build,
        release: false,
        hot_reload: false,
        features: Vec::new(),
        verbose: 0,
        log: Vec::new(),
    }
}

pub struct TestContext {
    pub cli: MontrsCli,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            cli: default_cli(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}