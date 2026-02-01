use montrs_core::AgentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Task execution failed: {0}")]
    Task(String),
    #[error("Build failed: {0}")]
    Build(String),
}

impl AgentError for CliError {
    fn error_code(&self) -> &'static str {
        match self {
            CliError::Config(_) => "CLI_CONFIG",
            CliError::Io(_) => "CLI_IO",
            CliError::Task(_) => "CLI_TASK",
            CliError::Build(_) => "CLI_BUILD",
        }
    }

    fn explanation(&self) -> String {
        match self {
            CliError::Config(e) => format!(
                "Failed to load or parse the MontRS configuration: {}.",
                e
            ),
            CliError::Io(e) => {
                format!("An I/O error occurred during CLI operation: {}.", e)
            }
            CliError::Task(e) => {
                format!("A custom task failed to execute: {}.", e)
            }
            CliError::Build(e) => {
                format!("The project build process failed: {}.", e)
            }
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            CliError::Config(_) => vec![
                "Check montrs.toml for syntax errors.".to_string(),
                "Ensure all required configuration fields are present."
                    .to_string(),
            ],
            CliError::Io(_) => {
                vec!["Verify file permissions and paths.".to_string()]
            }
            CliError::Task(e) => vec![format!("Debug the task logic: {}.", e)],
            CliError::Build(_) => vec![
                "Check the compiler output for detailed error messages."
                    .to_string(),
                "Ensure all dependencies are correctly specified in \
                 Cargo.toml."
                    .to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "cli"
    }

    fn documentation_refs(&self) -> Vec<String> {
        match self {
            CliError::Config(_) => {
                vec!["packages/cli/docs/invariants".to_string()]
            }
            CliError::Io(_) => vec!["packages/cli/docs/invariants".to_string()],
            CliError::Task(_) => {
                vec!["packages/cli/docs/invariants".to_string()]
            }
            CliError::Build(_) => {
                vec!["packages/cli/docs/invariants".to_string()]
            }
        }
    }
}
