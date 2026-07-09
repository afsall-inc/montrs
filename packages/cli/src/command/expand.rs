use anyhow::{Result, anyhow};
use console::style;
use montrs_utils::to_snake_case;
use std::{
    fs,
    path::{Path, PathBuf},
};
use syn::{File, Item, parse_str};

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

    // Parse the file to identify components
    let file: File = parse_str(&content)
        .map_err(|e| anyhow!("Failed to parse sketch file: {}", e))?;

    // Determine type by looking for Plate, Route, or AppConfig implementations
    let mut expanded = false;
    for item in &file.items {
        if let Item::Impl(item_impl) = item {
            let trait_path = item_impl.trait_.as_ref().map(|t| &t.1);
            if let Some(path) = trait_path {
                let trait_name = path
                    .segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .unwrap_or_default();
                if trait_name == "Plate" {
                    expand_plate(&file, &content)?;
                    expanded = true;
                    break;
                } else if trait_name == "Route" {
                    expand_route(&file, &content)?;
                    expanded = true;
                    break;
                } else if trait_name == "AppConfig" {
                    expand_app(&content)?;
                    expanded = true;
                    break;
                }
            }
        }
    }

    if !expanded {
        return Err(anyhow!(
            "Could not determine component type in sketch. Ensure it \
             implements Plate, Route, or AppConfig."
        ));
    }

    Ok(())
}

fn expand_plate(file: &File, content: &str) -> Result<()> {
    // 1. Find the Plate struct name
    let plate_struct: &syn::ItemStruct = file
        .items
        .iter()
        .find_map(|item| {
            if let Item::Struct(s) = item {
                if s.ident.to_string().contains("Plate") {
                    return Some(s);
                }
            }
            None
        })
        .ok_or_else(|| anyhow!("Could not find a Plate struct in sketch."))?;

    let plate_name = plate_struct.ident.to_string();
    let snake_name = to_snake_case(&plate_name.replace("Plate", ""));
    let target_dir = PathBuf::from("src/plates").join(&snake_name);
    let target_path = target_dir.join("mod.rs");

    // 2. Create directory structure
    fs::create_dir_all(&target_dir)?;
    fs::create_dir_all(target_dir.join("routes"))?;

    // 3. Write the original content to mod.rs to preserve comments and formatting
    fs::write(&target_path, content)?;

    // 4. Register in src/plates/mod.rs
    register_module("src/plates/mod.rs", &snake_name)?;

    println!(
        "{} Expanded plate {} to: {}",
        style("✨").green().bold(),
        style(&plate_name).cyan(),
        style(target_path.display().to_string()).underlined()
    );

    Ok(())
}

fn expand_route(file: &File, content: &str) -> Result<()> {
    // 1. Find the Route struct name
    let route_struct = file
        .items
        .iter()
        .find_map(|item| {
            if let Item::Struct(s) = item {
                if s.ident.to_string().contains("Route") {
                    return Some(s);
                }
            }
            None
        })
        .ok_or_else(|| anyhow!("Could not find a Route struct in sketch."))?;

    let route_name = route_struct.ident.to_string();
    let snake_name = to_snake_case(&route_name.replace("Route", ""));

    // Heuristic: Try to find which plate this belongs to.
    // We look for existing plate directories.
    let plates_dir = Path::new("src/plates");
    let plate_name = if plates_dir.exists() {
        fs::read_dir(plates_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .next()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .unwrap_or_else(|| "main".to_string())
    } else {
        "main".to_string()
    };

    let target_dir =
        PathBuf::from("src/plates").join(&plate_name).join("routes");
    let target_path = target_dir.join(format!("{}.rs", snake_name));

    fs::create_dir_all(&target_dir)?;
    fs::write(&target_path, content)?;

    // Register in src/plates/<plate>/routes/mod.rs
    register_module(target_dir.join("mod.rs"), &snake_name)?;

    println!(
        "{} Expanded route {} to plate {}: {}",
        style("✨").green().bold(),
        style(&route_name).cyan(),
        style(&plate_name).yellow(),
        style(target_path.display().to_string()).underlined()
    );

    Ok(())
}

fn expand_app(content: &str) -> Result<()> {
    // AppConfig expansion typically updates src/app_spec.rs
    let target_path = Path::new("src/app_spec.rs");

    if target_path.exists() {
        let mut existing_content = fs::read_to_string(target_path)?;
        existing_content.push_str("\n\n");
        existing_content.push_str(content);
        fs::write(target_path, existing_content)?;
    } else {
        fs::write(target_path, content)?;
    }

    println!(
        "{} Updated application spec at: {}",
        style("✨").green().bold(),
        style(target_path.display().to_string()).underlined()
    );

    Ok(())
}

fn register_module<P: AsRef<Path>>(
    mod_rs_path: P,
    mod_name: &str,
) -> Result<()> {
    let path = mod_rs_path.as_ref();
    let line = format!("pub mod {};\n", mod_name);

    if path.exists() {
        let content = fs::read_to_string(path)?;
        if !content.contains(&line) {
            let mut new_content = content;
            if !new_content.ends_with('\n') {
                new_content.push('\n');
            }
            new_content.push_str(&line);
            fs::write(path, new_content)?;
        }
    } else {
        fs::write(path, line)?;
    }
    Ok(())
}
