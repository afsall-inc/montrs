#[cfg(feature = "llm")]
use crate::llm::LlmConfig;
use crate::{
    analyzer::{ChangeCategory, DiffAnalysis, PrContext, analyze_commits},
    embed,
    summary::{SummaryContext, extract_public_api_from_diff},
    types::{Audience, BumpLevel, CrateChange, PrDoc, PrDocStatus},
};
#[cfg(not(feature = "llm"))]
type LlmConfig = ();

pub fn generate_prdoc(
    analysis: &DiffAnalysis,
    context: Option<&PrContext>,
) -> PrDoc {
    let title = derive_title(context, analysis);

    let author = context
        .map(|c| c.author.clone())
        .filter(|a| !a.is_empty())
        .unwrap_or_else(|| "@unknown".to_string());

    let pr = context.map(|c| c.pr_number);

    let commit_categories = context
        .map(|c| {
            let rule_based = analyze_commits(&c.commit_messages);
            let embed_based: Vec<ChangeCategory> = c
                .commit_messages
                .iter()
                .filter_map(|m| embed::classify_by_embedding(m))
                .collect();
            merge_categories(&rule_based, &embed_based)
        })
        .unwrap_or_default();

    let dominant = if commit_categories.is_empty() {
        analysis.dominant_category.clone()
    } else {
        most_frequent_category(&commit_categories)
            .cloned()
            .unwrap_or_else(|| analysis.dominant_category.clone())
    };

    let is_breaking = analysis.is_breaking
        || commit_categories.contains(&ChangeCategory::BreakingChange);

    let crate_changes = if analysis.crate_changes.is_empty() {
        analysis
            .packages
            .iter()
            .map(|pkg| CrateChange {
                name: pkg.clone(),
                bump: default_bump_for_category(&dominant, is_breaking),
                validate: true,
            })
            .collect()
    } else {
        analysis.crate_changes.clone()
    };

    let audience = if analysis.audience.is_empty() {
        vec![Audience::AppDev]
    } else {
        analysis.audience.clone()
    };

    PrDoc {
        title,
        pr,
        author,
        status: PrDocStatus::Draft,
        packages: analysis.packages.clone(),
        breaking: is_breaking,
        needs_review: derive_needs_review(is_breaking, &dominant),
        audience,
        crates: crate_changes,
    }
}

pub fn render_prdoc(prdoc: &PrDoc, analysis: &DiffAnalysis) -> String {
    render_prdoc_rich(prdoc, analysis, None, None, None)
}

pub fn render_prdoc_rich(
    prdoc: &PrDoc,
    analysis: &DiffAnalysis,
    context: Option<&PrContext>,
    diff: Option<&str>,
    llm_config: Option<&LlmConfig>,
) -> String {
    let mut out = String::new();

    out.push_str("---\n");
    out.push_str(&yaml_kv("title", &prdoc.title));
    if let Some(pr) = prdoc.pr {
        out.push_str(&format!("pr: {pr}\n"));
    }
    out.push_str(&yaml_kv("author", &prdoc.author));
    out.push_str(&format!(
        "status: {}\n",
        serde_yaml::to_string(&prdoc.status)
            .unwrap_or_default()
            .trim()
    ));
    out.push_str("packages:\n");
    for pkg in &prdoc.packages {
        out.push_str(&format!("  - {pkg}\n"));
    }
    out.push_str(&format!("breaking: {}\n", prdoc.breaking));
    if !prdoc.needs_review.is_empty() {
        out.push_str("needs-review:\n");
        for item in &prdoc.needs_review {
            out.push_str(&format!("  - {item}\n"));
        }
    }
    if !prdoc.audience.is_empty() {
        out.push_str("audience:\n");
        for a in &prdoc.audience {
            out.push_str(&format!("  - {}\n", a.as_str()));
        }
    }
    if !prdoc.crates.is_empty() {
        out.push_str("crates:\n");
        for change in &prdoc.crates {
            out.push_str(&format!(
                "  - name: {}\n    bump: {}\n    validate: {}\n",
                change.name,
                change.bump.as_str(),
                change.validate
            ));
        }
    }
    out.push_str("---\n\n");

    out.push_str("## Summary\n\n");
    let (api_additions, api_removals) =
        diff.map(extract_public_api_from_diff).unwrap_or_default();
    let summary_ctx = SummaryContext {
        analysis,
        context,
        public_api_additions: api_additions,
        public_api_removals: api_removals,
    };
    let rich_summary = crate::summary::generate_rich_summary(&summary_ctx);
    let final_summary = if let Some(config) = llm_config {
        #[cfg(feature = "llm")]
        {
            crate::llm::enhance_summary(&rich_summary, &summary_ctx, config)
        }
        #[cfg(not(feature = "llm"))]
        {
            let _ = config;
            rich_summary
        }
    } else {
        rich_summary
    };
    out.push_str(&final_summary);
    if !final_summary.ends_with('\n') {
        out.push('\n');
    }
    out.push('\n');

    out.push_str("## Changes\n");
    out.push_str("### Packages Affected\n");
    for pkg in &prdoc.packages {
        let bump = prdoc
            .crates
            .iter()
            .find(|c| c.name == *pkg)
            .map(|c| c.bump.as_str())
            .unwrap_or("none");
        let desc = describe_package_change(
            pkg,
            &analysis.file_changes,
            bump,
            prdoc.breaking,
        );
        out.push_str(&desc);
    }
    out.push('\n');

    out.push_str("## Agent Instructions\n");
    out.push_str("### Verification\n");
    out.push_str("1. Run `cargo test --workspace` — all tests must pass.\n");
    out.push_str(
        "2. Run `cargo clippy --workspace -- -D warnings` — no warnings.\n",
    );
    out.push_str("3. Run `montrs agent check` — no invariant violations.\n");
    out.push('\n');

    if !prdoc.needs_review.is_empty() {
        out.push_str("### Review Focus\n");
        for item in &prdoc.needs_review {
            out.push_str(&format!(
                "- {}\n",
                match item.as_str() {
                    "architecture" =>
                        "Architecture: verify structural integrity of public \
                         API changes.",
                    "design" =>
                        "Design: confirm new types and traits follow project \
                         conventions.",
                    "agent" =>
                        "Agent: ensure machine-readable metadata is present.",
                    "migration" =>
                        "Migration: validate that breaking changes are \
                         documented.",
                    other => other,
                }
            ));
        }
        out.push('\n');
    } else {
        out.push_str("### Review Focus\n\n");
    }

    out.push_str("## Migration Notes\n\n");
    if prdoc.breaking {
        let breaking_pkgs: Vec<&str> = prdoc
            .crates
            .iter()
            .filter(|c| c.bump == BumpLevel::Major)
            .map(|c| c.name.as_str())
            .collect();
        if !breaking_pkgs.is_empty() {
            out.push_str(&format!(
                "This PR introduces breaking changes to: {}.\n",
                breaking_pkgs.join(", ")
            ));
        }
        out.push_str(
            "Review the public API modifications carefully before merging.\n",
        );
    } else {
        out.push_str("None.\n");
    }

    out
}

fn yaml_kv(key: &str, value: &str) -> String {
    if value.contains('"')
        || value.contains('\n')
        || value.contains(':')
        || value.contains('#')
        || value.contains('@')
    {
        let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
        format!("{key}: \"{escaped}\"\n")
    } else if value.is_empty()
        || value.contains(' ')
        || value.contains('&')
        || value.contains('*')
        || value.contains('!')
        || value.contains('{')
        || value.contains('[')
    {
        format!("{key}: \"{value}\"\n")
    } else {
        format!("{key}: {value}\n")
    }
}

fn derive_title(
    context: Option<&PrContext>,
    analysis: &DiffAnalysis,
) -> String {
    if let Some(ctx) = context
        && !ctx.title.is_empty()
    {
        return ctx.title.clone();
    }

    let cat_str = match &analysis.dominant_category {
        ChangeCategory::NewFeature => "Add",
        ChangeCategory::BugFix => "Fix",
        ChangeCategory::BreakingChange => "Breaking change in",
        ChangeCategory::Refactor => "Refactor",
        ChangeCategory::Documentation => "Document",
        ChangeCategory::Internal => "Update",
    };

    if analysis.packages.is_empty() {
        format!("{cat_str} project")
    } else if analysis.packages.len() == 1 {
        format!("{cat_str} {}", analysis.packages[0])
    } else {
        format!(
            "{cat_str} {} and {} other package(s)",
            analysis.packages[0],
            analysis.packages.len() - 1
        )
    }
}

fn describe_package_change(
    pkg: &str,
    file_changes: &[crate::analyzer::FileChange],
    bump: &str,
    _is_breaking: bool,
) -> String {
    let pkg_files: Vec<&crate::analyzer::FileChange> = file_changes
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

    let total_added: usize = pkg_files.iter().map(|c| c.added_lines).sum();
    let total_removed: usize = pkg_files.iter().map(|c| c.removed_lines).sum();

    let mut details = Vec::new();
    if source_count > 0 {
        details.push(format!("{source_count} source file(s)"));
    }
    if test_count > 0 {
        details.push(format!("{test_count} test file(s)"));
    }

    let detail_str = if details.is_empty() {
        String::new()
    } else {
        format!(" ({})", details.join(", "))
    };

    let line_str = if total_added > 0 || total_removed > 0 {
        format!(" +{total_added}/-{total_removed}")
    } else {
        String::new()
    };

    format!("- **{pkg}** ({bump}):{detail_str}{line_str}\n")
}

fn default_bump_for_category(
    category: &ChangeCategory,
    is_breaking: bool,
) -> BumpLevel {
    if is_breaking {
        return BumpLevel::Major;
    }
    match category {
        ChangeCategory::NewFeature => BumpLevel::Minor,
        ChangeCategory::BugFix => BumpLevel::Patch,
        ChangeCategory::BreakingChange => BumpLevel::Major,
        ChangeCategory::Refactor => BumpLevel::Patch,
        ChangeCategory::Documentation => BumpLevel::None,
        ChangeCategory::Internal => BumpLevel::None,
    }
}

fn derive_needs_review(
    is_breaking: bool,
    category: &ChangeCategory,
) -> Vec<String> {
    let mut review = Vec::new();
    if is_breaking {
        review.push("architecture".to_string());
        review.push("migration".to_string());
    }
    match category {
        ChangeCategory::NewFeature => {
            review.push("design".to_string());
        }
        ChangeCategory::BreakingChange => {
            if !is_breaking {
                review.push("architecture".to_string());
            }
        }
        _ => {}
    }
    review
}

fn merge_categories(
    rule: &[ChangeCategory],
    embed: &[ChangeCategory],
) -> Vec<ChangeCategory> {
    let mut merged = rule.to_vec();
    for (i, embed_cat) in embed.iter().enumerate() {
        if i < merged.len() {
            if *embed_cat == ChangeCategory::BreakingChange
                && merged[i] != ChangeCategory::BreakingChange
            {
                merged[i] = ChangeCategory::BreakingChange;
            }
        } else {
            merged.push(embed_cat.clone());
        }
    }
    merged
}

fn most_frequent_category(
    categories: &[ChangeCategory],
) -> Option<&ChangeCategory> {
    use std::collections::HashMap;
    let mut counts: HashMap<&ChangeCategory, usize> = HashMap::new();
    for cat in categories {
        *counts.entry(cat).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|(_, c)| *c).map(|(c, _)| c)
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

use crate::analyzer::FileCategory;
