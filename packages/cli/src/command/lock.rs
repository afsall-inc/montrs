use montrs_lockfile::{
    LockfileTool, MontrsLock, lockfile_path_for_root, write_lockfile,
};
use std::collections::BTreeMap;

pub async fn run() -> anyhow::Result<()> {
    let root = std::env::current_dir()?;
    let lock_path = lockfile_path_for_root(&root);

    let mut lock = if lock_path.exists() {
        montrs_lockfile::read_lockfile(&lock_path)?
    } else {
        MontrsLock::new()
    };

    let montrs_toml = root.join("montrs.toml");
    if montrs_toml.exists() {
        let content = std::fs::read_to_string(&montrs_toml)?;
        let doc: toml::Value = content.parse()?;
        if let Some(tools) = doc.get("tools").and_then(|t| t.as_table()) {
            for (name, value) in tools {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(t) => t
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("latest")
                        .to_string(),
                    _ => continue,
                };
                lock.add_tool(
                    name,
                    LockfileTool {
                        version,
                        backend: None,
                        options: BTreeMap::new(),
                        platforms: BTreeMap::new(),
                    },
                );
            }
        }
    }

    lock.config_sources.push("montrs.toml".to_string());
    write_lockfile(&lock_path, &lock)?;
    println!("Lockfile written to {}", lock_path.display());
    println!("{} tools locked", lock.len());
    Ok(())
}
