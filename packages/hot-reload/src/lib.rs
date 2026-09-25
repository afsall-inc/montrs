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
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};
use syn::visit_mut::VisitMut;
use walkdir::WalkDir;

/// Per-file hot-reload state: the parsed `view!` baseline and a multiset of the
/// file's non-view tokens (its "skeleton").
pub struct ViewPatcher {
    baselines: HashMap<PathBuf, ViewMacros>,
    skeletons: HashMap<PathBuf, BTreeMap<String, usize>>,
    /// Root the compiler used to derive stable ids (cargo's workspace root).
    /// Ids are rewritten to match it because we key files by absolute path.
    workspace_root: Option<PathBuf>,
}

impl ViewPatcher {
    /// Snapshot the `view!` macros and skeletons of every `.rs` file under
    /// `roots`. Files that fail to parse are skipped (edits to them fall back
    /// to a full rebuild).
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
                    skeletons.insert(canon, skeleton_tokens(&src));
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

    /// Whether the change can be hot-patched without recompiling.
    ///
    /// True when the file's non-`view!` tokens are unchanged (markup-only
    /// edits, comment/formatting changes), or when the only non-`view!` change
    /// is a *removal* — deleting code cannot introduce new behaviour, and any
    /// visible effect is already carried by the `view!` patch. Additions or
    /// in-place edits to non-view code return false and trigger a rebuild.
    ///
    /// Updates the stored skeleton, so call once per change.
    pub fn is_view_only(&mut self, path: &Path) -> bool {
        let Some(canon) = canonical(path) else {
            return false;
        };
        let Ok(src) = std::fs::read_to_string(&canon) else {
            return false;
        };
        let new = skeleton_tokens(&src);
        let previous = self.skeletons.insert(canon, new.clone());
        match previous {
            Some(old) => old == new || is_token_subset(&new, &old),
            None => false,
        }
    }
}

/// Whether every token in `new` occurs at least as often in `old` — i.e. the
/// change added no new code (it may only have removed some).
fn is_token_subset(
    new: &BTreeMap<String, usize>,
    old: &BTreeMap<String, usize>,
) -> bool {
    new.iter()
        .all(|(token, count)| old.get(token).copied().unwrap_or(0) >= *count)
}

/// Canonical form used as a stable key for both the baseline map and notify's
/// event paths (which may be relative or `\\?\`-prefixed on Windows).
fn canonical(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

/// Multiset of tokens in a source file's AST with every `view!` macro stripped.
///
/// Because this uses the parsed Rust AST rather than raw text lines:
/// - Whitespace and indentation changes do NOT change the token counts
/// - Comments (which rustc does not see) do NOT change the token counts
/// - Adding/removing/modifying markup inside `view!` does NOT change the token counts
/// - Renumbering lines inside a `view!` does NOT change the token counts
/// - Only non-`view!` Rust code changes will affect the token bag
fn skeleton_tokens(src: &str) -> BTreeMap<String, usize> {
    let Ok(mut ast) = syn::parse_file(src) else {
        // Unparseable file: count raw words as fallback
        let mut bag = BTreeMap::new();
        for word in src.split_whitespace() {
            *bag.entry(word.to_string()).or_insert(0) += 1;
        }
        return bag;
    };

    struct StripView;
    impl VisitMut for StripView {
        fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
            let is_view = mac
                .path
                .segments
                .last()
                .is_some_and(|seg| seg.ident == "view");
            if is_view {
                mac.tokens = proc_macro2::TokenStream::new();
            }
            syn::visit_mut::visit_macro_mut(self, mac);
        }
    }

    StripView.visit_file_mut(&mut ast);

    let ts = quote::quote!(#ast);
    let mut bag = BTreeMap::new();
    for token in ts {
        *bag.entry(token.to_string()).or_insert(0) += 1;
    }
    bag
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
        assert_eq!(skeleton_tokens(a), skeleton_tokens(b));
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
        assert_ne!(skeleton_tokens(a), skeleton_tokens(b));
    }

    #[test]
    fn comment_only_edit_keeps_skeleton() {
        let a = r#"
// old wording
fn Card() -> impl IntoView {
    view! { <div>"hi"</div> }
}
"#;
        let b = r#"
// new wording
fn Card() -> impl IntoView {
    view! { <div>"hi"</div> }
}
"#;
        assert_eq!(skeleton_tokens(a), skeleton_tokens(b));
    }

    #[test]
    fn whitespace_and_formatting_edit_keeps_skeleton() {
        let a = "fn Card() -> impl IntoView {\n    view! { <div>\"hi\"</div> \
                 }\n}\n";
        let b = "fn Card() -> impl IntoView {\n\n      view! { \
                 <div>\"hi\"</div> }\n}\n";
        assert_eq!(skeleton_tokens(a), skeleton_tokens(b));
    }

    #[test]
    fn removing_a_constant_is_subset_so_it_is_view_only() {
        let a = r#"
const UNUSED: &str = "foo";
fn Card() -> impl IntoView {
    view! { <div>"hi"</div> }
}
"#;
        let b = r#"
fn Card() -> impl IntoView {
    view! { <div>"hi"</div> }
}
"#;
        let old = skeleton_tokens(a);
        let new = skeleton_tokens(b);
        assert!(is_token_subset(&new, &old));
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
    fn comments_above_view_keep_skeleton() {
        let a = "fn a() -> impl IntoView { view! { <div></div> } }\n";
        let b = "// note\nfn a() -> impl IntoView { view! { <div></div> } }\n";
        assert_eq!(skeleton_tokens(a), skeleton_tokens(b));
    }
}
