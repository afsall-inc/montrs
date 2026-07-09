use chrono::Utc;
use montrs_agent::{AgentManager, AgentSnapshot, PlateSummary};
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_invariant_dependencies() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let manager = AgentManager::new(root);

    let mut snapshot = AgentSnapshot {
        project_name: "test".to_string(),
        timestamp: Utc::now(),
        framework_version: "0.1.0".to_string(),
        structure: Vec::new(),
        plates: vec![PlateSummary {
            name: "PlateA".to_string(),
            description: "A".to_string(),
            dependencies: vec!["PlateB".to_string()],
            metadata: HashMap::new(),
        }],
        routes: Vec::new(),
        packages: vec![montrs_agent::PackageSummary {
            name: "test-pkg".to_string(),
            path: "packages/test-pkg".to_string(),
            invariants: Some("Must be fast".to_string()),
            description: None,
        }],
        agent_entry_point: Some("docs/agent/index.md".to_string()),
        documentation_snippets: HashMap::new(),
    };

    // Case 1: Missing dependency
    let violations = manager.check_invariants(&snapshot).unwrap();
    assert_eq!(violations.len(), 1);
    assert!(violations[0].contains("depends on missing plate 'PlateB'"));

    // Case 2: Dependency exists
    snapshot.plates.push(PlateSummary {
        name: "PlateB".to_string(),
        description: "B".to_string(),
        dependencies: Vec::new(),
        metadata: HashMap::new(),
    });
    let violations = manager.check_invariants(&snapshot).unwrap();
    assert_eq!(violations.len(), 0);
}

#[test]
fn test_invariant_cycles() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let manager = AgentManager::new(root);

    let snapshot = AgentSnapshot {
        project_name: "test".to_string(),
        timestamp: Utc::now(),
        framework_version: "0.1.0".to_string(),
        structure: Vec::new(),
        plates: vec![
            PlateSummary {
                name: "PlateA".to_string(),
                description: "A".to_string(),
                dependencies: vec!["PlateB".to_string()],
                metadata: HashMap::new(),
            },
            PlateSummary {
                name: "PlateB".to_string(),
                description: "B".to_string(),
                dependencies: vec!["PlateA".to_string()],
                metadata: HashMap::new(),
            },
        ],
        routes: Vec::new(),
        packages: vec![montrs_agent::PackageSummary {
            name: "test-pkg".to_string(),
            path: "packages/test-pkg".to_string(),
            invariants: Some("Must be fast".to_string()),
            description: None,
        }],
        agent_entry_point: Some("docs/agent/index.md".to_string()),
        documentation_snippets: HashMap::new(),
    };

    let violations = manager.check_invariants(&snapshot).unwrap();
    assert!(
        violations
            .iter()
            .any(|v| v.contains("Circular dependency detected"))
    );
}
