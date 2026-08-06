//! Invariant tests for montrs-runner.

use montrs_runner::{TaskConfig, TaskRunner};
use std::collections::HashMap;

#[test]
fn test_task_config_simple() {
    let config = TaskConfig::Simple("cargo build".to_string());
    match config {
        TaskConfig::Simple(cmd) => assert_eq!(cmd, "cargo build"),
        _ => panic!("expected Simple variant"),
    }
}

#[test]
fn test_task_config_detailed() {
    let config = TaskConfig::Detailed {
        command: "cargo test".to_string(),
        description: Some("Run tests".to_string()),
        category: Some("testing".to_string()),
        dependencies: vec!["build".to_string()],
        env: HashMap::new(),
    };
    match config {
        TaskConfig::Detailed {
            command,
            description,
            ..
        } => {
            assert_eq!(command, "cargo test");
            assert_eq!(description.unwrap(), "Run tests");
        }
        _ => panic!("expected Detailed variant"),
    }
}

#[test]
fn test_task_runner_empty() {
    let runner = TaskRunner::new(HashMap::new());
    assert!(runner.list().is_ok());
}

#[test]
fn test_task_runner_list() {
    let mut tasks = HashMap::new();
    tasks.insert(
        "build".to_string(),
        TaskConfig::Simple("cargo build".to_string()),
    );
    let runner = TaskRunner::new(tasks);
    assert!(runner.list().is_ok());
}

#[test]
fn test_task_runner_run_nonexistent() {
    let runner = TaskRunner::new(HashMap::new());
    let result = runner.run("nonexistent");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let err = rt.block_on(result).unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[test]
fn test_task_config_debug_and_clone() {
    let config = TaskConfig::Simple("echo hello".to_string());
    let cloned = config.clone();
    assert_eq!(format!("{:?}", config), format!("{:?}", cloned));
}
