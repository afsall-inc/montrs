use crate::types::{Audience, BumpLevel, CrateChange};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChangeCategory {
    NewFeature,
    BugFix,
    BreakingChange,
    Refactor,
    Documentation,
    Internal,
}

impl ChangeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeCategory::NewFeature => "new_feature",
            ChangeCategory::BugFix => "bug_fix",
            ChangeCategory::BreakingChange => "breaking_change",
            ChangeCategory::Refactor => "refactor",
            ChangeCategory::Documentation => "documentation",
            ChangeCategory::Internal => "internal",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub category: FileCategory,
    pub added_lines: usize,
    pub removed_lines: usize,
    pub is_public_api_change: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileCategory {
    Source,
    Test,
    Docs,
    Config,
    Ci,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovedItem {
    pub name: String,
    pub item_type: String,
    pub from_path: String,
    pub to_path: String,
    pub package: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffAnalysis {
    pub packages: Vec<String>,
    pub file_changes: Vec<FileChange>,
    pub commit_categories: Vec<ChangeCategory>,
    pub dominant_category: ChangeCategory,
    pub is_breaking: bool,
    pub crate_changes: Vec<CrateChange>,
    pub audience: Vec<Audience>,
    pub summary_hints: Vec<String>,
    pub moved_items: Vec<MovedItem>,
}

pub struct PrContext {
    pub pr_number: u64,
    pub title: String,
    pub body: Option<String>,
    pub labels: Vec<String>,
    pub commit_messages: Vec<String>,
    pub author: String,
}

pub fn analyze_diff(diff: &str) -> DiffAnalysis {
    let file_changes = parse_file_changes(diff);
    let packages = extract_packages(&file_changes);
    let moved_items = detect_moves(diff);
    let is_breaking =
        detect_breaking_changes(diff, &file_changes, &moved_items);
    let crate_changes = determine_crate_changes(
        &packages,
        &file_changes,
        is_breaking,
        &moved_items,
    );
    let audience = infer_audience(&file_changes, &packages);
    let summary_hints =
        generate_summary_hints(&file_changes, is_breaking, &moved_items);
    let dominant_category = infer_dominant_category(&file_changes);

    DiffAnalysis {
        packages,
        file_changes,
        commit_categories: Vec::new(),
        dominant_category,
        is_breaking,
        crate_changes,
        audience,
        summary_hints,
        moved_items,
    }
}

pub fn analyze_commits(messages: &[String]) -> Vec<ChangeCategory> {
    messages.iter().map(|m| classify_commit(m)).collect()
}

pub fn classify_commit(message: &str) -> ChangeCategory {
    let lower = message.to_lowercase();
    let trimmed = lower.trim();

    if trimmed.starts_with("feat!") || trimmed.starts_with("feat!(") {
        return ChangeCategory::BreakingChange;
    }
    if trimmed.starts_with("fix!") || trimmed.starts_with("fix!(") {
        return ChangeCategory::BreakingChange;
    }
    if trimmed.contains("breaking change") || trimmed.contains("breaking:") {
        return ChangeCategory::BreakingChange;
    }
    if trimmed.starts_with("feat:") || trimmed.starts_with("feat(") {
        return ChangeCategory::NewFeature;
    }
    if trimmed.starts_with("fix:") || trimmed.starts_with("fix(") {
        return ChangeCategory::BugFix;
    }
    if trimmed.starts_with("refactor:") || trimmed.starts_with("refactor(") {
        return ChangeCategory::Refactor;
    }
    if trimmed.starts_with("doc:") || trimmed.starts_with("docs:") {
        return ChangeCategory::Documentation;
    }
    if trimmed.starts_with("chore:")
        || trimmed.starts_with("ci:")
        || trimmed.starts_with("test:")
        || trimmed.starts_with("style:")
    {
        return ChangeCategory::Internal;
    }

    if lower.contains("add") || lower.contains("implement") {
        return ChangeCategory::NewFeature;
    }
    if lower.contains("fix") || lower.contains("resolve") {
        return ChangeCategory::BugFix;
    }
    if lower.contains("remove") || lower.contains("deprecate") {
        return ChangeCategory::BreakingChange;
    }
    if lower.contains("refactor") || lower.contains("restructure") {
        return ChangeCategory::Refactor;
    }
    if lower.contains("document") || lower.contains("readme") {
        return ChangeCategory::Documentation;
    }

    ChangeCategory::Internal
}

fn parse_file_changes(diff: &str) -> Vec<FileChange> {
    let mut changes = Vec::new();
    let mut current_path: Option<String> = None;
    let mut added = 0usize;
    let mut removed = 0usize;
    let mut is_public_api = false;

    for line in diff.lines() {
        if line.starts_with("diff --git") {
            if let Some(path) = current_path.take() {
                let cat = classify_file_path(&path);
                changes.push(FileChange {
                    path,
                    category: cat,
                    added_lines: added,
                    removed_lines: removed,
                    is_public_api_change: is_public_api,
                });
            }
            added = 0;
            removed = 0;
            is_public_api = false;
            if let Some(path) = extract_path_from_diff_line(line) {
                current_path = Some(path);
            }
        } else if line.starts_with('+') && !line.starts_with("+++") {
            added += 1;
            if line.contains("pub fn ")
                || line.contains("pub struct ")
                || line.contains("pub enum ")
                || line.contains("pub trait ")
            {
                is_public_api = true;
            }
        } else if line.starts_with('-') && !line.starts_with("---") {
            removed += 1;
            if line.contains("pub fn ")
                || line.contains("pub struct ")
                || line.contains("pub enum ")
                || line.contains("pub trait ")
            {
                is_public_api = true;
            }
        }
    }

    if let Some(path) = current_path {
        let cat = classify_file_path(&path);
        changes.push(FileChange {
            path,
            category: cat,
            added_lines: added,
            removed_lines: removed,
            is_public_api_change: is_public_api,
        });
    }

    changes
}

fn extract_path_from_diff_line(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.splitn(4, ' ').collect();
    if parts.len() >= 4 {
        let path = parts[3].trim_start_matches("a/").trim_start_matches("b/");
        Some(path.to_string())
    } else {
        None
    }
}

fn classify_file_path(path: &str) -> FileCategory {
    if path.contains("/tests/") || path.ends_with("_test.rs") {
        FileCategory::Test
    } else if path.ends_with(".md") || path.contains("/docs/") {
        FileCategory::Docs
    } else if path.starts_with(".github/")
        || path.ends_with(".yml")
        || path.ends_with(".yaml")
    {
        FileCategory::Ci
    } else if path.ends_with("Cargo.toml")
        || path.ends_with("Cargo.lock")
        || path.ends_with(".toml")
        || path.ends_with(".json")
    {
        FileCategory::Config
    } else {
        FileCategory::Source
    }
}

fn extract_package_from_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').collect();
    for (i, part) in parts.iter().enumerate() {
        if (*part == "packages" || *part == "apps") && i + 1 < parts.len() {
            return Some(parts[i + 1].to_string());
        }
    }
    None
}

fn extract_packages(file_changes: &[FileChange]) -> Vec<String> {
    let mut packages = Vec::new();
    for change in file_changes {
        if let Some(pkg) = extract_package_from_path(&change.path)
            && !packages.contains(&pkg)
        {
            packages.push(pkg);
        }
    }
    packages.sort();
    packages
}

fn detect_breaking_changes(
    diff: &str,
    file_changes: &[FileChange],
    moved_items: &[MovedItem],
) -> bool {
    let _moved_keys: HashSet<(String, String)> = moved_items
        .iter()
        .map(|m| (m.name.clone(), m.item_type.clone()))
        .collect();

    for change in file_changes {
        if change.removed_lines > 0 && change.is_public_api_change {
            return true;
        }
    }

    let lower = diff.to_lowercase();
    if lower.contains("breaking change")
        || lower.contains("breaking:")
        || lower.contains("//! breaking")
    {
        return true;
    }

    false
}

fn determine_bump_for_package(
    pkg: &str,
    file_changes: &[FileChange],
    is_breaking: bool,
    moved_items: &[MovedItem],
) -> BumpLevel {
    if is_breaking {
        let pkg_moved_names: HashSet<&str> = moved_items
            .iter()
            .filter(|m| m.package.as_deref() == Some(pkg))
            .map(|m| m.name.as_str())
            .collect();

        let pkg_has_public_api_removal = file_changes.iter().any(|c| {
            extract_package_from_path(&c.path) == Some(pkg.to_string())
                && c.is_public_api_change
                && c.removed_lines > 0
        });

        let has_actual_removal =
            pkg_has_public_api_removal && pkg_moved_names.is_empty();

        return if has_actual_removal {
            BumpLevel::Major
        } else {
            BumpLevel::Minor
        };
    }

    let pkg_changes: Vec<&FileChange> = file_changes
        .iter()
        .filter(|c| extract_package_from_path(&c.path) == Some(pkg.to_string()))
        .collect();

    let has_source = pkg_changes
        .iter()
        .any(|c| c.category == FileCategory::Source);
    let has_new_public = pkg_changes.iter().any(|c| {
        c.is_public_api_change && c.added_lines > 0 && c.removed_lines == 0
    });

    if has_new_public {
        return BumpLevel::Minor;
    }

    if has_source {
        return BumpLevel::Patch;
    }

    BumpLevel::None
}

fn determine_crate_changes(
    packages: &[String],
    file_changes: &[FileChange],
    is_breaking: bool,
    moved_items: &[MovedItem],
) -> Vec<CrateChange> {
    packages
        .iter()
        .map(|pkg| CrateChange {
            name: pkg.clone(),
            bump: determine_bump_for_package(
                pkg,
                file_changes,
                is_breaking,
                moved_items,
            ),
            validate: true,
            note: None,
        })
        .collect()
}

fn infer_audience(
    file_changes: &[FileChange],
    packages: &[String],
) -> Vec<Audience> {
    let mut audience = Vec::new();

    let has_framework_code = packages.iter().any(|p| {
        matches!(
            p.as_str(),
            "core" | "cli" | "agent" | "fmt" | "bench" | "utils" | "runner"
        )
    });
    let has_app_code = packages
        .iter()
        .any(|p| matches!(p.as_str(), "orm" | "validator" | "test"));
    let has_agent_feature = file_changes.iter().any(|c| {
        c.path.contains("agent")
            || c.path.contains("skill")
            || c.path.contains("prdoc")
    });
    let has_ci = file_changes.iter().any(|c| c.category == FileCategory::Ci);

    if has_framework_code {
        audience.push(Audience::FrameworkDev);
    }
    if has_app_code {
        audience.push(Audience::AppDev);
    }
    if has_agent_feature {
        audience.push(Audience::AgentUser);
    }
    if has_ci {
        audience.push(Audience::Operator);
    }

    if audience.is_empty() {
        audience.push(Audience::AppDev);
    }

    audience
}

fn infer_dominant_category(file_changes: &[FileChange]) -> ChangeCategory {
    let mut counts: HashMap<ChangeCategory, usize> = HashMap::new();

    for change in file_changes {
        let cat = match change.category {
            FileCategory::Source => {
                if change.is_public_api_change && change.added_lines > 0 {
                    ChangeCategory::NewFeature
                } else if change.is_public_api_change
                    && change.removed_lines > 0
                {
                    ChangeCategory::BreakingChange
                } else {
                    ChangeCategory::Refactor
                }
            }
            FileCategory::Test => ChangeCategory::Internal,
            FileCategory::Docs => ChangeCategory::Documentation,
            FileCategory::Config => ChangeCategory::Internal,
            FileCategory::Ci => ChangeCategory::Internal,
        };
        *counts.entry(cat).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(_, c)| *c)
        .map(|(cat, _)| cat)
        .unwrap_or(ChangeCategory::Internal)
}

fn generate_summary_hints(
    file_changes: &[FileChange],
    is_breaking: bool,
    moved_items: &[MovedItem],
) -> Vec<String> {
    let mut hints = Vec::new();

    if is_breaking {
        hints.push("Contains breaking changes".to_string());
    }

    if !moved_items.is_empty() {
        hints.push(format!(
            "Moved {} public API item(s) to new location(s)",
            moved_items.len()
        ));
    }

    let new_public: Vec<&FileChange> = file_changes
        .iter()
        .filter(|c| c.is_public_api_change && c.added_lines > 0)
        .collect();
    if !new_public.is_empty() {
        hints.push(format!(
            "Adds new public API surface in {} file(s)",
            new_public.len()
        ));
    }

    let source_changes: Vec<&FileChange> = file_changes
        .iter()
        .filter(|c| c.category == FileCategory::Source)
        .collect();
    if !source_changes.is_empty() {
        let total_added: usize =
            source_changes.iter().map(|c| c.added_lines).sum();
        let total_removed: usize =
            source_changes.iter().map(|c| c.removed_lines).sum();
        hints.push(format!(
            "Modifies {} source file(s) (+{}/-{})",
            source_changes.len(),
            total_added,
            total_removed
        ));
    }

    hints
}

fn detect_moves(diff: &str) -> Vec<MovedItem> {
    let mut removals: Vec<(String, String, String)> = Vec::new();
    let mut additions: Vec<(String, String, String)> = Vec::new();
    let mut current_path = String::new();

    for line in diff.lines() {
        if line.starts_with("diff --git") {
            if let Some(path) = extract_path_from_diff_line(line) {
                current_path = path;
            }
        } else if line.starts_with('-')
            && !line.starts_with("---")
            && let Some((name, item_type)) = parse_public_api_item(line)
        {
            removals.push((name, item_type, current_path.clone()));
        } else if line.starts_with('+')
            && !line.starts_with("+++")
            && let Some((name, item_type)) = parse_public_api_item(line)
        {
            additions.push((name, item_type, current_path.clone()));
        }
    }

    let mut moved_items = Vec::new();
    let mut used_additions: HashSet<usize> = HashSet::new();

    for (rem_name, rem_type, rem_path) in &removals {
        for (i, (add_name, add_type, add_path)) in additions.iter().enumerate()
        {
            if !used_additions.contains(&i)
                && rem_name == add_name
                && rem_type == add_type
                && rem_path != add_path
            {
                let package = extract_package_from_path(add_path);
                moved_items.push(MovedItem {
                    name: rem_name.clone(),
                    item_type: rem_type.clone(),
                    from_path: rem_path.clone(),
                    to_path: add_path.clone(),
                    package,
                });
                used_additions.insert(i);
                break;
            }
        }
    }

    moved_items
}

fn parse_public_api_item(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_start_matches('-').trim_start_matches('+').trim();

    if trimmed.starts_with("//")
        || trimmed.starts_with("//!")
        || trimmed.starts_with("///")
        || trimmed.starts_with("#[")
        || trimmed.starts_with("mod ")
        || trimmed.starts_with("use ")
        || trimmed.starts_with("type ")
        || trimmed.starts_with("macro_rules!")
    {
        return None;
    }

    if trimmed.starts_with("pub trait ") {
        let name = trimmed
            .strip_prefix("pub trait ")
            .unwrap_or("")
            .split('<')
            .next()
            .unwrap_or("")
            .split('{')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        if name.len() >= 2 {
            return Some((name, "trait".to_string()));
        }
    }

    if trimmed.starts_with("pub struct ") {
        let name = trimmed
            .strip_prefix("pub struct ")
            .unwrap_or("")
            .split('<')
            .next()
            .unwrap_or("")
            .split('{')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        if name.len() >= 2 {
            return Some((name, "struct".to_string()));
        }
    }

    if trimmed.starts_with("pub enum ") {
        let name = trimmed
            .strip_prefix("pub enum ")
            .unwrap_or("")
            .split('<')
            .next()
            .unwrap_or("")
            .split('{')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        if name.len() >= 2 {
            return Some((name, "enum".to_string()));
        }
    }

    if trimmed.starts_with("pub fn ") || trimmed.starts_with("pub async fn ") {
        let rest = if trimmed.starts_with("pub async fn ") {
            trimmed.strip_prefix("pub async fn ").unwrap_or("")
        } else {
            trimmed.strip_prefix("pub fn ").unwrap_or("")
        };
        let name = rest.split('(').next().unwrap_or("").trim().to_string();
        if name.len() >= 2 {
            return Some((name, "fn".to_string()));
        }
    }

    None
}

pub fn gather_pr_context_from_gh(pr_number: u64) -> Option<PrContext> {
    let output = std::process::Command::new("gh")
        .args([
            "pr",
            "view",
            &pr_number.to_string(),
            "--json",
            "title,body,labels,commits,author",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&json_str).ok()?;

    let title = json["title"].as_str().unwrap_or("").to_string();
    let body = json["body"].as_str().map(|s| s.to_string());
    let labels = json["labels"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|l| l["name"].as_str().unwrap_or("").to_string())
                .collect()
        })
        .unwrap_or_default();
    let commit_messages = json["commits"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|c| {
                    c["messageHeadline"].as_str().map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default();
    let author = json["author"]
        .get("login")
        .and_then(|l| l.as_str())
        .unwrap_or("@unknown")
        .to_string();

    Some(PrContext {
        pr_number,
        title,
        body,
        labels,
        commit_messages,
        author,
    })
}

pub fn get_diff_for_pr(pr_number: u64) -> Option<String> {
    let output = std::process::Command::new("gh")
        .args(["pr", "diff", &pr_number.to_string()])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

pub fn get_diff_for_range(range: &str) -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["diff", range])
        .output()
        .ok()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout).to_string();
        if s.is_empty() { None } else { Some(s) }
    } else {
        None
    }
}

pub fn get_commit_messages_for_range(range: &str) -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(["log", "--oneline", range])
        .output()
        .ok();

    match output {
        Some(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|l| {
                    let parts: Vec<&str> = l.splitn(2, ' ').collect();
                    parts.get(1).unwrap_or(&"").to_string()
                })
                .filter(|s| !s.is_empty())
                .collect()
        }
        _ => Vec::new(),
    }
}
