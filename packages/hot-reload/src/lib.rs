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

//! Server-side generation of Leptos `view!` hot-reload patches.
//!
//! [`leptos_hot_reload`] parses `view!` macros out of `.rs` source and diffs
//! them, producing JSON patches the browser can apply to the live DOM without
//! recompiling Rust. This crate wraps that so the MontRS dev server can send
//! patches the instant a file changes.
//!
//! It also provides a conservative **view-only** classifier: a file is treated
//! as view-only when everything *outside* its `view!` macros is byte-identical,
//! which lets the dev server skip a full cargo rebuild for markup-only edits.
//! When in doubt it reports "not view-only", so a needed compile is never
//! skipped.

use leptos_hot_reload::ViewMacros;
pub use leptos_hot_reload::diff::Patches;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};
use syn::{spanned::Spanned, visit::Visit};
use walkdir::WalkDir;

/// Per-file hot-reload state: the parsed `view!` baseline and a hash of the
/// file's non-view "skeleton".
pub struct ViewPatcher {
    baselines: HashMap<PathBuf, ViewMacros>,
    skeletons: HashMap<PathBuf, u64>,
    /// Root the compiler used to derive stable ids (cargo's workspace root).
    /// Ids are rewritten to match it because we key files by absolute path.
    workspace_root: Option<PathBuf>,
}

impl ViewPatcher {
    /// Snapshot the `view!` macros and skeleton hashes of every `.rs` file
    /// under `roots`. Files that fail to parse are skipped (edits to them fall
    /// back to a full rebuild).
    pub fn new(roots: &[PathBuf], workspace_root: Option<PathBuf>) -> Self {
        let mut baselines = HashMap::new();
        let mut skeletons = HashMap::new();

        for root in roots {
            for entry in WalkDir::new(root).into_iter().flatten() {
                if !entry.file_type().is_file() {
                    continue;
                }
                if entry.path().extension().and_then(|e| e.to_str())
                    != Some("rs")
                {
                    continue;
                }
                let Some(canon) = canonical(entry.path()) else {
                    continue;
                };

                let vm = ViewMacros::new();
                if vm.update_from_paths(std::slice::from_ref(&canon)).is_ok() {
                    baselines.insert(canon.clone(), vm);
                }
                if let Ok(src) = std::fs::read_to_string(&canon) {
                    skeletons.insert(canon, skeleton_hash(&src));
                }
            }
        }

        Self {
            baselines,
            skeletons,
            // Canonicalize so `strip_prefix` matches the canonical file keys.
            workspace_root: workspace_root.and_then(|p| canonical(&p)),
        }
    }

    /// Produce the view patches for a changed file, if any.
    pub fn patch(&self, path: &Path) -> Option<Patches> {
        let canon = canonical(path)?;
        let vm = self.baselines.get(&canon)?;
        let utf8 = camino::Utf8PathBuf::try_from(canon.clone()).ok()?;
        let patches = vm.patch(&utf8).ok().flatten()?;
        Some(self.rewrite_ids(canon, patches))
    }

    /// The compiler derives ids from the workspace-relative path
    /// (`apps/website/app/src/main.rs` → `apps-website-app-src-main.rs-36`),
    /// but we key files by absolute path, so each id is rebuilt from the
    /// relative path to match the markers in the DOM.
    fn rewrite_ids(&self, canon: PathBuf, patches: Patches) -> Patches {
        let Some(root) = &self.workspace_root else {
            return patches;
        };
        let Ok(rel) = canon.strip_prefix(root) else {
            return patches;
        };
        let token = rel.to_string_lossy().replace(['/', '\\'], "-");

        Patches(
            patches
                .0
                .into_iter()
                .map(|(id, patches)| {
                    let new_id = match id.rsplit_once('-') {
                        Some((_, line)) => format!("{token}-{line}"),
                        None => id,
                    };
                    (new_id, patches)
                })
                .collect(),
        )
    }

    /// Whether the file changed only inside its `view!` macros. Updates the
    /// stored skeleton hash, so call once per change.
    pub fn is_view_only(&mut self, path: &Path) -> bool {
        let Some(canon) = canonical(path) else {
            return false;
        };
        let Ok(src) = std::fs::read_to_string(&canon) else {
            return false;
        };
        let new_hash = skeleton_hash(&src);
        let previous = self.skeletons.insert(canon, new_hash);
        previous == Some(new_hash)
    }
}

/// Canonical form used as a stable key for both the baseline map and notify's
/// event paths (which may be relative or `\\?\`-prefixed on Windows).
fn canonical(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

/// Hash of a source file with every `view!` macro's lines blanked out.
///
/// Two versions of a file hash the same when only their `view!` content
/// differs — i.e. the change is markup-only and can be hot-patched.
fn skeleton_hash(src: &str) -> u64 {
    let lines: Vec<&str> = src.lines().collect();
    let mut blank = vec![false; lines.len()];

    if let Ok(ast) = syn::parse_file(src) {
        let mut visitor = ViewSpanVisitor::default();
        visitor.visit_file(&ast);
        for (start, end) in visitor.spans {
            for line in start..=end {
                if line >= 1 && line <= blank.len() {
                    blank[line - 1] = true;
                }
            }
        }
    }

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for (i, line) in lines.iter().enumerate() {
        if blank.get(i).copied().unwrap_or(false) {
            continue;
        }
        line.hash(&mut hasher);
    }
    hasher.finish()
}

/// Collects the line spans of every `view!` macro invocation.
#[derive(Default)]
struct ViewSpanVisitor {
    spans: Vec<(usize, usize)>,
}

impl<'ast> Visit<'ast> for ViewSpanVisitor {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let is_view = mac
            .path
            .segments
            .last()
            .is_some_and(|seg| seg.ident == "view");
        if is_view {
            let start = mac.span().start().line;
            let end = mac.span().end().line;
            if start >= 1 {
                self.spans.push((start, end.max(start)));
            }
        }
        syn::visit::visit_macro(self, mac);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markup_only_edit_keeps_skeleton() {
        let a = r#"
#[component]
fn Card() -> impl IntoView {
    let x = 1;
    view! { <div class="a">"hello"</div> }
}
"#;
        let b = r#"
#[component]
fn Card() -> impl IntoView {
    let x = 1;
    view! { <div class="b">"world"</div> }
}
"#;
        assert_eq!(skeleton_hash(a), skeleton_hash(b));
    }

    #[test]
    fn code_edit_changes_skeleton() {
        let a = r#"
fn Card() -> impl IntoView {
    let x = 1;
    view! { <div>"hi"</div> }
}
"#;
        let b = r#"
fn Card() -> impl IntoView {
    let x = 2;
    view! { <div>"hi"</div> }
}
"#;
        assert_ne!(skeleton_hash(a), skeleton_hash(b));
    }

    #[test]
    fn patches_are_id_rewritten_to_workspace_relative_form() {
        let root = std::env::temp_dir().join(format!(
            "montrs-hr-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let src = root.join("app").join("src");
        std::fs::create_dir_all(&src).unwrap();
        let file = src.join("card.rs");

        std::fs::write(
            &file,
            "fn c() -> impl IntoView {\n    let x = 1;\n    view! { <div \
             class=\"a\">\"hi\"</div> }\n}\n",
        )
        .unwrap();

        let mut patcher = super::ViewPatcher::new(
            std::slice::from_ref(&src),
            Some(root.clone()),
        );
        assert!(patcher.is_view_only(&file));

        std::fs::write(
            &file,
            "fn c() -> impl IntoView {\n    let x = 1;\n    view! { <div \
             class=\"b\">\"bye\"</div> }\n}\n",
        )
        .unwrap();

        let patches = patcher.patch(&file).expect("view patch");
        assert_eq!(patches.0.len(), 1);
        // Compiler-side ids look like `app-src-card.rs-3` (view starts line 3).
        assert_eq!(patches.0[0].0, "app-src-card.rs-3");
        assert!(patcher.is_view_only(&file));

        // A logic edit outside the view! macro must not be view-only.
        std::fs::write(
            &file,
            "fn c() -> impl IntoView {\n    let x = 2;\n    view! { <div \
             class=\"b\">\"bye\"</div> }\n}\n",
        )
        .unwrap();
        assert!(!patcher.is_view_only(&file));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn view_line_shift_is_conservative() {
        // Inserting a line above the view! shifts its line numbers, which
        // changes the skeleton hash (blanked lines move). This is the safe
        // direction: the classifier falls back to a rebuild.
        let a = "fn a() -> impl IntoView { view! { <div></div> } }\n";
        let b = "// note\nfn a() -> impl IntoView { view! { <div></div> } }\n";
        assert_ne!(skeleton_hash(a), skeleton_hash(b));
    }
}
