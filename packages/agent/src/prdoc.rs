use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrDoc {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<u64>,
    pub author: String,
    pub status: PrDocStatus,
    pub packages: Vec<String>,
    pub breaking: bool,
    #[serde(default)]
    pub needs_review: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrDocStatus {
    Draft,
    Review,
    Approved,
    Merged,
}

/// Parse a prdoc.md file, extracting the YAML frontmatter.
pub fn parse_prdoc(content: &str) -> Result<PrDoc, String> {
    let frontmatter = extract_frontmatter(content)?;
    toml::from_str(&frontmatter)
        .map_err(|e| format!("Invalid prdoc frontmatter: {e}"))
}

/// Load and parse a prdoc.md file from disk.
pub fn load_prdoc(path: &Path) -> Result<PrDoc, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read prdoc: {e}"))?;
    parse_prdoc(&content)
}

/// Validate a prdoc.md file, returning a list of issues.
pub fn validate_prdoc(prdoc: &PrDoc) -> Vec<String> {
    let mut issues = Vec::new();
    if prdoc.title.is_empty() {
        issues.push("title is required".to_string());
    }
    if prdoc.author.is_empty() {
        issues.push("author is required".to_string());
    }
    if prdoc.packages.is_empty() {
        issues.push("at least one package must be listed".to_string());
    }
    issues
}

fn extract_frontmatter(content: &str) -> Result<String, String> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Err(
            "prdoc.md must start with YAML frontmatter (---)".to_string()
        );
    }
    let end = trimmed[3..]
        .find("\n---")
        .ok_or("prdoc.md frontmatter must be closed with ---")?;
    Ok(trimmed[3..end + 3].trim().to_string())
}
