use clap::{Parser, Subcommand};
use montrs_prdoc::{
    generator::{GenerateOptions, default_output_path},
    types::{Audience, BumpLevel},
};

#[derive(Parser, Debug)]
#[command(name = "prdoc", version, about = "PR Documentation tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display a prdoc file as JSON
    Show {
        #[arg(default_value = "prdoc/pr_1.prdoc")]
        path: String,
    },
    /// Validate a prdoc file
    Validate {
        #[arg(default_value = "prdoc/pr_1.prdoc")]
        path: String,
        #[arg(long)]
        branch: Option<String>,
    },
    /// Auto-generate a prdoc skeleton
    Generate {
        #[arg(short, long)]
        pr: Option<u64>,
        #[arg(short, long, default_value = "minor")]
        bump: String,
        #[arg(short, long, default_value = "app_dev")]
        audience: String,
        #[arg(long)]
        force: bool,
    },
    /// Changelog operations
    Changelog {
        #[command(subcommand)]
        cmd: ChangelogCmd,
    },
}

#[derive(Subcommand, Debug)]
enum ChangelogCmd {
    /// Generate CHANGELOG.md from merged prdocs
    Generate {
        #[arg(short, long)]
        from: Option<String>,
        #[arg(short, long)]
        to: Option<String>,
        #[arg(short, long, default_value = "CHANGELOG.md")]
        output: String,
    },
    /// Compute version bumps
    Bump {
        #[arg(short, long)]
        current: Option<String>,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
    /// Verify PR docs exist
    Verify {
        #[arg(long)]
        from: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = run_command(cli);
    match result {
        Ok(msg) => println!("{msg}"),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_command(cli: Cli) -> Result<String, String> {
    match cli.command {
        Commands::Show { path } => {
            let prdoc_path = std::path::PathBuf::from(&path);
            if !prdoc_path.exists() {
                return Err(format!("File not found: {path}"));
            }
            let prdoc = montrs_prdoc::load_prdoc(&prdoc_path)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&prdoc).map_err(|e| e.to_string())
        }
        Commands::Validate { path, branch } => {
            let prdoc_path = std::path::PathBuf::from(&path);
            if !prdoc_path.exists() {
                if std::env::var("CI").is_ok() {
                    return Err(format!(
                        "prdoc not found at {path}. PRs require prdoc."
                    ));
                }
                return Ok("No prdoc found. Validation skipped.".to_string());
            }
            let prdoc = montrs_prdoc::load_prdoc(&prdoc_path)
                .map_err(|e| e.to_string())?;
            let issues = if let Some(ref branch_name) = branch {
                montrs_prdoc::validate_prdoc_for_branch(&prdoc, branch_name)
            } else {
                montrs_prdoc::validate_prdoc(&prdoc)
            };
            if issues.is_empty() {
                Ok("prdoc is valid.".to_string())
            } else {
                let mut out = "Issues:\n".to_string();
                for issue in issues {
                    out.push_str(&format!("  - {issue}\n"));
                }
                Err(out)
            }
        }
        Commands::Generate {
            pr,
            bump,
            audience,
            force,
        } => {
            let pr_number = pr.ok_or_else(|| "--pr is required".to_string())?;
            let bump_level = BumpLevel::from_str_lossy(&bump);
            let audience_val = Audience::from_str_lossy(&audience);

            let opts = GenerateOptions {
                pr_number,
                bump: bump_level,
                audience: audience_val,
                force,
            };

            let prdoc = montrs_prdoc::generator::generate_prdoc(&opts)
                .map_err(|e| e.to_string())?;

            let output_path = default_output_path(pr_number);
            let path = std::path::PathBuf::from(&output_path);

            if path.exists() && !force {
                return Err(format!(
                    "{output_path} exists. Use --force to overwrite."
                ));
            }

            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            let rendered = montrs_prdoc::generator::render_prdoc(&prdoc);
            std::fs::write(&path, &rendered).map_err(|e| e.to_string())?;

            Ok(format!(
                "Generated {output_path} ({} crate(s)). Edit the `...` \
                 placeholders.",
                prdoc.crates.len(),
            ))
        }
        Commands::Changelog { cmd } => match cmd {
            ChangelogCmd::Generate { from, to, output } => {
                let range = match (from, to) {
                    (Some(f), Some(t)) => format!("{f}..{t}"),
                    (Some(f), None) => format!("{f}..HEAD"),
                    (None, Some(t)) => format!("HEAD..{t}"),
                    (None, None) => "HEAD~10..HEAD".to_string(),
                };
                let prdocs = montrs_prdoc::collect_prdocs_from_git(&range);
                let mut changelog = montrs_prdoc::Changelog::new();
                for p in &prdocs {
                    changelog.add_prdoc(p);
                }
                let rendered = changelog.render();
                std::fs::write(&output, &rendered)
                    .map_err(|e| e.to_string())?;
                Ok(format!(
                    "Generated {output} with {} entr(ies) from '{range}'",
                    prdocs.len(),
                ))
            }
            ChangelogCmd::Bump {
                current,
                from,
                dry_run,
            } => {
                let version = current.unwrap_or_else(|| "0.1.0".to_string());
                let range = from.unwrap_or_else(|| format!("v{version}..HEAD"));
                let prdocs = montrs_prdoc::collect_prdocs_from_git(&range);
                let bumps =
                    montrs_prdoc::determine_next_version(&version, &prdocs);
                if bumps.is_empty() {
                    Ok("No version bumps needed.".to_string())
                } else {
                    let mut out = format!("Bumps from {version}:\n");
                    for (c, v) in &bumps {
                        out.push_str(&format!(
                            "  {c} -> {v}{}\n",
                            if dry_run { " (dry-run)" } else { "" }
                        ));
                    }
                    Ok(out)
                }
            }
            ChangelogCmd::Verify { from } => {
                let version = "0.1.0";
                let range = from.unwrap_or_else(|| format!("v{version}..HEAD"));
                let prdocs = montrs_prdoc::collect_prdocs_from_git(&range);
                let output = std::process::Command::new("git")
                    .args(["log", "--oneline", &range])
                    .output();
                let log_str = output
                    .ok()
                    .filter(|o| o.status.success())
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                    .unwrap_or_default();
                let total = log_str.lines().count();
                let missing = total.saturating_sub(prdocs.len());
                if missing == 0 {
                    Ok(format!("All {total} commit(s) have prdocs."))
                } else {
                    Ok(format!(
                        "{missing} commit(s) missing prdocs ({}/{total} \
                         found).",
                        prdocs.len(),
                    ))
                }
            }
        },
    }
}
