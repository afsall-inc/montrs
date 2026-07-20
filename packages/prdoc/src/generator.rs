use crate::{
    analyzer::{DiffAnalysis, PrContext},
    types::{Audience, BumpLevel, CrateChange, DocSection, PrDoc},
};

pub fn generate_prdoc(
    analysis: &DiffAnalysis,
    context: Option<&PrContext>,
) -> PrDoc {
    let title = context
        .map(|c| c.title.clone())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| "...".to_string());

    let author = context.map(|c| c.author.clone());

    let pr = context.map(|c| c.pr_number);

    let audience = infer_primary_audience(&analysis.packages);

    let description = context
        .and_then(|c| c.body.clone())
        .filter(|b| !b.is_empty())
        .unwrap_or_else(|| "...".to_string());

    let doc = vec![DocSection {
        audience,
        description,
        title: None,
    }];

    let crates = if analysis.crate_changes.is_empty() {
        analysis
            .packages
            .iter()
            .map(|pkg| CrateChange {
                name: pkg.clone(),
                bump: BumpLevel::Minor,
                validate: true,
                note: None,
            })
            .collect()
    } else {
        analysis
            .crate_changes
            .iter()
            .map(|c| CrateChange {
                name: c.name.clone(),
                bump: c.bump.clone(),
                validate: c.validate,
                note: None,
            })
            .collect()
    };

    PrDoc {
        title,
        author,
        pr,
        doc,
        crates,
        migrations: None,
        host_functions: None,
    }
}

fn infer_primary_audience(packages: &[String]) -> Audience {
    for pkg in packages {
        match pkg.as_str() {
            "core" | "cli" | "agent" | "fmt" | "bench" | "utils" | "runner" => {
                return Audience::FrameworkDev
            }
            "orm" | "validator" | "test" => return Audience::AppDev,
            _ => {}
        }
    }
    Audience::AppDev
}

pub fn render_prdoc(prdoc: &PrDoc) -> String {
    let mut out = String::new();

    out.push_str("# PRDoc: Pull Request Documentation\n");
    out.push_str("# Fill in the ... placeholders with meaningful content.\n");
    out.push_str("# See docs/contributor/prdoc.md for schema details.\n\n");

    out.push_str("---\n");
    out.push_str(&format!("title: {}\n", escape_yaml_string(&prdoc.title)));

    if let Some(ref author) = prdoc.author {
        out.push_str(&format!("author: {}\n", author));
    }

    if let Some(pr) = prdoc.pr {
        out.push_str(&format!("pr: {}\n", pr));
    }

    out.push_str("\ndoc:\n");
    for doc_section in &prdoc.doc {
        out.push_str(&format!(
            "  - audience: {}\n",
            doc_section.audience.as_str()
        ));
        out.push_str("    description: |\n");
        for line in doc_section.description.lines() {
            out.push_str(&format!("      {}\n", line));
        }
    }

    out.push_str("\ncrates:\n");
    for crate_change in &prdoc.crates {
        out.push_str(&format!("  - name: {}\n", crate_change.name));
        out.push_str(&format!("    bump: {}\n", crate_change.bump.as_str()));
        if !crate_change.validate {
            out.push_str("    validate: false\n");
        }
        if let Some(ref note) = crate_change.note {
            out.push_str(&format!("    note: {}\n", escape_yaml_string(note)));
        }
    }

    if let Some(ref migrations) = prdoc.migrations {
        if !migrations.db.is_empty() || !migrations.runtime.is_empty() {
            out.push_str("\nmigrations:\n");
            if !migrations.db.is_empty() {
                out.push_str("  db:\n");
                for mig in &migrations.db {
                    out.push_str(&format!("    - name: {}\n", mig.name));
                    out.push_str(&format!(
                        "      description: {}\n",
                        escape_yaml_string(&mig.description)
                    ));
                }
            }
            if !migrations.runtime.is_empty() {
                out.push_str("  runtime:\n");
                for mig in &migrations.runtime {
                    out.push_str("    - description: |\n");
                    for line in mig.description.lines() {
                        out.push_str(&format!("        {}\n", line));
                    }
                    if let Some(ref reference) = mig.reference {
                        out.push_str(&format!("        reference: {}\n", reference));
                    }
                }
            }
        }
    }

    if let Some(ref host_functions) = prdoc.host_functions {
        if !host_functions.is_empty() {
            out.push_str("\nhost_functions:\n");
            for hf in host_functions {
                out.push_str(&format!("  - name: {}\n", hf.name));
                out.push_str(&format!(
                    "    description: {}\n",
                    escape_yaml_string(&hf.description)
                ));
                if let Some(ref notes) = hf.notes {
                    out.push_str(&format!("    notes: {}\n", escape_yaml_string(notes)));
                }
            }
        }
    }

    out.push_str("---\n");

    out
}

fn escape_yaml_string(s: &str) -> String {
    if s.contains(':')
        || s.contains('\n')
        || s.contains('"')
        || s.contains('#')
        || s.is_empty()
    {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

pub fn create_skeleton(pr_number: Option<u64>, title: Option<String>) -> PrDoc {
    PrDoc {
        title: title.unwrap_or_else(|| "...".to_string()),
        author: None,
        pr: pr_number,
        doc: vec![DocSection {
            audience: Audience::AppDev,
            description: "...".to_string(),
            title: None,
        }],
        crates: vec![],
        migrations: None,
        host_functions: None,
    }
}
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
    let moved_items = analysis.moved_items.clone();
    let summary_ctx = SummaryContext {
        analysis,
        context,
        public_api_additions: api_additions,
        public_api_removals: api_removals,
        moved_items,
    };
    let rich_summary = crate::summary::generate_rich_summary(&summary_ctx);
    let final_summary = if use_llm {
        #[cfg(feature = "llm")]
        {
            let root = crate::config::find_project_root()
                .unwrap_or_else(std::path::PathBuf::new);
            let config = crate::config::load_config(&root);
            if let Some(cfg) = config.to_llm_config() {
                crate::llm::enhance_summary(&rich_summary, &summary_ctx, &cfg)
            } else {
                rich_summary
            }
        }
        #[cfg(not(feature = "llm"))]
        {
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
        if let Some(ctx) = context
            && let Some(body) = &ctx.body
        {
            let migration_hints = extract_migration_hints(body);
            if !migration_hints.is_empty() {
                out.push_str(&migration_hints);
            }
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

fn extract_migration_hints(body: &str) -> String {
    let lower = body.to_lowercase();
    let section_start = lower
        .find("migration")
        .or_else(|| lower.find("breaking change"))
        .or_else(|| lower.find("## migration"));
    match section_start {
        Some(pos) => {
            let snippet = &body[pos..];
            let first_para_end = snippet.find("\n\n").unwrap_or(snippet.len());
            let text = snippet[..first_para_end]
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect::<Vec<_>>()
                .join("\n");
            if text.is_empty() {
                String::new()
            } else {
                format!("From PR description:\n{text}\n")
            }
        }
        None => String::new(),
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
