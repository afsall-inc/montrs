//! Test helpers for montrs-runner.

use crate::types::Task;
use crate::TaskRunner;
use std::collections::HashMap;

pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .try_init();
}

pub fn empty_runner() -> TaskRunner {
    TaskRunner::new(HashMap::new())
}

pub fn simple_task_runner() -> TaskRunner {
    let mut tasks = HashMap::new();
    tasks.insert(
        "build".to_string(),
        Task {
            name: "build".to_string(),
            command: vec![crate::types::RunEntry::Script(
                "echo build".to_string(),
            )],
            ..Default::default()
        },
    );
    tasks.insert(
        "test".to_string(),
        Task {
            name: "test".to_string(),
            command: vec![crate::types::RunEntry::Script(
                "echo test".to_string(),
            )],
            ..Default::default()
        },
    );
    TaskRunner::new(tasks)
}

pub struct TestContext {
    pub runner: TaskRunner,
}

impl TestContext {
    pub fn new() -> Self {
        init_test_tracing();
        Self {
            runner: empty_runner(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}