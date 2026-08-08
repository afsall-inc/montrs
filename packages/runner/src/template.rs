use crate::types::Task;
use std::collections::HashMap;

/// Render Tera templates in task fields.
pub fn render_task(
    task: &mut Task,
    extra_vars: &HashMap<String, String>,
) -> anyhow::Result<()> {
    let mut ctx = tera::Context::new();

    // Add environment variables
    for (key, val) in std::env::vars() {
        ctx.insert(key, &val);
    }

    // Add extra vars (from CLI or config)
    for (key, val) in extra_vars {
        ctx.insert(key, val);
    }

    // Add task fields to context
    ctx.insert("task_name", &task.name);
    ctx.insert("task_dir", &task.dir);
    ctx.insert(
        "config_root",
        &task
            .config_root
            .as_ref()
            .map(|p| p.to_string_lossy().to_string()),
    );

    let mut tera = create_tera();

    // Render description
    if !task.description.is_empty() {
        task.description = tera
            .render_str(&task.description, &ctx)
            .unwrap_or_else(|_| task.description.clone());
    }

    // Render dir
    if let Some(ref dir) = task.dir {
        task.dir =
            Some(tera.render_str(dir, &ctx).unwrap_or_else(|_| dir.clone()));
    }

    // Render env values
    let mut rendered_env = std::collections::HashMap::new();
    for (key, val) in &task.env {
        let rendered =
            tera.render_str(val, &ctx).unwrap_or_else(|_| val.clone());
        rendered_env.insert(key.clone(), rendered);
    }
    task.env.clear();
    for (key, val) in rendered_env {
        task.env.insert(key, val);
    }

    // Render depends
    let mut rendered_deps = Vec::new();
    for dep in &task.depends {
        let task_name = dep.task_name();
        let rendered = tera
            .render_str(task_name, &ctx)
            .unwrap_or_else(|_| task_name.to_string());
        rendered_deps.push(crate::types::TaskDep::Simple(rendered));
    }
    task.depends = rendered_deps;

    Ok(())
}

fn create_tera() -> tera::Tera {
    let mut tera = tera::Tera::default();
    tera.add_raw_template("__dummy__", "").ok();

    // Register custom functions
    tera.register_function(
        "env",
        |args: &HashMap<String, serde_json::Value>| {
            let key = args
                .get("var")
                .or_else(|| args.get("key"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let default = args.get("default").and_then(|v| v.as_str());
            let val = std::env::var(key)
                .ok()
                .or_else(|| default.map(|s| s.to_string()));
            Ok(serde_json::Value::String(val.unwrap_or_default()))
        },
    );

    tera.register_function(
        "cwd",
        |_args: &HashMap<String, serde_json::Value>| {
            let cwd = std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            Ok(serde_json::Value::String(cwd))
        },
    );

    // Register `throw` as a no-op (mise compatibility)
    tera.register_function(
        "throw",
        |_: &HashMap<String, serde_json::Value>| Ok(serde_json::Value::Null),
    );

    tera
}
