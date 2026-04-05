use anyhow::{Result, anyhow};
use console::style;
use montrs_utils::to_snake_case;
use std::fs;
use std::path::Path;

pub async fn run(path: String) -> Result<()> {
    let sketch_path = Path::new(&path);
    if !sketch_path.exists() {
        return Err(anyhow!("Sketch file not found: {:?}", path));
    }

    println!(
        "{} Expanding sketch: {}",
        style("📦").bold(),
        style(&path).cyan().bold()
    );

    let content = fs::read_to_string(sketch_path)?;

    // Simple heuristic to determine what we're expanding
    if content.contains("impl<C: AppConfig> Plate<C>") {
        expand_plate(&content, &path)?;
    } else if content.contains("impl<C: AppConfig> Route<C>") {
        expand_route(&content, &path)?;
    } else if content.contains("impl AppConfig for") {
        expand_app(&content, &path)?;
    } else {
        return Err(anyhow!("Could not determine component type in sketch."));
    }

    Ok(())
}

fn expand_plate(content: &str, _path: &str) -> Result<()> {
    // In a real implementation, we'd use syn to parse the file.
    // For this prototype, we'll use regex/string manipulation or just inform the user.
    println!("Expanding Plate component...");

    // 1. Create src/plates directory if it doesn't exist
    fs::create_dir_all("src/plates")?;

    // 2. Extract plate name (naive)
    let plate_name = content
        .lines()
        .find(|l| l.contains("struct") && l.contains("Plate"))
        .and_then(|l| l.split_whitespace().nth(2))
        .map(|n| n.replace("Plate;", "").replace("Plate", ""))
        .unwrap_or_else(|| "Unknown".to_string());

    let snake_name = to_snake_case(&plate_name);
    let target_path = format!("src/plates/{}.rs", snake_name);

    // 3. Write the file (stripping sketch comments)
    let clean_content = content
        .lines()
        .filter(|l| !l.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(&target_path, clean_content)?;

    println!(
        "{} Expanded plate to: {}",
        style("✨").green().bold(),
        style(&target_path).underlined()
    );

    Ok(())
}

fn expand_route(_content: &str, _path: &str) -> Result<()> {
    println!("Expanding Route component...");
    // Similar logic to plate, but targets src/plates/<plate>/routes/
    println!("Note: Automatic route expansion requires identifying the parent plate.");
    Ok(())
}

fn expand_app(_content: &str, _path: &str) -> Result<()> {
    println!("Expanding App component...");
    // Targets src/main.rs and src/app_spec.rs
    Ok(())
}
