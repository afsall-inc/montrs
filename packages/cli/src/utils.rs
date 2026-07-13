use crate::config::MontrsConfig;
use anyhow::{Result, anyhow};
use clap::Parser;

pub async fn run_cargo_leptos(
    cmd: &str,
    args: &[String],
    config: &MontrsConfig,
) -> Result<()> {
    // Build arguments for cargo-leptos
    let mut args_list = vec!["cargo-leptos".to_string(), cmd.to_string()];

    if config.project.release {
        args_list.push("--release".to_string());
    }
    if config.project.hot_reload {
        args_list.push("--hot-reload".to_string());
    }

    for feature in &config.project.features {
        args_list.push("--features".to_string());
        args_list.push(feature.clone());
    }

    for _ in 0..config.project.verbose {
        args_list.push("-v".to_string());
    }

    // Add trailing arguments for serve/watch
    if !args.is_empty() {
        args_list.push("--".to_string());
        for arg in args {
            args_list.push(arg.clone());
        }
    }

    let cli =
        cargo_leptos::config::Cli::try_parse_from(args_list).map_err(|e| {
            anyhow!("Failed to parse cargo-leptos arguments: {}", e)
        })?;

    match cargo_leptos::run(cli).await {
        Ok(_) => {
            // Agent: Auto-resolve errors on success
            if let Ok(cwd) = std::env::current_dir() {
                let agent_manager = montrs_agent::AgentManager::new(cwd);
                let diff = agent_manager.generate_diff();
                let _ = agent_manager.auto_resolve_active_errors(
                    "Build/Command succeeded".to_string(),
                    diff,
                );
            }
            Ok(())
        }
        Err(e) => {
            if let Ok(cwd) = std::env::current_dir() {
                let agent_manager = montrs_agent::AgentManager::new(cwd);
                let error_msg = format!("{:?}", e);

                // Try to parse structured errors
                let parsed_errors =
                    montrs_agent::error_parser::parse_rustc_errors(&error_msg);
                if parsed_errors.is_empty() {
                    let _ = agent_manager.report_error(error_msg);
                } else {
                    for err in parsed_errors {
                        let _ = agent_manager.report_project_error(err);
                    }
                }
            }
            Err(anyhow!("{:?}", e))
        }
    }
}
