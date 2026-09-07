// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! `montrs add` — shadcn-style registry for MontRS apps.
//!
//! Copies components / icons / themes / libraries into an existing MontRS app
//! created by `montrs new`. Components are embedded from the canonical
//! montrs-ui sources at build time (see `add_registry.rs`).

use super::{add_blocks, add_registry};
use anyhow::{Context, Result, bail};
use console::style;
use std::{
    fs,
    path::{Path, PathBuf},
};

const THEME_NAMES: &[&str] =
    &["orange", "rose", "emerald", "sky", "violet", "zinc"];

pub async fn run(
    items: Vec<String>,
    library: Option<String>,
    icons: bool,
    theme: Option<String>,
    icon: Option<String>,
    list: bool,
    path: &str,
) -> Result<()> {
    if list {
        list_available();
        return Ok(());
    }

    let root = PathBuf::from(path);
    if !root.exists() {
        bail!("Directory `{}` does not exist", root.display());
    }
    let src = app_src(&root)?;
    let components_ui = src.join("components/ui");
    let blocks_dir = src.join("components/blocks");
    let icons_dir = src.join("components/icons");
    let cargo_toml = if root.join("app/Cargo.toml").exists() {
        root.join("app/Cargo.toml")
    } else {
        root.join("Cargo.toml")
    };
    let style_css = root.join("style/main.css");

    let mut wrote_any = false;

    if let Some(lib) = &library {
        ensure_dep(&cargo_toml, lib)?;
        println!(
            "  {} Added `{}` to your dependencies",
            style("✓").green().bold(),
            lib
        );
        wrote_any = true;
    }

    if icons {
        ensure_dep(&cargo_toml, "montrs-icons")?;
        println!(
            "  {} Added `montrs-icons` to your dependencies",
            style("✓").green().bold()
        );
        wrote_any = true;
    }

    if let Some(t) = &theme {
        add_theme(&style_css, t)?;
        wrote_any = true;
    }

    if let Some(g) = &icon {
        add_icon(&icons_dir, &src, g)?;
        wrote_any = true;
    }

    for item in &items {
        let item = item.as_str();
        if item.contains('/') {
            add_icon(&icons_dir, &src, item)?;
        } else if THEME_NAMES.contains(&item) {
            add_theme(&style_css, item)?;
        } else if add_registry::COMPONENTS
            .iter()
            .any(|(k, _)| *k == item.replace('-', "_").as_str())
        {
            add_component(&components_ui, &src, &cargo_toml, item)?;
        } else if add_blocks::BLOCKS.iter().any(|(k, _)| *k == item) {
            add_block(&blocks_dir, &src, &cargo_toml, item)?;
        } else {
            bail!(
                "Could not resolve `{item}`. Is it a component, block, theme, \
                 or icon (`collection/name`)? Run `montrs add --list` to see \
                 what you can add."
            );
        }
        wrote_any = true;
    }

    if !wrote_any {
        print_usage();
    } else {
        println!(
            "\n{} Run `cargo build` to verify, then `montrs serve` to preview.",
            style("✨").green().bold()
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Component registry
// ---------------------------------------------------------------------------

fn add_component(
    dir: &Path,
    src: &Path,
    cargo_toml: &Path,
    name: &str,
) -> Result<()> {
    let key = name.replace('-', "_");
    let (_, source) = add_registry::COMPONENTS
        .iter()
        .find(|(k, _)| *k == key.as_str())
        .with_context(|| {
            format!(
                "Unknown component `{name}`. Run `montrs add --list` to see \
                 everything available."
            )
        })?;

    let rewritten = source.replace("crate::", "montrs_ui::");
    fs::create_dir_all(dir).context("creating components/ui directory")?;
    let dest = dir.join(format!("{key}.rs"));
    fs::write(&dest, &rewritten)
        .with_context(|| format!("failed to write {}", dest.display()))?;

    // Auto-register modules we own (components/mod.rs, components/ui/mod.rs).
    let components_mod = dir.parent().unwrap().join("mod.rs");
    ensure_mod_line(&components_mod, "pub mod ui;")?;
    ensure_mod_line(&dir.join("mod.rs"), &format!("pub mod {key};"))?;

    // The app crate still needs `pub mod components;` in its lib/main.
    let lib_rs = src.join("lib.rs");
    let lib_text = fs::read_to_string(&lib_rs).unwrap_or_default();
    if !lib_text.contains("mod components;") {
        println!(
            "  {} Add `pub mod components;` to your crate root ({}).",
            style("!").yellow().bold(),
            lib_rs.display()
        );
    }

    ensure_dep(cargo_toml, "montrs-ui")?;
    println!(
        "  {} Wrote {} (uses the montrs-ui dependency)",
        style("✓").green().bold(),
        dest.display()
    );
    println!(
        "  {} Import it with `use components::ui::{}::{}`.",
        style("→").cyan().bold(),
        key,
        pascal_case(&key)
    );
    Ok(())
}

fn add_block(
    dir: &Path,
    src: &Path,
    cargo_toml: &Path,
    name: &str,
) -> Result<()> {
    let (_, source) = add_blocks::BLOCKS
        .iter()
        .find(|(k, _)| *k == name)
        .with_context(|| {
            format!(
                "Unknown block `{name}`. Run `montrs add --list` to see every \
                 available block."
            )
        })?;

    fs::create_dir_all(dir).context("creating components/blocks directory")?;
    let dest = dir.join(format!("{name}.rs"));
    fs::write(&dest, *source)
        .with_context(|| format!("failed to write {}", dest.display()))?;

    let components_mod = dir.parent().unwrap().join("mod.rs");
    ensure_mod_line(&components_mod, "pub mod blocks;")?;
    ensure_mod_line(&dir.join("mod.rs"), &format!("pub mod {name};"))?;

    let lib_rs = src.join("lib.rs");
    let lib_text = fs::read_to_string(&lib_rs).unwrap_or_default();
    if !lib_text.contains("mod components;") {
        println!(
            "  {} Add `pub mod components;` to your crate root ({}).",
            style("!").yellow().bold(),
            lib_rs.display()
        );
    }

    ensure_dep(cargo_toml, "montrs-ui")?;
    ensure_dep(cargo_toml, "montrs-icons")?;
    println!(
        "  {} Wrote {} (uses montrs-ui + montrs-icons)",
        style("✓").green().bold(),
        dest.display()
    );
    println!(
        "  {} Import it with `use components::blocks::{name}::...`.",
        style("→").cyan().bold()
    );
    Ok(())
}

fn ensure_mod_line(mod_path: &Path, line: &str) -> Result<()> {
    if !mod_path.exists() {
        fs::write(mod_path, format!("{line}\n"))
            .context("creating module file")?;
        return Ok(());
    }
    let text = fs::read_to_string(mod_path)?;
    if !text.lines().any(|l| l.trim() == line) {
        fs::write(mod_path, format!("{text}{line}\n"))
            .context("appending module declaration")?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Icons
// ---------------------------------------------------------------------------

fn add_icon(icons_dir: &Path, src: &Path, glyph: &str) -> Result<()> {
    // Accept `<collection>/<name>` (e.g. `lucide/home`, `tabler/arrow-right`,
    // `simple-icons/github`). The collection defaults to `lucide`.
    let (collection, name) = match glyph.split_once('/') {
        Some((c, n)) => (c, n),
        None => ("lucide", glyph),
    };
    let valid_collections = [
        "lucide",
        "radix",
        "tabler",
        "iconoir",
        "phosphor",
        "mdi",
        "bootstrap",
        "simple-icons",
        "cryptocurrency",
    ];
    if !valid_collections.contains(&collection) {
        bail!(
            "Unknown icon collection `{collection}`. Use \
             `<collection>/<name>`, e.g. `montrs add --icon lucide/home` or \
             `montrs add --icon tabler/arrow-right`.\nAvailable collections: \
             {}",
            valid_collections.join(", ")
        );
    }

    let pascal = pascal_case(name);
    let file = format!(
        "{}_{}",
        collection.replace('-', "_"),
        name.replace('-', "_")
    );
    let collection_pascal = pascal_case(collection);
    let fn_name = format!("{collection_pascal}{pascal}");
    let feature = format!("col-{collection}");

    let component = if collection == "lucide" {
        format!(
            r#"// Generated by `montrs add --icon {glyph}`
use leptos::prelude::*;
use montrs_icons::*;

/// The `{name}` icon as a standalone component.
#[component]
pub fn {fn_name}(#[prop(into, optional)] class: String) -> impl IntoView {{
    view! {{
        <Icon glyph=Glyph::{pascal} class=class />
    }}
}}
"#
        )
    } else {
        format!(
            r#"// Generated by `montrs add --icon {glyph}`
use leptos::prelude::*;
use montrs_icons::*;

/// The `{collection}/{name}` icon as a standalone component.
/// Requires the `{feature}` feature on your montrs-icons dependency.
#[component]
pub fn {fn_name}(#[prop(into, optional)] class: String) -> impl IntoView {{
    view! {{
        {{
            Collection::{collection_pascal}.glyph("{name}").map(|g| view! {{
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox={{g.viewbox}}
                    fill={{g.fill}}
                    stroke={{g.stroke}}
                    stroke-width=move || if g.stroke == "none" {{ String::new() }} else {{ "1.5".to_string() }}
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    inner_html={{g.svg}}
                    class=class
                />
            }})
        }}
    }}
}}
"#
        )
    };

    fs::create_dir_all(icons_dir).context("creating components/icons")?;
    let dest = icons_dir.join(format!("{file}.rs"));
    fs::write(&dest, &component)
        .with_context(|| format!("failed to write {}", dest.display()))?;

    let components_mod = icons_dir.parent().unwrap().join("mod.rs");
    ensure_mod_line(&components_mod, "pub mod icons;")?;
    ensure_mod_line(&icons_dir.join("mod.rs"), &format!("pub mod {file};"))?;

    let lib_rs = src.join("lib.rs");
    let lib_text = fs::read_to_string(&lib_rs).unwrap_or_default();
    if !lib_text.contains("mod components;") {
        println!(
            "  {} Add `pub mod components;` to your crate root ({}).",
            style("!").yellow().bold(),
            lib_rs.display()
        );
    }

    println!("  {} Wrote {}", style("✓").green().bold(), dest.display());
    if collection != "lucide" {
        println!(
            "  {} Requires `montrs-icons` with the `{}` feature enabled.",
            style("!").yellow().bold(),
            feature
        );
    }
    println!(
        "  {} Import it with `use components::icons::{file}::{fn_name}`.",
        style("→").cyan().bold()
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Themes
// ---------------------------------------------------------------------------

fn add_theme(css: &Path, name: &str) -> Result<()> {
    let block = theme_block(name).with_context(|| {
        format!(
            "Unknown theme `{name}`. Try orange, rose, emerald, sky, violet, \
             zinc."
        )
    })?;
    let mut text = fs::read_to_string(css).unwrap_or_default();
    if text.contains(&format!("/* montrs add theme:{name} */")) {
        println!(
            "  {} Theme `{name}` already applied to {}",
            style("✓").green().bold(),
            css.display()
        );
        return Ok(());
    }
    text.push('\n');
    text.push_str(&block);
    fs::write(css, &text).context("failed to write style/main.css")?;
    println!(
        "  {} Appended theme `{name}` to {}",
        style("✓").green().bold(),
        css.display()
    );
    Ok(())
}

fn theme_block(name: &str) -> Option<String> {
    // hsl() values tuned for both light (:root) and dark (.dark) modes.
    let (light, dark) = match name {
        "orange" => ("24 96% 53%", "24 96% 55%"),
        "rose" => ("350 89% 60%", "350 89% 62%"),
        "emerald" => ("152 76% 40%", "152 76% 44%"),
        "sky" => ("199 89% 48%", "199 89% 52%"),
        "violet" => ("262 83% 58%", "262 83% 62%"),
        "zinc" => ("240 5% 34%", "240 5% 46%"),
        _ => return None,
    };
    Some(format!(
        "/* montrs add theme:{name} */\n:root {{ --primary: {light}; --ring: \
         {light}; }}\n.dark {{ --primary: {dark}; --ring: {dark}; }}\n"
    ))
}

// ---------------------------------------------------------------------------
// Dependencies
// ---------------------------------------------------------------------------

fn ensure_dep(cargo_toml: &Path, crate_name: &str) -> Result<()> {
    let text = fs::read_to_string(cargo_toml).unwrap_or_default();
    let dep_key = crate_name.replace('-', "_");
    // TOML normalizes dashes and underscores, so match either form.
    let present = text.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with(&format!("{crate_name} ="))
            || t.starts_with(&format!("{dep_key} ="))
    });
    if present {
        return Ok(()); // already a dependency
    }
    let Some(idx) = text.find("[dependencies]") else {
        bail!(
            "Could not find [dependencies] in {} — add `{crate_name}` manually",
            cargo_toml.display()
        );
    };
    let segment = crate_name.strip_prefix("montrs-").unwrap_or(crate_name);
    // Locate the MontRS workspace root walking up from the Cargo.toml.
    let mut dir = cargo_toml.parent().unwrap();
    let mut workspace_pkgs = None;
    while let Some(d) = dir.parent() {
        if d.join("packages/ui").exists() {
            workspace_pkgs = Some(d.to_path_buf());
            break;
        }
        dir = d;
    }
    let line = match &workspace_pkgs {
        Some(root) => {
            let rel = relative_path(
                cargo_toml.parent().unwrap(),
                &root.join("packages").join(segment),
            );
            format!("{crate_name} = {{ path = \"{rel}\" }}")
        }
        None => {
            // Not inside a montrs workspace — fall back to crates.io.
            return add_crates_io(cargo_toml, crate_name);
        }
    };
    let insert_at = text[idx..]
        .find('\n')
        .map(|off| idx + off + 1)
        .unwrap_or(text.len());
    let mut out = text.clone();
    out.insert_str(insert_at, &format!("{line}\n"));
    fs::write(cargo_toml, &out).context("failed to update Cargo.toml")?;
    println!(
        "  {} Added `{crate_name}` to {}",
        style("✓").green().bold(),
        cargo_toml.display()
    );
    Ok(())
}

fn add_crates_io(cargo_toml: &Path, crate_name: &str) -> Result<()> {
    let status = std::process::Command::new("cargo")
        .arg("add")
        .arg(crate_name)
        .current_dir(cargo_toml.parent().unwrap())
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        _ => {
            println!(
                "  {} Could not auto-add `{crate_name}` — add it to {} \
                 manually.",
                style("!").yellow().bold(),
                cargo_toml.display()
            );
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Listing
// ---------------------------------------------------------------------------

fn list_available() {
    println!("{} montrs add — registry", style("✦").bold());
    println!(
        "\n{} Components ({} available):",
        style("Components").green().bold(),
        add_registry::COMPONENTS.len()
    );
    let names = add_registry::COMPONENTS
        .iter()
        .map(|(n, _)| *n)
        .collect::<Vec<_>>();
    print_columns(&names);

    println!(
        "\n{} Blocks ({} available):",
        style("Blocks").green().bold(),
        add_blocks::BLOCKS.len()
    );
    let block_names = add_blocks::BLOCKS
        .iter()
        .map(|(n, _)| *n)
        .collect::<Vec<_>>();
    print_columns(&block_names);

    println!("\n{} Themes:", style("Themes").green().bold());
    print_columns(&["orange", "rose", "emerald", "sky", "violet", "zinc"]);

    println!(
        "\n{} Icons (collection/name):",
        style("Icons").green().bold()
    );
    println!(
        "  montrs add --icon lucide/home\n  montrs add --icon \
         tabler/arrow-right\n  montrs add --icon simple-icons/github"
    );

    println!("\n{} Libraries:", style("Libraries").green().bold());
    println!(
        "  montrs add --icons              (montrs-icons dependency)\n  \
         montrs add --library montrs-content"
    );

    println!("\nUsage:");
    println!("  montrs add button card");
    println!("  montrs add --theme orange");
    println!("  montrs add --icon lucide/home");
    println!("  montrs add --icons");
}

fn print_columns(items: &[&str]) {
    for chunk in items.chunks(4) {
        println!(
            "  {}",
            chunk.iter().map(|c| format!("{c:<18}")).collect::<String>()
        );
    }
}

fn print_usage() {
    println!(
        "{} Specify something to add. Examples:\n  montrs add button card\n  \
         montrs add --theme orange\n  montrs add --icon lucide/home\n  montrs \
         add --icons\n  montrs add --library montrs-content\n  montrs add \
         --list",
        style("? Nothing to add.").yellow().bold()
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn app_src(root: &Path) -> Result<PathBuf> {
    for candidate in [root.join("app/src"), root.join("src")] {
        if candidate.join("lib.rs").exists() {
            return Ok(candidate);
        }
    }
    bail!(
        "`{}` does not look like a MontRS app (no app/src/lib.rs or \
         src/lib.rs). Create one with `montrs new <name>`.",
        root.display()
    )
}

fn pascal_case(s: &str) -> String {
    s.split(['-', '_'])
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

/// Minimal relative path from `from_dir` to `to` (no pathdiff dependency).
fn relative_path(from_dir: &Path, to: &Path) -> String {
    use std::path::Component;
    let from = from_dir
        .canonicalize()
        .unwrap_or_else(|_| from_dir.to_path_buf());
    let to = to.canonicalize().unwrap_or_else(|_| to.to_path_buf());
    let from_comps: Vec<Component> = from.components().collect();
    let to_comps: Vec<Component> = to.components().collect();
    let common = from_comps
        .iter()
        .zip(to_comps.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut out = String::new();
    for _ in common..from_comps.len() {
        out.push_str("../");
    }
    let rel: Vec<String> = to_comps[common..]
        .iter()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    out.push_str(&rel.join("/"));
    if out.is_empty() { ".".to_string() } else { out }
}
