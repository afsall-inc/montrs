use anyhow::Result;
use montrs_agent::AgentManager;

pub async fn run(include_docs: bool, format: String) -> Result<()> {
    let output = run_to_string(include_docs, format).await?;
    println!("{}", output);
    Ok(())
}

pub async fn run_to_string(
    include_docs: bool,
    format: String,
) -> Result<String> {
    let cwd = std::env::current_dir()?;
    let manager = AgentManager::new(&cwd);

    // Agent: Ensure tools.json is updated when running spec
    if let Err(e) = manager.write_tools_spec() {
        eprintln!("Warning: Failed to update tools spec: {}", e);
    }

    let mut snapshot = manager.generate_snapshot("unknown")?;

    // Try to load basic project info from Cargo.toml
    if let Ok(cargo_toml_content) =
        std::fs::read_to_string(cwd.join("Cargo.toml"))
        && let Ok(value) = cargo_toml_content.parse::<toml::Value>()
    {
        if let Some(package) = value.get("package") {
            snapshot.project_name = package
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
        } else if let Some(workspace) = value.get("workspace")
            && let Some(package) = workspace.get("package")
        {
            snapshot.project_name = package
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("workspace")
                .to_string();
        }
    }

    if include_docs {
        // AgentManager already includes some documentation, but we can add more if needed
    }

    let output = match format.as_str() {
        "yaml" => serde_yaml::to_string(&snapshot)?,
        "txt" => format!("{:#?}", snapshot),
        _ => serde_json::to_string_pretty(&snapshot)?,
    };

    Ok(output)
}
