use anyhow::{Result, anyhow};
use console::style;
use std::{fs, path::Path, process::Command};

pub async fn run(name: String, template: String) -> Result<()> {
    println!(
        "{} Creating new MontRS project: {}",
        style("🚀").bold(),
        style(&name).cyan().bold()
    );

    let cwd = std::env::current_dir()?;
    let template_dir = cwd.join("templates").join(&template);
    let dest_dir = cwd.join(&name);

    if !template_dir.exists() {
        return Err(anyhow!(
            "Template '{}' not found at {}. Available templates: {}",
            template,
            template_dir.display(),
            list_templates(&cwd)?
        ));
    }

    if dest_dir.exists() {
        return Err(anyhow!(
            "Directory '{}' already exists. Remove it first or choose a \
             different name.",
            name
        ));
    }

    println!("  Copying template '{}' → '{}'", template, name);
    copy_dir_recursive(&template_dir, &dest_dir)?;

    // Substitute project name in Cargo.toml and montrs.toml
    substitute_project_name(&dest_dir, &name)?;

    // Initialize git
    println!("  Initializing git repository...");
    let _ = Command::new("git")
        .args(["init"])
        .current_dir(&dest_dir)
        .output();

    println!();
    println!(
        "{} Project {} created at {}",
        style("✨").green().bold(),
        style(&name).cyan().bold(),
        style(dest_dir.display()).underlined()
    );
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  montrs serve");

    Ok(())
}

fn list_templates(cwd: &Path) -> Result<String> {
    let dir = cwd.join("templates");
    if !dir.exists() {
        return Ok("none".to_string());
    }
    let mut names: Vec<String> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    Ok(names.join(", "))
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest = dst.join(entry.file_name());

        if path.file_name().is_some_and(|n| n == ".agent") {
            continue;
        }

        if path.is_dir() {
            copy_dir_recursive(&path, &dest)?;
        } else {
            fs::copy(&path, &dest)?;
        }
    }
    Ok(())
}

fn substitute_project_name(dir: &Path, name: &str) -> Result<()> {
    // Walk all .toml files and replace {{project-name}}
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name().to_str().is_some_and(|n| n.ends_with(".toml"))
        })
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;
        let new_content = content
            .replace("{{project-name}}", name)
            .replace("{{crate_name}}", &name.replace('-', "_"));
        if content != new_content {
            fs::write(path, new_content)?;
        }
    }
    Ok(())
}
