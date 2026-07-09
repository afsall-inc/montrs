use crate::{AgentSubcommand, RulesSubcommand};

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
        },
        AgentSubcommand::Check { path } => {
            output.push_str(&format!(
                "Checking MontRS invariants at {}...\n",
                path
            ));

            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);

            // 1. Generate snapshot (heuristically if no spec is available in this context)
            // In a real run, we'd ideally have the AppSpec, but for 'check' we can start with discovery.
            let snapshot = manager.generate_snapshot("montrs-project")?;

            // 2. Check invariants
            let violations = manager.check_invariants(&snapshot)?;

            if violations.is_empty() {
                output.push_str("✅ No invariant violations found.\n");
            } else {
                output.push_str("❌ Invariant violations found:\n");
                for violation in violations {
                    output.push_str(&format!("  - {}\n", violation));
                }
            }

            Ok(output)
        }
        AgentSubcommand::Doctor { package } => {
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

            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);

            let diagnostics = manager.run_doctor(package.as_deref())?;

            for report in diagnostics {
                output.push_str(&format!("  {}\n", report));
            }

            Ok(output)
        }
        AgentSubcommand::Diff { path } => {
            output.push_str("### Agent Diagnostic Report\n");
            output.push_str(&format!("Target: {}\n", path));

            // 1. Load error file
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
        AgentSubcommand::ListErrors { status } => {
            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);
            let tracking = manager.load_tracking()?;

            output.push_str("### Agent Error Tracking\n\n");

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

            if filtered_errors.is_empty() {
                output.push_str("No errors tracked yet.\n");
            } else {
                output.push_str(
                    "| ID | Package | File | Line | Level | Status | Message \
                     |\n",
                );
                output
                    .push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
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
}
