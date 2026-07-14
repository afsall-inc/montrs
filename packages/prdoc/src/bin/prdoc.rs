use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "prdoc", version, about = "PR Documentation tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display a prdoc.md file as JSON
    Show {
        #[arg(default_value = "prdoc.md")]
        path: String,
    },
    /// Validate a prdoc.md file
    Validate {
        #[arg(default_value = "prdoc.md")]
        path: String,
    },
    /// Auto-generate a prdoc.md
    Generate {
        /// PR number (uses gh CLI)
        #[arg(short, long)]
        pr: Option<u64>,
        /// Local diff file
        #[arg(long)]
        from_diff: Option<String>,
        /// Git commit range
        #[arg(long)]
        from_commits: Option<String>,
        /// Enable LLM-enhanced summary
        #[arg(long)]
        llm: bool,
        /// Output path
        #[arg(short, long, default_value = "prdoc.md")]
        output: String,
        /// Overwrite existing file
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
        Commands::Validate { path } => {
            let prdoc_path = std::path::PathBuf::from(&path);
            if !prdoc_path.exists() {
                if std::env::var("CI").is_ok() {
                    return Err(format!(
                        "prdoc.md not found at {path}. PRs require prdoc.md."
                    ));
                }
                return Ok("No prdoc.md found. Validation skipped.".to_string());
            }
            let prdoc = montrs_prdoc::load_prdoc(&prdoc_path)
                .map_err(|e| e.to_string())?;
            let issues = montrs_prdoc::validate_prdoc(&prdoc);
            if issues.is_empty() {
                Ok("prdoc.md is valid.".to_string())
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
            from_diff,
            from_commits,
            llm,
            output,
            force,
        } => {
            let output_path = std::path::PathBuf::from(&output);
            if output_path.exists() && !force {
                return Err(format!(
                    "{output} exists. Use --force to overwrite."
                ));
            }

            let diff = if let Some(p) = from_diff {
                std::fs::read_to_string(&p).ok()
            } else if let Some(n) = pr {
                montrs_prdoc::get_diff_for_pr(n)
            } else if let Some(r) = from_commits {
                montrs_prdoc::get_diff_for_range(&r)
            } else {
                montrs_prdoc::get_diff_for_range("main..HEAD")
            };

            let diff_str = diff.unwrap_or_default();
            if diff_str.is_empty() {
                return Err("No diff found. Use --pr, --from-diff, or \
                            --from-commits."
                    .to_string());
            }

            let analysis = montrs_prdoc::analyze_diff(&diff_str);
            let context =
                pr.and_then(|n| montrs_prdoc::gather_pr_context_from_gh(n));

            let prdoc =
                montrs_prdoc::generate_prdoc(&analysis, context.as_ref());

            let rendered = montrs_prdoc::render_prdoc_rich(
                &prdoc,
                &analysis,
                context.as_ref(),
                Some(&diff_str),
                llm,
            );

            if let Some(parent) = output_path.parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(&output_path, &rendered)
                .map_err(|e| e.to_string())?;

            Ok(format!(
                "Generated {output} ({} package(s), {} crate(s))",
                prdoc.packages.len(),
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
