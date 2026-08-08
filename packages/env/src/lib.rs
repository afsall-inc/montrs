/// Environment directive types — how a single env var is defined.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single environment variable directive from `[env]` section.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvDirective {
    /// Simple string value: `FOO = "bar"`
    Value(String),
    /// Structured: `FOO = { value = "bar", export = true }`
    Structured(StructuredEnv),
    /// Path manipulation: `_.path = ["path1", "path2"]` or `_.path = "path"`
    Path(PathDirective),
}

/// Structured env var with options.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StructuredEnv {
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub export: Option<bool>,
    #[serde(default)]
    pub redact: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub expand: Option<bool>,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

/// Path manipulation directive for `_.path`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PathDirective {
    #[serde(default)]
    pub add: Vec<String>,
    #[serde(default)]
    pub prepend: Vec<String>,
    #[serde(default)]
    pub remove: Vec<String>,
    #[serde(default)]
    pub default: Vec<String>,
}

/// Parsed environment from the `[env]` section of montrs.toml.
#[derive(Debug, Clone, Default)]
pub struct Environment {
    /// Direct env vars (key → value).
    pub vars: HashMap<String, String>,
    /// Whether each var should be exported (default true).
    pub exports: HashMap<String, bool>,
    /// Path manipulation directives.
    pub path: PathState,
}

/// The state of PATH after applying all directives.
#[derive(Debug, Clone, Default)]
pub struct PathState {
    pub prepend: Vec<String>,
    pub append: Vec<String>,
    pub remove: Vec<String>,
}

/// Parse a raw `[env]` section from montrs.toml into a list of directives.
pub fn parse_env_section(
    raw: &HashMap<String, toml::Value>,
) -> Vec<(String, EnvDirective)> {
    let mut directives = Vec::new();
    for (key, value) in raw {
        if key == "_" {
            if let Some(table) = value.as_table()
                && let Some(path_val) = table.get("path")
            {
                let path_dir = parse_path_directive(path_val);
                directives
                    .push(("_".to_string(), EnvDirective::Path(path_dir)));
            }
        } else {
            match value {
                toml::Value::String(s) => {
                    directives
                        .push((key.clone(), EnvDirective::Value(s.clone())));
                }
                toml::Value::Table(t) => {
                    let structured = StructuredEnv {
                        value: t
                            .get("value")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        export: t.get("export").and_then(|v| v.as_bool()),
                        redact: t.get("redact").and_then(|v| v.as_bool()),
                        required: t
                            .get("required")
                            .map(|v| !matches!(v, toml::Value::Boolean(false))),
                        expand: t.get("expand").and_then(|v| v.as_bool()),
                        file: t
                            .get("file")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        source: t
                            .get("source")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                    };
                    directives.push((
                        key.clone(),
                        EnvDirective::Structured(structured),
                    ));
                }
                _ => {}
            }
        }
    }
    directives
}

fn parse_path_directive(value: &toml::Value) -> PathDirective {
    let mut dir = PathDirective::default();
    match value {
        toml::Value::String(s) => {
            dir.prepend.push(s.clone());
        }
        toml::Value::Array(arr) => {
            for v in arr {
                if let Some(s) = v.as_str() {
                    dir.prepend.push(s.to_string());
                }
            }
        }
        toml::Value::Table(t) => {
            if let Some(add) = t.get("add").and_then(|v| v.as_array()) {
                dir.add = add
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }
            if let Some(prepend) = t.get("prepend").and_then(|v| v.as_array()) {
                dir.prepend = prepend
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }
            if let Some(remove) = t.get("remove").and_then(|v| v.as_array()) {
                dir.remove = remove
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }
            if let Some(default) = t.get("default").and_then(|v| v.as_array()) {
                dir.default = default
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }
        }
        _ => {}
    }
    dir
}

/// Render Tera templates in env values.
pub fn render_env_values(
    directives: &mut [(String, EnvDirective)],
    extra_vars: &HashMap<String, String>,
) -> anyhow::Result<()> {
    let mut ctx = tera::Context::new();
    for (key, val) in std::env::vars() {
        ctx.insert(key, &val);
    }
    for (key, val) in extra_vars {
        ctx.insert(key, val);
    }

    let mut tera = create_tera();

    for (_, directive) in directives.iter_mut() {
        match directive {
            EnvDirective::Value(val) => {
                if contains_template(val) {
                    *val = tera
                        .render_str(val, &ctx)
                        .unwrap_or_else(|_| val.clone());
                }
            }
            EnvDirective::Structured(s) => {
                if let Some(ref value) = s.value
                    && contains_template(value)
                {
                    s.value = Some(
                        tera.render_str(value, &ctx)
                            .unwrap_or_else(|_| value.clone()),
                    );
                }
            }
            EnvDirective::Path(p) => {
                for list in [&mut p.prepend, &mut p.add, &mut p.remove] {
                    for val in list.iter_mut() {
                        if contains_template(val) {
                            *val = tera
                                .render_str(val, &ctx)
                                .unwrap_or_else(|_| val.clone());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn contains_template(s: &str) -> bool {
    s.contains("{{") || s.contains("{%")
}

fn create_tera() -> tera::Tera {
    let mut tera = tera::Tera::default();
    tera.add_raw_template("__dummy__", "").ok();
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
                .or_else(|| default.map(String::from));
            Ok(serde_json::Value::String(val.unwrap_or_default()))
        },
    );
    tera.register_function("cwd", |_: &HashMap<String, serde_json::Value>| {
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        Ok(serde_json::Value::String(cwd))
    });
    tera
}

/// Resolve directives into a final Environment, reading from current env.
pub fn resolve_environment(
    directives: &[(String, EnvDirective)],
) -> Environment {
    let mut env = Environment::default();

    for (key, directive) in directives {
        match directive {
            EnvDirective::Value(val) => {
                env.vars.insert(key.clone(), val.clone());
                env.exports.insert(key.clone(), true);
            }
            EnvDirective::Structured(s) => {
                if let Some(ref val) = s.value {
                    env.vars.insert(key.clone(), val.clone());
                } else if s.file.is_some() || s.source.is_some() {
                    // load from file or source — stub for now
                }
                let export = s.export.unwrap_or(true);
                env.exports.insert(key.clone(), export);
            }
            EnvDirective::Path(p) => {
                env.path.prepend.extend(p.prepend.clone());
                env.path.append.extend(p.add.clone());
                env.path.remove.extend(p.remove.clone());
            }
        }
    }

    env
}

/// Apply the resolved environment to the current process.
pub fn apply_environment(env: &Environment) {
    for (key, val) in &env.vars {
        let export = env.exports.get(key).copied().unwrap_or(true);
        if export {
            unsafe {
                std::env::set_var(key, val);
            }
        }
    }

    // Apply PATH modifications
    let current_path = std::env::var("PATH").unwrap_or_default();
    let mut parts: Vec<String> = std::env::split_paths(&current_path)
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    // Remove specified paths
    parts.retain(|p| !env.path.remove.contains(p));

    // Prepend paths
    for p in env.path.prepend.iter().rev() {
        parts.insert(0, p.clone());
    }

    // Append paths
    for p in &env.path.append {
        parts.push(p.clone());
    }

    let new_path = std::env::join_paths(&parts)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(current_path);
    unsafe {
        std::env::set_var("PATH", &new_path);
    }
}

/// Load environment from a `.env` file.
pub fn load_dotenv(
    path: &std::path::Path,
) -> anyhow::Result<HashMap<String, String>> {
    let content = std::fs::read_to_string(path)?;
    let mut vars = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim().to_string();
            let val = val.trim().trim_matches('"').to_string();
            vars.insert(key, val);
        }
    }
    Ok(vars)
}

/// Compute the diff between two environments.
#[derive(Debug, Clone, Default)]
pub struct EnvDiff {
    pub set: HashMap<String, String>,
    pub unset: Vec<String>,
}

impl EnvDiff {
    pub fn compute(
        before: &HashMap<String, String>,
        after: &HashMap<String, String>,
    ) -> Self {
        let mut set = HashMap::new();
        let mut unset = Vec::new();

        for (key, val) in after {
            match before.get(key) {
                Some(before_val) if before_val != val => {
                    set.insert(key.clone(), val.clone());
                }
                None => {
                    set.insert(key.clone(), val.clone());
                }
                _ => {}
            }
        }

        for key in before.keys() {
            if !after.contains_key(key) {
                unset.push(key.clone());
            }
        }

        Self { set, unset }
    }
}
