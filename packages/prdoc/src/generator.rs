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
