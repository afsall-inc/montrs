use anyhow::Result;
use colored::Colorize;
use montrs_fmt::{FormatterSettings, format_source};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub async fn run(
    settings: FormatterSettings,
    check: bool,
    path: String,
    verbose: bool,
) -> Result<()> {
    let input_path = PathBuf::from(path);

    let mut exit_code = 0;
    let mut files_checked = 0;
    let mut files_formatted = 0;

    if input_path.is_file() {
        if format_one_file(&input_path, &settings, check, verbose)? {
            exit_code = 1;
            files_formatted += 1;
        }
        files_checked += 1;
    } else {
        for entry in WalkDir::new(&input_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
        {
            if format_one_file(entry.path(), &settings, check, verbose)? {
                exit_code = 1;
                files_formatted += 1;
            }
            files_checked += 1;
        }
    }

    if check {
        if exit_code == 0 {
            println!("{}", "All files are correctly formatted.".green());
        } else {
            println!(
                "{}",
                format!("{} files need formatting.", files_formatted).red()
            );
            anyhow::bail!("Formatting check failed");
        }
    } else {
        if verbose {
            println!(
                "Checked {} files, formatted {} files.",
                files_checked, files_formatted
            );
        }
    }

    Ok(())
}

fn format_one_file(
    path: &Path,
    settings: &FormatterSettings,
    check: bool,
    verbose: bool,
) -> anyhow::Result<bool> {
    let original = std::fs::read_to_string(path)?;
    let formatted = match format_source(&original, settings) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} {}: {}", "Error".red().bold(), path.display(), e);
            return Ok(false);
        }
    };

    if original != formatted {
        if check {
            println!("{} {} is not formatted", "✘".red(), path.display());
            return Ok(true);
        } else {
            std::fs::write(path, formatted)?;
            if verbose {
                println!("{} Formatted {}", "✓".green(), path.display());
            }
        }
    }

    Ok(false)
}
