use crate::{
    analyzer::{
        ChangeCategory, DiffAnalysis, FileCategory, FileChange, MovedItem,
        PrContext,
    },
    types::CrateChange,
};
use std::collections::{HashMap, HashSet};

pub struct SummaryContext<'a> {
    pub analysis: &'a DiffAnalysis,
    pub context: Option<&'a PrContext>,
    pub public_api_additions: Vec<PublicApiChange>,
    pub public_api_removals: Vec<PublicApiChange>,
    pub moved_items: Vec<MovedItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicApiChange {
    pub item_type: String,
    pub name: String,
    pub package: Option<String>,
    pub path: String,
    pub moved_from: Option<String>,
}

const NOISE_NAMES: &[&str] = &[
    "as_str",
    "from_str_lossy",
    "dominates",
    "provider_name",
    "ensure_model",
    "new",
];

const MAX_ITEMS_PER_CATEGORY: usize = 12;

pub fn generate_rich_summary(ctx: &SummaryContext) -> String {
    let mut out = String::new();

    let narrative = build_narrative_sentence(ctx);

    if !narrative.is_empty() {
        out.push_str(&narrative);
        out.push_str("\n\n");
    }

    let (additions, removals, moves) = filter_and_dedup_api_changes(
        &ctx.public_api_additions,
        &ctx.public_api_removals,
        &ctx.moved_items,
    );

    let has_additions = !additions.is_empty();
    let has_removals = !removals.is_empty();
    let has_moves = !moves.is_empty();
    let has_pkg_changes = !ctx.analysis.packages.is_empty();

    if has_additions || has_removals || has_moves || has_pkg_changes {
        out.push_str("### Key Changes\n");

        if has_additions {
            let grouped = group_api_by_package_and_type(&additions);
            out.push_str(&format_additions_section(&grouped));
        }

        if has_moves {
            out.push_str(&format_moved_section(&moves));
        }

        if has_removals {
            let grouped = group_api_by_package_and_type(&removals);
            out.push_str(&format_removals_section(&grouped));
        }

        if has_pkg_changes {
            out.push_str(&format_package_breakdown(
                &ctx.analysis.packages,
                &ctx.analysis.crate_changes,
                &ctx.analysis.file_changes,
            ));
        }

        if ctx.analysis.is_breaking {
            out.push_str(
                "\n**Breaking:** This change modifies the public API in a \
                 backward-incompatible way.",
            );
        }
        out.push('\n');
    }

    if out.is_empty() {
        return ctx.analysis.summary_hints.join(". ");
    }

    out
}

fn build_narrative_sentence(ctx: &SummaryContext) -> String {
    if let Some(context) = ctx.context {
        if let Some(ref body) = context.body {
            let excerpt = extract_first_paragraph(body);
            if !excerpt.is_empty() {
                return excerpt;
            }
        }

        if !context.commit_messages.is_empty() {
            let narrative = build_commit_narrative(&context.commit_messages);
            if !narrative.is_empty() {
                return narrative;
            }
        }
    }

    let pkg_count = ctx.analysis.packages.len();
    if pkg_count == 0 {
        return String::new();
    }

    let verb = match ctx.analysis.dominant_category {
        ChangeCategory::NewFeature => "Adds functionality across",
        ChangeCategory::BugFix => "Fixes issues in",
        ChangeCategory::BreakingChange => "Introduces breaking changes to",
        ChangeCategory::Refactor => "Refactors",
        ChangeCategory::Documentation => "Updates documentation for",
        ChangeCategory::Internal => "Updates",
    };

    format!(
        "{verb} {} package(s): {}.",
        pkg_count,
        ctx.analysis.packages.join(", ")
    )
}

fn extract_first_paragraph(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let first_break = trimmed
        .find("\n\n")
        .unwrap_or_else(|| trimmed.find("\r\n\r\n").unwrap_or(trimmed.len()));
    let paragraph = &trimmed[..first_break];
    let cleaned = paragraph
        .lines()
        .map(|l| l.trim())
        .filter(|l| {
            !l.is_empty() && !l.starts_with("##") && !l.starts_with("###")
        })
        .collect::<Vec<_>>()
        .join(" ");
    if cleaned.len() > 500 {
        let trunc = cleaned.chars().take(500).collect::<String>();
        format!("{trunc}...")
    } else {
        cleaned
    }
}

fn build_commit_narrative(messages: &[String]) -> String {
    let mut groups: HashMap<ChangeCategory, Vec<String>> = HashMap::new();
    for msg in messages {
        let cat = crate::analyzer::classify_commit(msg);
        groups.entry(cat).or_default().push(msg.clone());
    }

    let mut sentences = Vec::new();

    let order = [
        ChangeCategory::NewFeature,
        ChangeCategory::BugFix,
        ChangeCategory::Refactor,
        ChangeCategory::BreakingChange,
        ChangeCategory::Documentation,
        ChangeCategory::Internal,
    ];

    for cat in &order {
        if let Some(msgs) = groups.get(cat) {
            let descriptions = extract_descriptions(msgs);
            if descriptions.is_empty() {
                continue;
            }
            let prefix = match cat {
                ChangeCategory::NewFeature => "Adds",
                ChangeCategory::BugFix => "Fixes",
                ChangeCategory::Refactor => "Refactors",
                ChangeCategory::BreakingChange => "Breaking:",
                ChangeCategory::Documentation => "Docs:",
                ChangeCategory::Internal => "Chore:",
            };
            let text = descriptions.join(", ");
            sentences.push(format!("{prefix} {text}."));
        }
    }

    sentences.join(" ")
}

fn extract_descriptions(messages: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
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
            if after_prefix.is_empty() || !seen.insert(after_prefix.to_string())
            {
                None
            } else {
                Some(after_prefix.to_string())
            }
        })
        .collect()
}

fn filter_and_dedup_api_changes<'a>(
    additions: &'a [PublicApiChange],
    removals: &'a [PublicApiChange],
    moved_items: &[MovedItem],
) -> (
    Vec<&'a PublicApiChange>,
    Vec<&'a PublicApiChange>,
    Vec<&'a MovedItem>,
) {
    let moved_keys: HashSet<(String, String)> = moved_items
        .iter()
        .map(|m| (m.name.clone(), m.item_type.clone()))
        .collect();

    let filtered_adds: Vec<&PublicApiChange> = additions
        .iter()
        .filter(|a| {
            !is_noise_item(a)
                && !is_test_item(a)
                && !moved_keys.contains(&(a.name.clone(), a.item_type.clone()))
        })
        .collect();

    let addition_names: HashSet<&str> =
        filtered_adds.iter().map(|a| a.name.as_str()).collect();

    let filtered_removals: Vec<&PublicApiChange> = removals
        .iter()
        .filter(|r| {
            !is_noise_item(r)
                && !is_test_item(r)
                && !addition_names.contains(r.name.as_str())
                && !moved_keys.contains(&(r.name.clone(), r.item_type.clone()))
        })
        .collect();

    (filtered_adds, filtered_removals, moved_items.iter().collect())
}

fn is_noise_item(change: &PublicApiChange) -> bool {
    NOISE_NAMES.contains(&change.name.as_str())
}

fn is_test_item(change: &PublicApiChange) -> bool {
    change.path.contains("/tests/")
        || change.path.ends_with("_test.rs")
        || change.name.starts_with("test_")
        || change.name.starts_with("Test")
}

fn group_api_by_package_and_type<'a>(
    changes: &[&'a PublicApiChange],
) -> HashMap<Option<String>, HashMap<&'a str, Vec<&'a str>>> {
    let mut by_pkg: HashMap<Option<String>, HashMap<&str, Vec<&str>>> =
        HashMap::new();
    for change in changes {
        let pkg_entry = by_pkg.entry(change.package.clone()).or_default();
        pkg_entry
            .entry(&change.item_type)
            .or_default()
            .push(&change.name);
    }
    by_pkg
}

fn format_additions_section(
    grouped: &HashMap<Option<String>, HashMap<&str, Vec<&str>>>,
) -> String {
    if grouped.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push_str("**Added**\n");

    let type_order = ["trait", "struct", "enum", "fn"];

    for (pkg, by_type) in grouped {
        let pkg_label = pkg.as_deref().unwrap_or("project");
        let mut lines = Vec::new();
        for item_type in &type_order {
            if let Some(names) = by_type.get(item_type) {
                let mut unique: Vec<&str> = names.to_vec();
                unique.sort();
                unique.dedup();
                let label = match *item_type {
                    "trait" => "traits",
                    "struct" => "structs",
                    "enum" => "enums",
                    _ => "functions",
                };
                let (shown, rest) = split_and_ellipsis(&unique);
                lines.push(format!("`{}` {label}", shown.join("`, `")));
                if let Some(count) = rest {
                    lines.push(format!("...and {count} more"));
                }
            }
        }
        if !lines.is_empty() {
            out.push_str(&format!("- **{pkg_label}**: {}\n", lines.join("; ")));
        }
    }
    out
}

fn format_removals_section(
    grouped: &HashMap<Option<String>, HashMap<&str, Vec<&str>>>,
) -> String {
    if grouped.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push_str("**Removed**\n");

    for (pkg, by_type) in grouped {
        let pkg_label = pkg.as_deref().unwrap_or("project");
        let mut names: Vec<&str> = Vec::new();
        for list in by_type.values() {
            names.extend(list.iter().copied());
        }
        names.sort();
        names.dedup();
        if names.is_empty() {
            continue;
        }
        let (shown, rest) = split_and_ellipsis(&names);
        out.push_str(&format!("- **{pkg_label}**: `{}`", shown.join("`, `")));
        if let Some(count) = rest {
            out.push_str(&format!("; ...and {count} more"));
        }
        out.push('\n');
    }
    out
}

fn format_moved_section(moves: &[&MovedItem]) -> String {
    if moves.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push_str("**Moved**\n");

    let grouped: HashMap<Option<String>, Vec<&MovedItem>> = moves
        .iter()
        .filter_map(|m| {
            let pkg = m.package.clone();
            Some((pkg, *m))
        })
        .fold(HashMap::new(), |mut acc, (pkg, m)| {
            acc.entry(pkg).or_default().push(m);
            acc
        });

    for (pkg, items) in grouped {
        let pkg_label = pkg.as_deref().unwrap_or("project");
        let mut details: Vec<String> = items
            .iter()
            .map(|m| {
                format!(
                    "`{}` ({}: {} → {})",
                    m.name,
                    m.item_type,
                    shorten_path(&m.from_path),
                    shorten_path(&m.to_path)
                )
            })
            .collect();
        details.sort();
        let (shown, rest) = split_and_ellipsis_str(&details);
        out.push_str(&format!("- **{pkg_label}**: {}", shown.join("; ")));
        if let Some(count) = rest {
            out.push_str(&format!("; ...and {count} more"));
        }
        out.push('\n');
    }
    out
}

fn shorten_path(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() > 3 {
        parts[parts.len() - 2..].join("/")
    } else {
        path.to_string()
    }
}

fn split_and_ellipsis_str(items: &[String]) -> (Vec<&str>, Option<usize>) {
    if items.len() <= MAX_ITEMS_PER_CATEGORY {
        (items.iter().map(|s| s.as_str()).collect(), None)
    } else {
        let shown: Vec<&str> = items[..MAX_ITEMS_PER_CATEGORY]
            .iter()
            .map(|s| s.as_str())
            .collect();
        let rest = items.len() - MAX_ITEMS_PER_CATEGORY;
        (shown, Some(rest))
    }
}

fn format_package_breakdown(
    packages: &[String],
    crate_changes: &[CrateChange],
    file_changes: &[FileChange],
) -> String {
    let mut out = String::new();
    out.push_str("### Package Breakdown\n");

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
        let total_added: usize = pkg_files.iter().map(|c| c.added_lines).sum();
        let total_removed: usize =
            pkg_files.iter().map(|c| c.removed_lines).sum();

        let bump = crate_changes
            .iter()
            .find(|c| c.name == *pkg)
            .map(|c| c.bump.as_str())
            .unwrap_or("none");

        let line_str = if total_added > 0 || total_removed > 0 {
            format!(" (+{total_added}/-{total_removed})")
        } else {
            String::new()
        };

        out.push_str(&format!(
            "- **{pkg}** ({bump}): {source_count} source file(s){line_str}\n",
        ));
    }
    out.push('\n');
    out
}

fn split_and_ellipsis<'a>(
    names: &'a [&'a str],
) -> (Vec<&'a str>, Option<usize>) {
    if names.len() <= MAX_ITEMS_PER_CATEGORY {
        (names.to_vec(), None)
    } else {
        let shown: Vec<&str> = names[..MAX_ITEMS_PER_CATEGORY].to_vec();
        let rest = names.len() - MAX_ITEMS_PER_CATEGORY;
        (shown, Some(rest))
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

    let patterns = [
        ("trait", r"pub\s+trait\s+(\w+)"),
        ("struct", r"pub\s+struct\s+(\w+)"),
        ("enum", r"pub\s+enum\s+(\w+)"),
        ("fn", r"pub\s+(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)"),
    ];

    for (item_type, pattern) in &patterns {
        if let Ok(re) = regex::Regex::new(pattern)
            && let Some(caps) = re.captures(trimmed)
        {
            let name = caps[1].to_string();
            if name.len() < 2 {
                return None;
            }
            let package = extract_package_from_path(path);
            return Some(PublicApiChange {
                item_type: item_type.to_string(),
                name,
                package,
                path: path.to_string(),
                moved_from: None,
            });
        }
    }

    None
}
