use crate::{
    AgentSubcommand, ChangelogSubcommand, PrdocSubcommand, RulesSubcommand,
};

pub async fn run(subcommand: AgentSubcommand) -> anyhow::Result<String> {
    let mut output = String::new();
    match subcommand {
        AgentSubcommand::Rules { subcommand } => match subcommand {
            RulesSubcommand::Setup => {
                let cwd = std::env::current_dir()?;
                let manager = montrs_agent::AgentManager::new(cwd);
                let result = manager.setup_ide_rules()?;
                Ok(result)
            }
            RulesSubcommand::Export { format } => {
                let cwd = std::env::current_dir()?;
                let manager = montrs_agent::AgentManager::new(cwd);
                match format.as_str() {
                    "trae" => {
                        manager.export_rules_for_trae()?;
                        Ok("Exported rules to .trae/rules/".to_string())
                    }
                    "cursor" => {
                        manager.export_rules_for_cursor()?;
                        Ok("Exported rules to .cursorrules".to_string())
                    }
                    _ => Err(anyhow::anyhow!(
                        "Unknown format '{}'. Supported: trae, cursor",
                        format
                    )),
                }
            }
            RulesSubcommand::List => {
                let cwd = std::env::current_dir()?;
                let manager = montrs_agent::AgentManager::new(cwd);
                let rules_dir = manager.agent_dir().join("rules");
                if !rules_dir.exists() {
                    return Ok("No rules found. Run `montrs agent rules \
                               setup` first."
                        .to_string());
                }
                output.push_str("Available rule sets in .agent/rules/:\n");
                for entry in std::fs::read_dir(&rules_dir)? {
                    let entry = entry?;
                    if entry.path().extension().and_then(|s| s.to_str())
                        == Some("md")
                    {
                        let name = entry
                            .path()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string();
                        output.push_str(&format!("  - {}\n", name));
                    }
                }
                Ok(output)
            }
        },
        AgentSubcommand::Check { path, json } => {
            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);
            let snapshot = manager.generate_snapshot("montrs-project")?;
            let violations = manager.check_invariants(&snapshot)?;

            if json {
                let json_violations: Vec<serde_json::Value> = violations
                    .iter()
                    .map(|v| serde_json::json!({ "violation": v }))
                    .collect();
                Ok(serde_json::to_string_pretty(&json_violations)?)
            } else {
                output.push_str(&format!(
                    "Checking MontRS invariants at {}...\n",
                    path
                ));
                if violations.is_empty() {
                    output.push_str("No invariant violations found.\n");
                } else {
                    output.push_str("Invariant violations found:\n");
                    for violation in violations {
                        output.push_str(&format!("  - {}\n", violation));
                    }
                }
                Ok(output)
            }
        }
        AgentSubcommand::Doctor { package, json } => {
            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);
            let diagnostics = manager.run_doctor(package.as_deref())?;

            if json {
                let json_diag: Vec<serde_json::Value> = diagnostics
                    .iter()
                    .map(|d| serde_json::json!({ "report": d }))
                    .collect();
                Ok(serde_json::to_string_pretty(&json_diag)?)
            } else {
                if let Some(ref pkg) = package {
                    output.push_str(&format!(
                        "Running agent doctor for package {}...\n",
                        pkg
                    ));
                } else {
                    output.push_str(
                        "Running agent doctor for the entire project...\n",
                    );
                }
                for report in diagnostics {
                    output.push_str(&format!("  {}\n", report));
                }
                Ok(output)
            }
        }
        AgentSubcommand::Diff { path } => {
            output.push_str("### Agent Diagnostic Report\n");
            output.push_str(&format!("Target: {}\n", path));

            let error_content = std::fs::read_to_string(&path)?;
            output.push_str(&format!(
                "\n#### Error Context\n```\n{}\n```\n",
                error_content
            ));

            output.push_str("\n#### LLM Workflow Instructions\n");
            output.push_str(
                "1. **Analyze**: Examine the error above and identify the \
                 root cause in the source code.\n",
            );
            output.push_str(
                "2. **Locate**: Find the exact file and line range where the \
                 fix should be applied.\n",
            );
            output.push_str(
                "3. **Draft**: Create a minimal, atomic diff that fixes the \
                 error while maintaining MontRS invariants.\n",
            );
            output.push_str(
                "4. **Verify**: Ensure the fix doesn't introduce new \
                 structural issues (use `agent check`).\n",
            );

            Ok(output)
        }
        AgentSubcommand::ListErrors { status, json } => {
            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);
            let tracking = manager.load_tracking()?;

            let filtered_errors: Vec<_> = tracking
                .errors
                .into_iter()
                .filter(|e| {
                    if let Some(s) = &status {
                        e.status.to_lowercase() == s.to_lowercase()
                    } else {
                        true
                    }
                })
                .collect();

            if json {
                Ok(serde_json::to_string_pretty(&filtered_errors)?)
            } else {
                output.push_str("### Agent Error Tracking\n\n");
                if filtered_errors.is_empty() {
                    output.push_str("No errors tracked yet.\n");
                } else {
                    output.push_str(
                        "| ID | Package | File | Line | Level | Status | \
                         Message |\n",
                    );
                    output.push_str(
                        "| --- | --- | --- | --- | --- | --- | --- |\n",
                    );
                    for error in filtered_errors {
                        output.push_str(&format!(
                            "| {} | {} | {} | {} | {} | {} | {} |\n",
                            error.id,
                            error.package.unwrap_or_else(|| "-".to_string()),
                            error.file,
                            error.line,
                            error.level,
                            error.status,
                            error.message
                        ));
                    }
                }
                Ok(output)
            }
        }
        AgentSubcommand::Resolve { id, message } => {
            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);
            let fix_msg =
                message.unwrap_or_else(|| "Resolved via CLI".to_string());
            let diff = manager.generate_diff();
            manager.resolve_error(&id, fix_msg, diff)?;
            Ok(format!("Error {} resolved.", id))
        }
        AgentSubcommand::Snapshot { format } => {
            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);
            let app_name = std::fs::read_to_string("montrs.toml")
                .ok()
                .and_then(|c| toml::from_str::<toml::Value>(&c).ok())
                .and_then(|v| {
                    v.get("project")
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "app".to_string());
            let snapshot = manager.generate_snapshot(&app_name)?;
            manager.write_snapshot(&snapshot, &format)?;
            Ok(format!("Snapshot written to .agent/agent.{}", format))
        }
        AgentSubcommand::Skills { name } => {
            let cwd = std::env::current_dir()?;
            let discovered = montrs_agent::skills::discover_skills(&cwd);
            if let Some(skill_name) = name {
                let skill =
                    discovered.iter().find(|s| s.skill.name == skill_name);
                if let Some(s) = skill {
                    output.push_str(&format!(
                        "### Skill: {}\n\n{}\n\n**Workflow:**\n",
                        s.skill.name, s.skill.description
                    ));
                    for (i, step) in s.workflow.steps.iter().enumerate() {
                        output.push_str(&format!("{}. {}\n", i + 1, step));
                    }
                    if !s.context.prompts.is_empty() {
                        output.push_str("\n**Context Prompts:**\n");
                        for prompt in &s.context.prompts {
                            output.push_str(&format!("  - {}\n", prompt));
                        }
                    }
                    if !s.context.invariants.is_empty() {
                        output.push_str("\n**Invariants:**\n");
                        for inv in &s.context.invariants {
                            output.push_str(&format!("  - {}\n", inv));
                        }
                    }
                } else {
                    output.push_str(&format!(
                        "Skill '{}' not found.\n",
                        skill_name
                    ));
                }
            } else {
                output.push_str("### Available Skills\n\n");
                if discovered.is_empty() {
                    output.push_str(
                        "No skills found. Add skill.toml files to skills/ \
                         directory.\n",
                    );
                } else {
                    for s in &discovered {
                        output.push_str(&format!(
                            "- **{}** (v{}): {}\n",
                            s.skill.name, s.skill.version, s.skill.description
                        ));
                    }
                }
            }
            Ok(output)
        }
        AgentSubcommand::Prdoc { subcommand } => match subcommand {
            PrdocSubcommand::Show { path } => {
                let prdoc_path = std::path::PathBuf::from(&path);
                if !prdoc_path.exists() {
                    return Err(anyhow::anyhow!(
                        "prdoc.md not found at {}. Create one with `montrs \
                         agent prdoc generate`.",
                        path
                    ));
                }
                let prdoc = montrs_agent::prdoc::load_prdoc(&prdoc_path)
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
                Ok(serde_json::to_string_pretty(&prdoc)?)
            }
            PrdocSubcommand::Validate { path } => {
                let prdoc_path = std::path::PathBuf::from(&path);
                if !prdoc_path.exists() {
                    if std::env::var("CI").is_ok() {
                        return Err(anyhow::anyhow!(
                            "prdoc.md not found at {}. Pull requests require \
                             a prdoc.md file.",
                            path
                        ));
                    }
                    return Ok("No prdoc.md found. Validation skipped (not \
                               required outside of pull requests)."
                        .to_string());
                }
                let prdoc = montrs_agent::prdoc::load_prdoc(&prdoc_path)
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
                let issues = montrs_agent::prdoc::validate_prdoc(&prdoc);
                if issues.is_empty() {
                    Ok("prdoc.md is valid.".to_string())
                } else {
                    let mut out = "prdoc.md validation issues:\n".to_string();
                    for issue in issues {
                        out.push_str(&format!("  - {}\n", issue));
                    }
                    Err(anyhow::anyhow!("{}", out))
                }
            }
            PrdocSubcommand::Generate {
                pr,
                from_diff,
                from_commits,
                embed: _,
                llm,
                output,
                force,
            } => {
                let output_path = std::path::PathBuf::from(&output);
                if output_path.exists() && !force {
                    return Err(anyhow::anyhow!(
                        "{} already exists. Use --force to overwrite.",
                        output
                    ));
                }

                let diff = if let Some(diff_path) = from_diff {
                    std::fs::read_to_string(&diff_path).ok()
                } else if let Some(pr_num) = pr {
                    montrs_agent::prdoc_analyzer::get_diff_for_pr(pr_num)
                } else if let Some(ref range) = from_commits {
                    montrs_agent::prdoc_analyzer::get_diff_for_range(range)
                } else {
                    montrs_agent::prdoc_analyzer::get_diff_for_range(
                        "main..HEAD",
                    )
                };

                let diff_str = diff.unwrap_or_default();
                if diff_str.is_empty() {
                    return Err(anyhow::anyhow!(
                        "No diff found. Use --pr, --from-diff, or \
                         --from-commits to specify a source."
                    ));
                }

                let analysis =
                    montrs_agent::prdoc_analyzer::analyze_diff(&diff_str);

                let context = if let Some(pr_num) = pr {
                    montrs_agent::prdoc_analyzer::gather_pr_context_from_gh(
                        pr_num,
                    )
                } else {
                    None
                };

                let prdoc = montrs_agent::prdoc_generator::generate_prdoc(
                    &analysis,
                    context.as_ref(),
                );

                let rendered = montrs_agent::prdoc_generator::render_prdoc_rich(
                    &prdoc,
                    &analysis,
                    context.as_ref(),
                    Some(&diff_str),
                    llm,
                );

                if let Some(parent) = output_path.parent()
                    && !parent.as_os_str().is_empty()
                {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&output_path, &rendered)?;

                Ok(format!(
                    "Generated prdoc.md at {} ({} package(s), {} crate(s))",
                    output,
                    prdoc.packages.len(),
                    prdoc.crates.len(),
                ))
            }
        },
        AgentSubcommand::Changelog { subcommand } => match subcommand {
            ChangelogSubcommand::Generate { from, to, output } => {
                let range = build_git_range(from, to);
                let prdocs =
                    montrs_agent::changelog::collect_prdocs_from_git(&range);
                if prdocs.is_empty() {
                    return Ok("No prdocs found in the specified range. \
                               Ensure merged PRs have prdoc.md files \
                               committed."
                        .to_string());
                }
                let mut changelog = montrs_agent::changelog::Changelog::new();
                for prdoc in &prdocs {
                    changelog.add_prdoc(prdoc);
                }
                let rendered = changelog.render();
                std::fs::write(&output, &rendered)?;
                Ok(format!(
                    "Generated {} with {} entr(ies) from range '{}'",
                    output,
                    prdocs.len(),
                    range,
                ))
            }
            ChangelogSubcommand::Bump {
                current,
                from,
                dry_run,
            } => {
                let current_version =
                    current.unwrap_or_else(read_workspace_version);
                let range = from
                    .unwrap_or_else(|| format!("v{}..HEAD", current_version));
                let prdocs =
                    montrs_agent::changelog::collect_prdocs_from_git(&range);
                let bumps = montrs_agent::changelog::determine_next_version(
                    &current_version,
                    &prdocs,
                );
                if bumps.is_empty() {
                    return Ok("No version bumps needed. No prdocs with bump \
                               levels found."
                        .to_string());
                }
                let mut out =
                    format!("Version bumps from {}:\n", current_version);
                for (crate_name, next_version) in &bumps {
                    out.push_str(&format!(
                        "  {} -> {}{}\n",
                        crate_name,
                        next_version,
                        if dry_run { " (dry-run)" } else { "" }
                    ));
                }
                if !dry_run {
                    for (crate_name, next_version) in &bumps {
                        update_crate_version(crate_name, next_version)?;
                    }
                    out.push_str("Cargo.toml files updated.\n");
                }
                Ok(out)
            }
            ChangelogSubcommand::Verify { from } => {
                let current_version = read_workspace_version();
                let range = from
                    .unwrap_or_else(|| format!("v{}..HEAD", current_version));
                let output = std::process::Command::new("git")
                    .args(["log", "--oneline", &range])
                    .output();
                let log_str = match output {
                    Ok(o) if o.status.success() => {
                        String::from_utf8_lossy(&o.stdout).to_string()
                    }
                    _ => {
                        return Ok("Could not read git log for the specified \
                                   range."
                            .to_string());
                    }
                };
                let total_commits = log_str.lines().count();
                let prdocs =
                    montrs_agent::changelog::collect_prdocs_from_git(&range);
                let missing = total_commits.saturating_sub(prdocs.len());
                if missing == 0 {
                    Ok(format!(
                        "All {} commit(s) in '{}' have prdocs.",
                        total_commits, range
                    ))
                } else {
                    Ok(format!(
                        "{} commit(s) in '{}' are missing prdocs ({} found, \
                         {} total).",
                        missing,
                        range,
                        prdocs.len(),
                        total_commits,
                    ))
                }
            }
        },
    }
}

fn build_git_range(from: Option<String>, to: Option<String>) -> String {
    match (from, to) {
        (Some(f), Some(t)) => format!("{}..{}", f, t),
        (Some(f), None) => format!("{}..HEAD", f),
        (None, Some(t)) => format!("HEAD..{}", t),
        (None, None) => "HEAD~10..HEAD".to_string(),
    }
}

fn read_workspace_version() -> String {
    std::fs::read_to_string("Cargo.toml")
        .ok()
        .and_then(|c| toml::from_str::<toml::Value>(&c).ok())
        .and_then(|v| {
            v.get("workspace")
                .and_then(|w| w.get("package"))
                .and_then(|p| p.get("version"))
                .and_then(|n| n.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "0.1.0".to_string())
}

fn update_crate_version(
    crate_name: &str,
    new_version: &str,
) -> anyhow::Result<()> {
    let cargo_toml_path = format!("packages/{}/Cargo.toml", crate_name);
    if !std::path::Path::new(&cargo_toml_path).exists() {
        return Ok(());
    }
    let content = std::fs::read_to_string(&cargo_toml_path)?;
    let updated = content.replace(
        "version.workspace = true",
        &format!("version = \"{}\"", new_version),
    );
    std::fs::write(&cargo_toml_path, updated)?;
    Ok(())
}
