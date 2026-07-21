use crate::types::{
    BumpLevel, CrateChange, DocSection, PrDoc, load_prdoc, parse_prdoc,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    pub version: String,
    pub date: String,
    pub pr: Option<u64>,
    pub title: String,
    pub crates: Vec<CrateChange>,
    pub doc: Vec<DocSection>,
    pub category: ChangelogCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChangelogCategory {
    Added,
    Changed,
    Fixed,
    Deprecated,
    Removed,
    Security,
}

impl ChangelogCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangelogCategory::Added => "Added",
            ChangelogCategory::Changed => "Changed",
            ChangelogCategory::Fixed => "Fixed",
            ChangelogCategory::Deprecated => "Deprecated",
            ChangelogCategory::Removed => "Removed",
            ChangelogCategory::Security => "Security",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Changelog {
    pub entries: Vec<ChangelogEntry>,
}

impl Changelog {
    pub fn new() -> Self {
        Changelog {
            entries: Vec::new(),
        }
    }

    pub fn add_prdoc(&mut self, prdoc: &PrDoc) {
        let category = classify_prdoc(prdoc);
        let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        self.entries.push(ChangelogEntry {
            version: String::new(),
            date,
            pr: prdoc.pr,
            title: prdoc.title.clone(),
            crates: prdoc.crates.clone(),
            doc: prdoc.doc.clone(),
            category,
        });
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("# Changelog\n\n");
        out.push_str(
            "All notable changes to this project will be documented in this \
             file.\n\n",
        );
        out.push_str(
            "The format is based on [Keep a Changelog](https://keepachangelog.com/),\n\
             and this project adheres to [Semantic Versioning](https://semver.org/).\n\n",
        );

        let grouped = self.group_by_version();
        for (version, entries) in &grouped {
            out.push_str(&format!("## [{}]\n\n", version));
            let by_category = group_by_category(entries);
            for (cat, cat_entries) in &by_category {
                out.push_str(&format!("### {}\n\n", cat.as_str()));
                for entry in cat_entries {
                    let pr_str = entry
                        .pr
                        .map(|p| format!(" (#${})", p))
                        .unwrap_or_default();
                    let crate_strs: Vec<String> = entry
                        .crates
                        .iter()
                        .map(|c| format!("{}({})", c.name, c.bump.as_str()))
                        .collect();
                    let crates_info = if crate_strs.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", crate_strs.join(", "))
                    };
                    out.push_str(&format!(
                        "- {}{}{}\n",
                        entry.title, crates_info, pr_str,
                    ));
                }
                out.push('\n');
            }
        }

        out
    }

    fn group_by_version(&self) -> Vec<(String, Vec<&ChangelogEntry>)> {
        let mut map: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, entry) in self.entries.iter().enumerate() {
            let key = if entry.version.is_empty() {
                "Unreleased".to_string()
            } else {
                entry.version.clone()
            };
            map.entry(key).or_default().push(i);
        }

        let mut result: Vec<(String, Vec<&ChangelogEntry>)> = map
            .into_iter()
            .map(|(version, indices)| {
                let entries: Vec<&ChangelogEntry> =
                    indices.iter().map(|&i| &self.entries[i]).collect();
                (version, entries)
            })
            .collect();

        result.sort_by(|a, b| {
            if a.0 == "Unreleased" {
                std::cmp::Ordering::Less
            } else if b.0 == "Unreleased" {
                std::cmp::Ordering::Greater
            } else {
                b.0.cmp(&a.0)
            }
        });

        result
    }
}

fn group_by_category<'a>(
    entries: &'a [&ChangelogEntry],
) -> Vec<(ChangelogCategory, Vec<&'a ChangelogEntry>)> {
    let mut map: HashMap<ChangelogCategory, Vec<&'a ChangelogEntry>> =
        HashMap::new();
    for entry in entries {
        map.entry(entry.category.clone()).or_default().push(*entry);
    }

    let order = [
        ChangelogCategory::Added,
        ChangelogCategory::Changed,
        ChangelogCategory::Deprecated,
        ChangelogCategory::Removed,
        ChangelogCategory::Fixed,
        ChangelogCategory::Security,
    ];

    let mut result = Vec::new();
    for cat in &order {
        if let Some(mut list) = map.remove(cat) {
            list.sort_by(|a, b| a.title.cmp(&b.title));
            result.push((cat.clone(), list));
        }
    }
    result
}

fn classify_prdoc(prdoc: &PrDoc) -> ChangelogCategory {
    let has_major = prdoc.crates.iter().any(|c| c.bump == BumpLevel::Major);
    let has_minor = prdoc.crates.iter().any(|c| c.bump == BumpLevel::Minor);
    let has_patch = prdoc.crates.iter().any(|c| c.bump == BumpLevel::Patch);

    if has_major {
        ChangelogCategory::Removed
    } else if has_minor {
        ChangelogCategory::Added
    } else if has_patch {
        ChangelogCategory::Fixed
    } else {
        ChangelogCategory::Changed
    }
}

pub fn determine_next_version(
    current: &str,
    prdocs: &[PrDoc],
) -> HashMap<String, String> {
    let mut crate_bumps: HashMap<String, BumpLevel> = HashMap::new();

    for prdoc in prdocs {
        for change in &prdoc.crates {
            let entry = crate_bumps
                .entry(change.name.clone())
                .or_insert(BumpLevel::None);
            if change.bump.dominates(entry) {
                *entry = change.bump.clone();
            }
        }
    }

    let mut result = HashMap::new();
    for (crate_name, bump) in &crate_bumps {
        let next = bump_version(current, bump);
        result.insert(crate_name.clone(), next);
    }

    result
}

fn bump_version(current: &str, bump: &BumpLevel) -> String {
    let parts: Vec<&str> = current.split('.').collect();
    if parts.len() != 3 {
        return current.to_string();
    }

    let mut major: u32 = parts[0].parse().unwrap_or(0);
    let mut minor: u32 = parts[1].parse().unwrap_or(0);
    let mut patch: u32 = parts[2].parse().unwrap_or(0);

    match bump {
        BumpLevel::Major => {
            major += 1;
            minor = 0;
            patch = 0;
        }
        BumpLevel::Minor => {
            minor += 1;
            patch = 0;
        }
        BumpLevel::Patch => {
            patch += 1;
        }
        BumpLevel::None => {}
    }

    format!("{}.{}.{}", major, minor, patch)
}

pub fn collect_prdocs_from_git(range: &str) -> Vec<PrDoc> {
    let output = std::process::Command::new("git")
        .args(["log", "--oneline", range])
        .output()
        .ok();

    let log_str = match output {
        Some(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).to_string()
        }
        _ => return Vec::new(),
    };

    let mut prdocs = Vec::new();
    for line in log_str.lines() {
        let hash = line.split_whitespace().next().unwrap_or("");
        let prdoc_content = std::process::Command::new("git")
            .args(["show", &format!("{}:prdoc.md", hash)])
            .output()
            .ok();

        if let Some(content) = prdoc_content
            && content.status.success()
        {
            let text = String::from_utf8_lossy(&content.stdout);
            if let Ok(prdoc) = parse_prdoc(&text) {
                prdocs.push(prdoc);
            }
        }
    }

    prdocs
}

pub fn load_prdocs_from_dir(dir: &std::path::Path) -> Vec<PrDoc> {
    let mut prdocs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().and_then(|n| n.to_str()) == Some("prdoc.md")
                && let Ok(prdoc) = load_prdoc(&path)
            {
                prdocs.push(prdoc);
            }
        }
    }
    prdocs
}
