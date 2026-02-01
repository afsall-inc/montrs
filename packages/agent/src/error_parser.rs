use crate::{ProjectError, AgentErrorMetadata};
use regex::Regex;
use std::sync::OnceLock;
use std::fs;
use std::path::Path;

static ERROR_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn parse_rustc_errors(output: &str) -> Vec<ProjectError> {
    let re = ERROR_REGEX.get_or_init(|| {
        Regex::new(r"error\[(?P<code>E\d+)\]: (?P<msg>.*)\n\s+--> (?P<file>.*):(?P<line>\d+):(?P<col>\d+)").unwrap()
    });

    let mut errors = Vec::new();
    for cap in re.captures_iter(output) {
        let code = cap.name("code").map(|m| m.as_str().to_string()).unwrap_or_default();
        let message = cap.name("msg").map(|m| m.as_str().to_string()).unwrap_or_default();
        let file = cap.name("file").map(|m| m.as_str().to_string()).unwrap_or_default();
        let line = cap.name("line").and_then(|m| m.as_str().parse::<usize>().ok()).unwrap_or(0);
        let column = cap.name("col").and_then(|m| m.as_str().parse::<usize>().ok()).unwrap_or(0);

        let mut docs = vec![format!("https://doc.rust-lang.org/error-index.html#{}", code)];
        
        // Map common errors to MontRS framework invariants if applicable
        match code.as_str() {
            "E0433" | "E0432" => {
                // Missing import/crate - often related to missing dependencies in Cargo.toml
                docs.push("docs/architecture/packages.md".to_string());
            }
            "E0277" | "E0599" => {
                // Trait bounds not met - often related to missing Plate or Route implementation
                docs.push("docs/core/plates.md".to_string());
                docs.push("docs/core/router.md".to_string());
            }
            _ => {}
        }

        // Read code context if the file exists
        let code_context = if line > 0 && Path::new(&file).exists() {
            match fs::read_to_string(&file) {
                Ok(content) => {
                    let lines: Vec<&str> = content.lines().collect();
                    let start = line.saturating_sub(3);
                    let end = (line + 2).min(lines.len());
                    
                    let mut context = String::new();
                    for i in start..end {
                        let line_num = i + 1;
                        let indicator = if line_num == line { "> " } else { "  " };
                        context.push_str(&format!("{}{:4} | {}\n", indicator, line_num, lines[i]));
                    }
                    context
                }
                Err(_) => "".to_string(),
            }
        } else {
            "".to_string()
        };

        errors.push(ProjectError {
            package: None, // Will be filled by AgentManager if possible
            file,
            line: line as u32,
            column: column as u32,
            message: message.clone(),
            code_context,
            level: "Error".to_string(),
            agent_metadata: Some(AgentErrorMetadata {
                error_code: code.clone(),
                explanation: format!("Rust compiler error {}: {}", code, message),
                suggested_fixes: Vec::new(),
                rustc_error: Some(output.to_string()),
                documentation_refs: docs,
            }),
        });
    }
    errors
}
