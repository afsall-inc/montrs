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
    #[serde(default)]
    pub audience: Vec<Audience>,
    #[serde(default)]
    pub crates: Vec<CrateChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrDocStatus {
    Draft,
    Review,
    Approved,
    Merged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BumpLevel {
    None,
    Patch,
    Minor,
    Major,
}

impl BumpLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            BumpLevel::None => "none",
            BumpLevel::Patch => "patch",
            BumpLevel::Minor => "minor",
            BumpLevel::Major => "major",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "patch" => BumpLevel::Patch,
            "minor" => BumpLevel::Minor,
            "major" => BumpLevel::Major,
            _ => BumpLevel::None,
        }
    }

    pub fn dominates(&self, other: &BumpLevel) -> bool {
        let ord = |l: &BumpLevel| match l {
            BumpLevel::None => 0,
            BumpLevel::Patch => 1,
            BumpLevel::Minor => 2,
            BumpLevel::Major => 3,
        };
        ord(self) >= ord(other)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    AppDev,
    FrameworkDev,
    AgentUser,
    Operator,
}

impl Audience {
    pub fn as_str(&self) -> &'static str {
        match self {
            Audience::AppDev => "app_dev",
            Audience::FrameworkDev => "framework_dev",
            Audience::AgentUser => "agent_user",
            Audience::Operator => "operator",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CrateChange {
    pub name: String,
    pub bump: BumpLevel,
    #[serde(default)]
    pub validate: bool,
}

pub fn parse_prdoc(content: &str) -> Result<PrDoc, String> {
    let frontmatter = extract_frontmatter(content)?;
    serde_yaml::from_str(&frontmatter)
        .map_err(|e| format!("Invalid prdoc frontmatter: {e}"))
}

pub fn load_prdoc(path: &Path) -> Result<PrDoc, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read prdoc: {e}"))?;
    parse_prdoc(&content)
}

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
    for pkg in &prdoc.packages {
        let has_crate_entry = prdoc.crates.iter().any(|c| c.name == *pkg);
        if !has_crate_entry {
            issues.push(format!(
                "package '{}' has no crate entry with a bump level",
                pkg
            ));
        }
    }
    for change in &prdoc.crates {
        if change.name.is_empty() {
            issues.push("crate name must not be empty".to_string());
        }
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
