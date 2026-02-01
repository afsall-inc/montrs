use clap::Parser;
use montrs_fmt::FormatterSettings;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files or directories to format
    #[arg(default_value = ".")]
    input: Vec<PathBuf>,

    /// Check if files are formatted without modifying them
    #[arg(long)]
    check: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load settings using the Cascade of Truth
    let settings = FormatterSettings::load();

    let mut exit_code = 0;

    for input_path in &args.input {
        if input_path.is_file() {
            if format_one_file(input_path, &settings, args.check, args.verbose)?
            {
                exit_code = 1;
            }
        } else {
            for entry in WalkDir::new(input_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().map_or(false, |ext| ext == "rs")
                })
            {
                if format_one_file(
                    entry.path(),
                    &settings,
                    args.check,
                    args.verbose,
                )? {
                    exit_code = 1;
                }
            }
        }
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
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
    let formatted = match montrs_fmt::format_source(&original, settings) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error formatting {}: {}", path.display(), e);
            return Ok(true);
        }
    };

    if original != formatted {
        if check {
            println!("File {} is not formatted", path.display());
            return Ok(true);
        } else {
            std::fs::write(path, formatted)?;
            if verbose {
                println!("Formatted {}", path.display());
            }
        }
    }

    Ok(false)
}
