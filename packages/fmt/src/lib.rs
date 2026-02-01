// @agent-tool: name="code_format" desc="Formats Rust and view! code according to MontRS standards."

use montrs_core::AgentError;
use std::path::Path;
use thiserror::Error;

pub mod comments;
pub mod config;
pub mod macro_fmt;

pub use config::FormatterSettings;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Parse error: {0}")]
    Parse(#[from] syn::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Macro format error: {0}")]
    Macro(String),
}

impl AgentError for FormatError {
    fn error_code(&self) -> &'static str {
        match self {
            FormatError::Parse(_) => "FMT_PARSE",
            FormatError::Io(_) => "FMT_IO",
            FormatError::Macro(_) => "FMT_MACRO",
        }
    }

    fn explanation(&self) -> String {
        match self {
            FormatError::Parse(e) => {
                format!("Failed to parse Rust source code: {}.", e)
            }
            FormatError::Io(e) => {
                format!("An I/O error occurred during formatting: {}.", e)
            }
            FormatError::Macro(e) => format!(
                "An error occurred while formatting a MontRS macro: {}.",
                e
            ),
        }
    }

    fn suggested_fixes(&self) -> Vec<String> {
        match self {
            FormatError::Parse(_) => vec![
                "Check the Rust source code for syntax errors.".to_string(),
                "Ensure that all macros are properly closed.".to_string(),
            ],
            FormatError::Io(_) => vec![
                "Verify that the file path is correct and accessible."
                    .to_string(),
                "Check for file system permissions.".to_string(),
            ],
            FormatError::Macro(_) => vec![
                "Check the syntax within the view! or other MontRS macros."
                    .to_string(),
                "Ensure that the macro contents follow the expected MontRS \
                 specification."
                    .to_string(),
            ],
        }
    }

    fn subsystem(&self) -> &'static str {
        "fmt"
    }
}

/// Formats a single Rust file.
pub fn format_file(
    path: impl AsRef<Path>,
    settings: &FormatterSettings,
) -> Result<String, FormatError> {
    let source = std::fs::read_to_string(path)?;
    format_source(&source, settings)
}

/// Formats a Rust source string.
pub fn format_source(
    source: &str,
    settings: &FormatterSettings,
) -> Result<String, FormatError> {
    // 0. Normalize Scaffolded headers
    let source = normalize_scaffold_headers(source);

    // 1. Extract comments
    let (source_rope, comments) = comments::extract_comments(&source);

    // 2. Parse the file into a syn::File
    let file = syn::parse_file(&source)?;

    // 3. Collect and format view! macros
    let mut edits = Vec::new();
    macro_fmt::collect_and_format_macros(
        &file,
        &source_rope,
        settings,
        &mut edits,
    )?;

    // 4. Format the file using prettyplease
    // Note: prettyplease will format the macros too, but we will overwrite them
    let formatted = prettyplease::unparse(&file);

    // 5. Re-apply macro edits to the formatted output
    // This is tricky because prettyplease changed the spans.
    // Instead, we should have formatted the macros and then used them.
    // For now, let's stick to the pipeline:
    // If we have macros, we need to find them in the formatted output.

    // Simplified: re-parse the formatted output and find macros again to apply edits
    let formatted_ast = syn::parse_file(&formatted)?;
    let mut formatted_rope = crop::Rope::from(formatted);

    let mut formatted_edits = Vec::new();
    macro_fmt::collect_and_format_macros(
        &formatted_ast,
        &formatted_rope,
        settings,
        &mut formatted_edits,
    )?;

    macro_fmt::apply_edits(&mut formatted_rope, formatted_edits);

    // 6. Re-insert comments
    let final_source =
        comments::reinsert_comments(&formatted_rope.to_string(), comments);

    Ok(final_source)
}

fn normalize_scaffold_headers(source: &str) -> String {
    let mut lines: Vec<String> =
        source.lines().map(|s| s.to_string()).collect();
    if lines.is_empty() {
        return source.to_string();
    }

    // Standardize "MontRS Plate Sketch" and "MontRS Route Sketch" headers
    for line in lines.iter_mut().take(3) {
        if line.contains("MontRS")
            && (line.contains("Sketch") || line.contains("Blueprint"))
        {
            if !line.starts_with("//!") {
                *line = format!(
                    "//! {}",
                    line.trim_start_matches(|c: char| !c.is_alphabetic())
                );
            }
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FormatterSettings;

    #[test]
    fn test_format_basic_rust() {
        let source = "fn main() { let x = 1; }";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("fn main() {"));
        assert!(formatted.contains("let x = 1;"));
    }

    #[test]
    fn test_format_with_comments() {
        let source = "fn main() {\n    // A line comment\n    let x = 1; /* a \
                      block comment */\n}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("// A line comment"));
        assert!(formatted.contains("/* a block comment */"));
    }

    #[test]
    fn test_format_view_macro() {
        let source = "fn main() {\n    view! { <div \
                      class=\"test\"><span>\"Hello\"</span></div> };\n}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("<div class=\"test\">"));
        assert!(formatted.contains("<span>"));
        assert!(formatted.contains("\"Hello\""));
    }

    #[test]
    fn test_normalize_headers() {
        let source = "// MontRS Plate Sketch: Test\nfn main() {}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("//! // MontRS Plate Sketch: Test"));
    }

    #[test]
    fn test_agent_tool_preservation() {
        let source = "// @agent-tool: name=\"test\"\nfn main() {}";
        let settings = FormatterSettings::default();
        let formatted = format_source(source, &settings).unwrap();
        assert!(formatted.contains("@agent-tool"));
        assert!(
            formatted.find("@agent-tool").unwrap()
                < formatted.find("fn main").unwrap()
        );
    }
}
