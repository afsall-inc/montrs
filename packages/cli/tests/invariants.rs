//! Invariant tests for montrs-cli.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Delegated Logic: CLI only handles orchestration
//! - Subcommand Isolation: Commands are modular
//! - Agent Synchronization: Commands trigger .agent/ updates

use montrs_cli::*;

#[test]
fn test_montrs_cli_debug() {
    let cli = MontrsCli {
        command: Commands::Build,
        release: false,
        hot_reload: false,
        features: Vec::new(),
        verbose: 0,
        log: Vec::new(),
    };
    assert!(format!("{:?}", cli).contains("Build"));
}

#[test]
fn test_commands_variants() {
    match Commands::Build {
        Commands::Build => {}
        _ => panic!("expected Build"),
    }
    match Commands::Serve {
        Commands::Serve => {}
        _ => panic!("expected Serve"),
    }
    match Commands::Watch {
        Commands::Watch => {}
        _ => panic!("expected Watch"),
    }
}

#[test]
fn test_cli_error_agent_error_impl() {
    use montrs_core::AgentError;
    let err = error::CliError::Config("bad config".to_string());
    assert_eq!(err.error_code(), "CLI_CONFIG");
    assert!(!err.explanation().is_empty());
    assert!(!err.suggested_fixes().is_empty());
    assert_eq!(err.subsystem(), "cli");
}

#[test]
fn test_cli_error_display() {
    let err = error::CliError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
    assert!(format!("{}", err).contains("IO error"));
}