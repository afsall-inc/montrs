use crate::types::Task;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Discover tasks across a monorepo workspace.
pub struct Workspace {
    pub root: std::path::PathBuf,
    pub projects: Vec<PathBuf>,
}

impl Workspace {
    /// Discover workspace members from a root directory.
    pub fn discover(root: &Path) -> Self {
        let mut projects = Vec::new();

        // Look for a workspace root marker (Cargo.toml with [workspace])
        let root_is_workspace = root.join("Cargo.toml").is_file()
            && std::fs::read_to_string(root.join("Cargo.toml"))
                .map(|s| s.contains("[workspace]"))
                .unwrap_or(false);

        if root_is_workspace {
            projects.push(root.to_path_buf());
            // Discover member directories
            if let Some(members) = discover_cargo_members(root) {
                projects.extend(members);
            }
        } else {
            projects.push(root.to_path_buf());
        }

        Self {
            root: root.to_path_buf(),
            projects,
        }
    }

    /// Gather all tasks from all projects.
    pub fn all_tasks(&self) -> Vec<Task> {
        let mut tasks = Vec::new();
        let mut seen = HashMap::new();

        for project in &self.projects {
            let montrs_toml = project.join("montrs.toml");
            if !montrs_toml.exists() {
                continue;
            }

            if let Ok(content) = std::fs::read_to_string(&montrs_toml)
                && let Ok(doc) = content.parse::<toml::Value>()
                && let Some(tasks_table) =
                    doc.get("tasks").and_then(|t| t.as_table())
            {
                let mut raw = HashMap::new();
                for (name, value) in tasks_table {
                    raw.insert(name.clone(), value.clone());
                }
                let project_tasks =
                    crate::parser::parse_tasks_from_toml(raw, project);
                for task in project_tasks {
                    if !seen.contains_key(&task.name) {
                        seen.insert(task.name.clone(), project.clone());
                        tasks.push(task);
                    }
                }
            }
        }

        tasks
    }
}

fn discover_cargo_members(root: &Path) -> Option<Vec<std::path::PathBuf>> {
    let content = std::fs::read_to_string(root.join("Cargo.toml")).ok()?;
    let doc: toml::Value = content.parse().ok()?;
    let members = doc.get("workspace")?.get("members")?.as_array()?;
    Some(
        members
            .iter()
            .filter_map(|m| m.as_str())
            .map(|m| root.join(m))
            .filter(|p| p.join("montrs.toml").exists())
            .collect(),
    )
}
