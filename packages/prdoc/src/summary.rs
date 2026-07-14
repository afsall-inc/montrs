use crate::{
    analyzer::{
        ChangeCategory, DiffAnalysis, FileCategory, FileChange, PrContext,
    },
    types::CrateChange,
};
use std::collections::HashMap;

pub struct SummaryContext<'a> {
    pub analysis: &'a DiffAnalysis,
    pub context: Option<&'a PrContext>,
    pub public_api_additions: Vec<PublicApiChange>,
    pub public_api_removals: Vec<PublicApiChange>,
}

#[derive(Debug, Clone)]
pub struct PublicApiChange {
    pub item_type: String,
    pub name: String,
    pub package: Option<String>,
    pub path: String,
}

pub fn generate_rich_summary(ctx: &SummaryContext) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(context) = ctx.context
        && !context.commit_messages.is_empty()
    {
        let narrative = build_commit_narrative(&context.commit_messages);
        if !narrative.is_empty() {
            parts.push(narrative);
        }
    }

    if !ctx.public_api_additions.is_empty() {
        let additions = describe_api_additions(&ctx.public_api_additions);
        parts.push(additions);
    }

    if !ctx.public_api_removals.is_empty() {
        let removals = describe_api_removals(&ctx.public_api_removals);
        parts.push(removals);
    }

    if !ctx.analysis.packages.is_empty() {
        let pkg_summary = describe_package_changes(
            &ctx.analysis.packages,
            &ctx.analysis.crate_changes,
            &ctx.analysis.file_changes,
        );
        parts.push(pkg_summary);
    }

    if ctx.analysis.is_breaking {
        parts.push(
            "This change introduces breaking modifications to the public API."
                .to_string(),
        );
    }

    if parts.is_empty() {
        return ctx.analysis.summary_hints.join(". ");
    }

    parts.join(" ")
}

fn build_commit_narrative(messages: &[String]) -> String {
    let mut groups: HashMap<ChangeCategory, Vec<String>> = HashMap::new();
    for msg in messages {
        let cat = crate::analyzer::classify_commit(msg);
        groups.entry(cat).or_default().push(msg.clone());
    }

    let mut sentences = Vec::new();

    if let Some(feats) = groups.get(&ChangeCategory::NewFeature) {
        let descriptions = extract_descriptions(feats);
        if !descriptions.is_empty() {
            sentences.push(format!("Adds {}.", descriptions.join(", ")));
        }
    }

    if let Some(fixes) = groups.get(&ChangeCategory::BugFix) {
        let descriptions = extract_descriptions(fixes);
        if !descriptions.is_empty() {
            sentences.push(format!("Fixes {}.", descriptions.join(", ")));
        }
    }

    if let Some(refactors) = groups.get(&ChangeCategory::Refactor) {
        let descriptions = extract_descriptions(refactors);
        if !descriptions.is_empty() {
            sentences.push(format!("Refactors {}.", descriptions.join(", ")));
        }
    }

    if let Some(docs) = groups.get(&ChangeCategory::Documentation) {
        let descriptions = extract_descriptions(docs);
        if !descriptions.is_empty() {
            sentences.push(format!(
                "Updates documentation for {}.",
                descriptions.join(", ")
            ));
        }
    }

    if let Some(breaking) = groups.get(&ChangeCategory::BreakingChange) {
        let descriptions = extract_descriptions(breaking);
        if !descriptions.is_empty() {
            sentences.push(format!("Breaking: {}.", descriptions.join(", ")));
        }
    }

    sentences.join(" ")
}

fn extract_descriptions(messages: &[String]) -> Vec<String> {
    messages
        .iter()
        .filter_map(|msg| {
            let trimmed = msg.trim();
            let after_prefix = if let Some(rest) = trimmed
                .strip_prefix("feat:")
                .or_else(|| trimmed.strip_prefix("feat("))
                .or_else(|| trimmed.strip_prefix("fix:"))
                .or_else(|| trimmed.strip_prefix("fix("))
                .or_else(|| trimmed.strip_prefix("refactor:"))
                .or_else(|| trimmed.strip_prefix("refactor("))
                .or_else(|| trimmed.strip_prefix("docs:"))
                .or_else(|| trimmed.strip_prefix("doc:"))
                .or_else(|| trimmed.strip_prefix("chore:"))
                .or_else(|| trimmed.strip_prefix("breaking:"))
            {
                rest.trim()
            } else {
                trimmed
            };
            if after_prefix.is_empty() {
                None
            } else {
                Some(after_prefix.to_string())
            }
        })
        .collect()
}

fn describe_api_additions(additions: &[PublicApiChange]) -> String {
    let by_type = group_by_item_type(additions);
    let mut parts = Vec::new();

    if let Some(fns) = by_type.get("fn") {
        let names: Vec<&str> = fns.iter().map(|a| a.name.as_str()).collect();
        parts.push(format!("new functions {}", names.join(", ")));
    }
    if let Some(structs) = by_type.get("struct") {
        let names: Vec<&str> =
            structs.iter().map(|a| a.name.as_str()).collect();
        parts.push(format!("new structs {}", names.join(", ")));
    }
    if let Some(enums) = by_type.get("enum") {
        let names: Vec<&str> = enums.iter().map(|a| a.name.as_str()).collect();
        parts.push(format!("new enums {}", names.join(", ")));
    }
    if let Some(traits) = by_type.get("trait") {
        let names: Vec<&str> = traits.iter().map(|a| a.name.as_str()).collect();
        parts.push(format!("new traits {}", names.join(", ")));
    }

    if parts.is_empty() {
        return String::new();
    }

    format!("Introduces {}.", parts.join(", "))
}

fn describe_api_removals(removals: &[PublicApiChange]) -> String {
    let names: Vec<&str> = removals.iter().map(|a| a.name.as_str()).collect();
    if names.is_empty() {
        return String::new();
    }
    format!("Removes {}.", names.join(", "))
}

fn describe_package_changes(
    packages: &[String],
    crate_changes: &[CrateChange],
    file_changes: &[FileChange],
) -> String {
    let mut parts = Vec::new();

    for pkg in packages {
        let pkg_files: Vec<&FileChange> = file_changes
            .iter()
            .filter(|c| {
                extract_package_from_path(&c.path)
                    .as_ref()
                    .map(|p| p == pkg)
                    .unwrap_or(false)
            })
            .collect();

        let source_count = pkg_files
            .iter()
            .filter(|c| c.category == FileCategory::Source)
            .count();
        let test_count = pkg_files
            .iter()
            .filter(|c| c.category == FileCategory::Test)
            .count();
        let doc_count = pkg_files
            .iter()
            .filter(|c| c.category == FileCategory::Docs)
            .count();

        let total_added: usize = pkg_files.iter().map(|c| c.added_lines).sum();
        let total_removed: usize =
            pkg_files.iter().map(|c| c.removed_lines).sum();

        let bump = crate_changes
            .iter()
            .find(|c| c.name == *pkg)
            .map(|c| c.bump.as_str())
            .unwrap_or("none");

        let mut details = Vec::new();
        if source_count > 0 {
            details.push(format!("{} source file(s)", source_count));
        }
        if test_count > 0 {
            details.push(format!("{} test file(s)", test_count));
        }
        if doc_count > 0 {
            details.push(format!("{} doc file(s)", doc_count));
        }

        let detail_str = if details.is_empty() {
            String::new()
        } else {
            format!(" ({})", details.join(", "))
        };

        let line_str = if total_added > 0 || total_removed > 0 {
            format!("+{}/-{}", total_added, total_removed)
        } else {
            String::new()
        };

        parts.push(format!(
            "{}{} [{}]{}",
            pkg,
            detail_str,
            bump,
            if line_str.is_empty() {
                String::new()
            } else {
                format!(" {}", line_str)
            }
        ));
    }

    if parts.is_empty() {
        return String::new();
    }

    format!("Affects: {}.", parts.join(", "))
}

fn group_by_item_type(
    changes: &[PublicApiChange],
) -> HashMap<&str, Vec<&PublicApiChange>> {
    let mut map: HashMap<&str, Vec<&PublicApiChange>> = HashMap::new();
    for change in changes {
        map.entry(&change.item_type).or_default().push(change);
    }
    map
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

pub fn extract_public_api_from_diff(
    diff: &str,
) -> (Vec<PublicApiChange>, Vec<PublicApiChange>) {
    let mut additions = Vec::new();
    let mut removals = Vec::new();
    let mut current_path = String::new();

    for line in diff.lines() {
        if line.starts_with("diff --git")
            && let Some(path) = extract_path_from_diff_line(line)
        {
            current_path = path;
        } else if line.starts_with('+')
            && !line.starts_with("+++")
            && let Some(api) = parse_public_api_line(line, &current_path)
        {
            additions.push(api);
        } else if line.starts_with('-')
            && !line.starts_with("---")
            && let Some(api) = parse_public_api_line(line, &current_path)
        {
            removals.push(api);
        }
    }

    (additions, removals)
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

fn parse_public_api_line(line: &str, path: &str) -> Option<PublicApiChange> {
    let trimmed = line.trim_start_matches('+').trim_start_matches('-').trim();

    let patterns = [
        ("fn", r"pub\s+(?:async\s+)?fn\s+(\w+)"),
        ("struct", r"pub\s+struct\s+(\w+)"),
        ("enum", r"pub\s+enum\s+(\w+)"),
        ("trait", r"pub\s+trait\s+(\w+)"),
    ];

    for (item_type, pattern) in &patterns {
        if let Ok(re) = regex::Regex::new(pattern)
            && let Some(caps) = re.captures(trimmed)
        {
            let name = caps[1].to_string();
            let package = extract_package_from_path(path);
            return Some(PublicApiChange {
                item_type: item_type.to_string(),
                name,
                package,
                path: path.to_string(),
            });
        }
    }

    None
}
