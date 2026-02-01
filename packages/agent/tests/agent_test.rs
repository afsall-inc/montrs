use montrs_agent::AgentManager;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_agent_generation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a dummy file
    fs::write(root.join("test.rs"), "fn main() {}").unwrap();

    let manager = AgentManager::new(root);
    let snapshot = manager.generate_snapshot("test-project").unwrap();

    assert_eq!(snapshot.project_name, "test-project");
    assert!(snapshot.structure.iter().any(|f| f.path == "test.rs"));
    assert!(snapshot.documentation_snippets.contains_key("architecture"));

    manager.write_snapshot(&snapshot, "json").unwrap();
    assert!(root.join(".agent/agent.json").exists());
}

#[tokio::test]
async fn test_error_reporting() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let manager = AgentManager::new(root);
    let error_id = manager
        .report_project_error(montrs_agent::ProjectError {
            package: None,
            file: "unknown".to_string(),
            line: 0,
            column: 0,
            message: "Something went wrong".to_string(),
            code_context: "".to_string(),
            level: "Error".to_string(),
            agent_metadata: None,
        })
        .unwrap();

    let error_group_dir = root.join(".agent/errorfiles").join(&error_id);
    assert!(error_group_dir.exists());
    assert!(error_group_dir.join("v1.json").exists());

    let content = fs::read_to_string(error_group_dir.join("v1.json")).unwrap();
    assert!(content.contains("Something went wrong"));
}

#[tokio::test]
async fn test_consolidated_error_tracking() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create packages structure
    let pkg_dir = root.join("packages/test-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();
    let file_path = pkg_dir.join("src/lib.rs");
    fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    fs::write(&file_path, "fn error() {}").unwrap();

    let manager = AgentManager::new(root);
    let error_file = "packages/test-pkg/src/lib.rs";

    manager
        .report_project_error(montrs_agent::ProjectError {
            package: None,
            file: error_file.to_string(),
            line: 10,
            column: 5,
            message: "Test error".to_string(),
            code_context: "fn error() {}".to_string(),
            level: "Error".to_string(),
            agent_metadata: None,
        })
        .unwrap();

    assert!(root.join(".agent/errorfiles/error_tracking.json").exists());
    let tracking = manager.load_tracking().unwrap();
    assert_eq!(tracking.errors.len(), 1);
    assert_eq!(tracking.errors[0].package, Some("test-pkg".to_string()));
    assert_eq!(tracking.errors[0].status, "Pending");
}
