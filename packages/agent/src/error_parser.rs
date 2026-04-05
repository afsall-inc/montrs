use crate::{AgentErrorMetadata, ProjectError};
use regex::Regex;
use std::sync::OnceLock;

static ERROR_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn parse_rustc_errors(output: &str) -> Vec<ProjectError> {
    let re = ERROR_REGEX.get_or_init(|| {
        Regex::new(
            r"error\[(?P<code>E\d+)\]: (?P<msg>.*)\n\s+--> (?P<file>.*):(?P<line>\d+):(?P<col>\d+)",
        )
        .unwrap()
    });

    let mut errors = Vec::new();
    for cap in re.captures_iter(output) {
        let code = cap
            .name("code")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let message = cap
            .name("msg")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let file = cap
            .name("file")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let line = cap
            .name("line")
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
        let column = cap
            .name("col")
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);

        errors.push(ProjectError {
            package: None, // Will be filled by AgentManager if possible
            file,
            line,
            column,
            message: message.clone(),
            code_context: "".to_string(), // In a real implementation, we'd read the file here
            level: "Error".to_string(),
            agent_metadata: Some(AgentErrorMetadata {
                error_code: code.clone(),
                explanation: format!("Rust compiler error {}: {}", code, message),
                suggested_fixes: Vec::new(),
                rustc_error: Some(output.to_string()),
                documentation_refs: Vec::new(),
            }),
        });
    }
    errors
}
