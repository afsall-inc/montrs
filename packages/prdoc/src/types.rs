use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrDoc {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<u64>,
    pub doc: Vec<DocSection>,
    pub crates: Vec<CrateChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrations: Option<Migrations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_functions: Option<Vec<HostFunction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PrDocStatus {
    Draft,
    Review,
    Approved,
    Merged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Audience {
    #[serde(rename = "Framework Dev")]
    FrameworkDev,
    #[serde(rename = "App Dev")]
    AppDev,
    #[serde(rename = "Agent User")]
    AgentUser,
    #[serde(rename = "Operator")]
    Operator,
}

impl Audience {
    pub fn as_str(&self) -> &'static str {
        match self {
            Audience::FrameworkDev => "Framework Dev",
            Audience::AppDev => "App Dev",
            Audience::AgentUser => "Agent User",
            Audience::Operator => "Operator",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        let normalized = s.to_lowercase().replace("-", "").replace("_", "");
        match normalized.as_str() {
            "frameworkdev" => Audience::FrameworkDev,
            "appdev" => Audience::AppDev,
            "agentuser" => Audience::AgentUser,
            "operator" => Audience::Operator,
            _ => Audience::AppDev,
        }
    }

    pub fn all() -> &'static [Audience] {
        &[
            Audience::FrameworkDev,
            Audience::AppDev,
            Audience::AgentUser,
            Audience::Operator,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSection {
    pub audience: Audience,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CrateChange {
    pub name: String,
    pub bump: BumpLevel,
    #[serde(
        default = "default_validate",
        skip_serializing_if = "is_default_validate"
    )]
    pub validate: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

fn default_validate() -> bool {
    true
}

fn is_default_validate(v: &bool) -> bool {
    *v
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BumpLevel {
    Major,
    Minor,
    Patch,
    #[serde(rename = "none")]
    None,
}

impl BumpLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            BumpLevel::Major => "major",
            BumpLevel::Minor => "minor",
            BumpLevel::Patch => "patch",
            BumpLevel::None => "none",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "major" => BumpLevel::Major,
            "minor" => BumpLevel::Minor,
            "patch" => BumpLevel::Patch,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Migrations {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub db: Vec<DbMigration>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub runtime: Vec<RuntimeMigration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbMigration {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMigration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostFunction {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
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

    if prdoc.title.is_empty() || prdoc.title == "..." {
        issues.push("title is required and cannot be '...'".to_string());
    }

    if prdoc.doc.is_empty() {
        issues.push("at least one doc section is required".to_string());
    }

    for (i, doc) in prdoc.doc.iter().enumerate() {
        if doc.description.is_empty() || doc.description == "..." {
            issues.push(format!(
                "doc[{}].description is required and cannot be '...'",
                i
            ));
        }
    }

    if prdoc.crates.is_empty() {
        issues.push("at least one crate must be listed".to_string());
    }

    for crate_change in &prdoc.crates {
        if crate_change.name.is_empty() {
            issues.push("crate name must not be empty".to_string());
        }
    }

    issues
}

pub fn validate_prdoc_for_branch(prdoc: &PrDoc, branch: &str) -> Vec<String> {
    let mut issues = validate_prdoc(prdoc);

    if branch.starts_with("stable") || branch.starts_with("release") {
        for crate_change in &prdoc.crates {
            if crate_change.bump == BumpLevel::Major && crate_change.validate {
                issues.push(format!(
                    "crate '{}' has major bump on backport branch '{}' but \
                     validate=true. Set validate: false if intentional.",
                    crate_change.name, branch
                ));
            }
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
