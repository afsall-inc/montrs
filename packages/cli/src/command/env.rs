use crate::config::MontrsConfig;
use montrs_env::{
    apply_environment, parse_env_section, render_env_values,
    resolve_environment,
};
use std::{collections::HashMap, process::Command};

fn load_env() -> anyhow::Result<Vec<(String, montrs_env::EnvDirective)>> {
    let config = MontrsConfig::load()?;
    let raw = config.meta.env.vars.clone();
    let mut dirs = parse_env_section(&raw);
    render_env_values(&mut dirs, &HashMap::new())?;
    Ok(dirs)
}

pub async fn list() -> anyhow::Result<()> {
    let dirs = load_env()?;
    let env = resolve_environment(&dirs);

    if env.vars.is_empty() {
        println!("No environment variables defined in montrs.toml [env]");
        return Ok(());
    }

    println!("Environment variables from montrs.toml:");
    let mut keys: Vec<&String> = env.vars.keys().collect();
    keys.sort();
    for key in keys {
        let export = env.exports.get(key).copied().unwrap_or(true);
        let export_marker = if export { "export" } else { "no-export" };
        println!("  {}={} ({})", key, env.vars[key], export_marker);
    }

    if !env.path.prepend.is_empty() || !env.path.append.is_empty() {
        println!("\nPATH modifications:");
        for p in &env.path.prepend {
            println!("  prepend: {p}");
        }
        for p in &env.path.append {
            println!("  append: {p}");
        }
    }

    Ok(())
}

pub async fn set(key_value: &str) -> anyhow::Result<()> {
    let (key, value) = key_value.split_once('=').ok_or_else(|| {
        anyhow::anyhow!("Expected KEY=value format, got: {key_value}")
    })?;

    let path = std::path::Path::new("montrs.toml");
    if !path.exists() {
        anyhow::bail!("montrs.toml not found in current directory");
    }

    let content = std::fs::read_to_string(path)?;
    let mut doc: toml::Value = content.parse()?;

    // Ensure [env] table exists
    let env_table = doc
        .as_table_mut()
        .ok_or_else(|| anyhow::anyhow!("montrs.toml is not a valid table"))?
        .entry("env".to_string())
        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));

    if let toml::Value::Table(table) = env_table {
        table.insert(key.to_string(), toml::Value::String(value.to_string()));
    }

    std::fs::write(path, doc.to_string())?;
    println!("Set {key}={value} in montrs.toml");
    Ok(())
}

pub async fn unset(key: &str) -> anyhow::Result<()> {
    let path = std::path::Path::new("montrs.toml");
    if !path.exists() {
        anyhow::bail!("montrs.toml not found in current directory");
    }

    let content = std::fs::read_to_string(path)?;
    let mut doc: toml::Value = content.parse()?;

    if let Some(table) = doc
        .as_table_mut()
        .and_then(|t| t.get_mut("env"))
        .and_then(|t| t.as_table_mut())
    {
        table.remove(key);
        println!("Removed {key} from montrs.toml [env]");
    } else {
        println!("{key} not found in montrs.toml [env]");
    }

    std::fs::write(path, doc.to_string())?;
    Ok(())
}

pub async fn exec(args: &[String]) -> anyhow::Result<()> {
    if args.is_empty() {
        anyhow::bail!("Usage: montrs env exec -- <command>");
    }

    let dirs = load_env()?;
    let env = resolve_environment(&dirs);
    apply_environment(&env);

    let program = &args[0];
    let rest = &args[1..];
    let status = Command::new(program).args(rest).status()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}
