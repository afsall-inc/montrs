use crate::types::{RunEntry, Task, TaskDep};
use indexmap::IndexMap;
use std::path::Path;

/// Parse tasks from a `montrs.toml` `[tasks]` section (raw value map).
pub fn parse_tasks_from_toml(
    raw: std::collections::HashMap<String, toml::Value>,
    config_root: &Path,
) -> Vec<Task> {
    let mut tasks = Vec::new();
    for (name, value) in raw {
        let task = parse_single_task(name, value, config_root);
        tasks.push(task);
    }
    tasks
}

fn parse_single_task(
    name: String,
    value: toml::Value,
    config_root: &Path,
) -> Task {
    let mut task = Task {
        name: name.clone(),
        config_source: Some(config_root.join("montrs.toml")),
        config_root: Some(config_root.to_path_buf()),
        ..Default::default()
    };

    match value {
        toml::Value::String(s) => {
            task.command = vec![RunEntry::Script(s)];
        }
        toml::Value::Table(table) => {
            if let Some(val) = table.get("description").and_then(|v| v.as_str())
            {
                task.description = val.to_string();
            }
            if let Some(val) =
                table.get("display_name").and_then(|v| v.as_str())
            {
                task.display_name = val.to_string();
            }
            if let Some(val) = table.get("dir").and_then(|v| v.as_str()) {
                task.dir = Some(val.to_string());
            }
            if let Some(val) = table.get("shell").and_then(|v| v.as_str()) {
                task.shell = Some(val.to_string());
            }
            if let Some(val) = table.get("usage").and_then(|v| v.as_str()) {
                task.usage = val.to_string();
            }
            if let Some(val) = table.get("file").and_then(|v| v.as_str()) {
                task.file = Some(val.to_string());
            }
            if let Some(val) = table.get("extends").and_then(|v| v.as_str()) {
                task.extends = Some(val.to_string());
            }

            // aliases
            if let Some(arr) = table.get("aliases").and_then(|v| v.as_array()) {
                task.aliases = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }

            // depends
            if let Some(deps) =
                table.get("depends").or_else(|| table.get("dependencies"))
            {
                task.depends = parse_dep_list(deps);
            }

            // command/run
            if let Some(cmd) = table.get("run").or_else(|| table.get("command"))
            {
                task.command = parse_run_entries(cmd);
            }

            // env
            if let Some(env) = table.get("env").and_then(|v| v.as_table()) {
                for (k, v) in env {
                    if let Some(s) = v.as_str() {
                        task.env.insert(k.clone(), s.to_string());
                    }
                }
            }

            // vars
            if let Some(vars) = table.get("vars").and_then(|v| v.as_table()) {
                for (k, v) in vars {
                    if let Some(s) = v.as_str() {
                        task.vars.insert(k.clone(), s.to_string());
                    }
                }
            }

            // sources
            if let Some(arr) = table.get("sources").and_then(|v| v.as_array()) {
                task.sources = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }

            // hide, raw, interactive, quiet, global
            if let Some(b) = table.get("hide").and_then(|v| v.as_bool()) {
                task.hide = b;
            }
            if let Some(b) = table.get("raw").and_then(|v| v.as_bool()) {
                task.raw = b;
            }
            if let Some(b) = table.get("interactive").and_then(|v| v.as_bool())
            {
                task.interactive = b;
            }
            if let Some(b) = table.get("quiet").and_then(|v| v.as_bool()) {
                task.quiet = b;
            }
            if let Some(b) = table.get("global").and_then(|v| v.as_bool()) {
                task.global = b;
            }

            // tools
            if let Some(tools) = table.get("tools").and_then(|v| v.as_table()) {
                for (k, v) in tools {
                    if let Some(s) = v.as_str() {
                        task.tools.insert(
                            k.clone(),
                            crate::types::TaskToolValue::String(s.to_string()),
                        );
                    }
                }
            }

            // timeout
            if let Some(val) = table.get("timeout").and_then(|v| v.as_str()) {
                task.timeout = Some(val.to_string());
            }
        }
        _ => {}
    }

    // If command is empty, try to find a file-based task
    if task.command.is_empty() && task.file.is_none() {
        let file_path = config_root.join("tasks").join(format!("{name}.sh"));
        if file_path.exists() {
            task.file = Some(file_path.to_string_lossy().to_string());
        }
    }

    task
}

fn parse_dep_list(value: &toml::Value) -> Vec<TaskDep> {
    match value {
        toml::Value::Array(arr) => arr
            .iter()
            .map(|v| match v {
                toml::Value::String(s) => TaskDep::Simple(s.clone()),
                toml::Value::Table(t) => TaskDep::Detailed {
                    task: t
                        .get("task")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    args: t
                        .get("args")
                        .and_then(|v| v.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| {
                                    v.as_str().map(|s| s.to_string())
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    env: IndexMap::new(),
                    optional: t
                        .get("optional")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                },
                _ => TaskDep::Simple(v.to_string()),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn parse_run_entries(value: &toml::Value) -> Vec<RunEntry> {
    match value {
        toml::Value::String(s) => vec![RunEntry::Script(s.clone())],
        toml::Value::Array(arr) => arr
            .iter()
            .map(|v| match v {
                toml::Value::String(s) => RunEntry::Script(s.clone()),
                toml::Value::Table(t) => {
                    if let Some(task) = t.get("task").and_then(|v| v.as_str()) {
                        RunEntry::SingleTask {
                            task: task.to_string(),
                            args: t
                                .get("args")
                                .and_then(|v| v.as_array())
                                .map(|a| {
                                    a.iter()
                                        .filter_map(|v| {
                                            v.as_str().map(|s| s.to_string())
                                        })
                                        .collect()
                                })
                                .unwrap_or_default(),
                            env: IndexMap::new(),
                        }
                    } else if let Some(tasks) =
                        t.get("tasks").and_then(|v| v.as_array())
                    {
                        RunEntry::TaskGroup {
                            tasks: tasks
                                .iter()
                                .filter_map(|v| {
                                    v.as_str().map(|s| s.to_string())
                                })
                                .collect(),
                        }
                    } else {
                        RunEntry::Script(v.to_string())
                    }
                }
                _ => RunEntry::Script(v.to_string()),
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Expand `:` separator in task specs (e.g. `build:test`).
pub fn expand_colon_task_syntax(spec: &str) -> (&str, Vec<&str>) {
    let parts: Vec<&str> = spec.split(':').collect();
    if parts.len() > 1 {
        (parts[0], parts[1..].to_vec())
    } else {
        (spec, vec![])
    }
}
