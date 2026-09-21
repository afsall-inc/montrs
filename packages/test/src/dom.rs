// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Component and DOM testing without a browser.
//!
//! [`ComponentTest`] renders a `view!` to HTML in-process and exposes
//! selector-based queries and assertions. Use it for structure, text, classes,
//! attributes, and ARIA checks. For real layout and interaction, use the
//! browser/CDP backend (see the crate roadmap).

use crate::strip_hot_reload_markers;
#[allow(unused_imports)]
use leptos::prelude::RenderHtml;
use scraper::{ElementRef, Html, Selector};

/// A rendered component, queryable by CSS selector.
pub struct ComponentTest {
    html: String,
    document: Html,
}

impl ComponentTest {
    /// Render a `view!` (or any `IntoView`) to HTML, in-process.
    ///
    /// The closure is rendered on the calling thread with the SSR renderer.
    pub fn render<F, IV>(f: F) -> Self
    where
        F: FnOnce() -> IV + 'static,
        IV: leptos::prelude::IntoView + 'static,
    {
        let html = f().to_html();
        Self::from_html(&html)
    }

    /// Build a test from an existing HTML string.
    pub fn from_html(html: &str) -> Self {
        let cleaned = strip_hot_reload_markers(html);
        let document = Html::parse_fragment(&cleaned);
        Self {
            html: cleaned,
            document,
        }
    }

    /// The (marker-stripped) HTML.
    pub fn html(&self) -> &str {
        &self.html
    }

    /// All visible text, whitespace-collapsed.
    pub fn text(&self) -> String {
        self.document
            .root_element()
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn selector(sel: &str) -> Selector {
        Selector::parse(sel)
            .unwrap_or_else(|e| panic!("invalid CSS selector {sel:?}: {e:?}"))
    }

    /// All elements matching `sel`.
    pub fn all<'a>(&'a self, sel: &str) -> Vec<ElementRef<'a>> {
        let selector = Self::selector(sel);
        self.document.select(&selector).collect()
    }

    /// The first element matching `sel`, if any.
    pub fn first<'a>(&'a self, sel: &str) -> Option<ElementRef<'a>> {
        self.all(sel).into_iter().next()
    }

    /// The number of elements matching `sel`.
    pub fn count(&self, sel: &str) -> usize {
        self.all(sel).len()
    }

    /// Whether at least one element matches `sel`.
    pub fn exists(&self, sel: &str) -> bool {
        self.first(sel).is_some()
    }

    /// The value of `attr` on the first match of `sel`.
    pub fn attr(&self, sel: &str, attr: &str) -> Option<String> {
        self.first(sel)
            .and_then(|el| el.value().attr(attr).map(|s| s.to_string()))
    }

    /// Whether the first match of `sel` has the given class token.
    pub fn has_class(&self, sel: &str, class: &str) -> bool {
        self.attr(sel, "class")
            .map(|c| c.split_whitespace().any(|t| t == class))
            .unwrap_or(false)
    }

    /// Whether the first match of `sel` has the exact ARIA role.
    pub fn role(&self, sel: &str) -> Option<String> {
        self.attr(sel, "role")
    }

    // ---- assertions -------------------------------------------------------

    /// Assert at least one element matches `sel`.
    #[track_caller]
    pub fn assert_selector(&self, sel: &str) -> &Self {
        assert!(
            self.exists(sel),
            "expected an element matching {sel:?} in:\n{}",
            self.html
        );
        self
    }

    /// Assert no element matches `sel`.
    #[track_caller]
    pub fn assert_no_selector(&self, sel: &str) -> &Self {
        assert!(
            !self.exists(sel),
            "expected NO element matching {sel:?}, found {}",
            self.count(sel)
        );
        self
    }

    /// Assert the rendered text contains `needle`.
    #[track_caller]
    pub fn assert_text(&self, needle: &str) -> &Self {
        let text = self.text();
        assert!(
            text.contains(needle),
            "expected text to contain {needle:?}, got: {text:?}"
        );
        self
    }

    /// Assert `sel`'s attribute equals `value`.
    #[track_caller]
    pub fn assert_attr(&self, sel: &str, attr: &str, value: &str) -> &Self {
        let actual = self.attr(sel, attr);
        assert_eq!(
            actual.as_deref(),
            Some(value),
            "expected {sel:?} {attr:?}={value:?}"
        );
        self
    }

    /// Assert `sel` has the given class token.
    #[track_caller]
    pub fn assert_class(&self, sel: &str, class: &str) -> &Self {
        assert!(
            self.has_class(sel, class),
            "expected {sel:?} to have class {class:?}, got: {:?}",
            self.attr(sel, "class")
        );
        self
    }

    /// Assert `sel` has the exact ARIA role.
    #[track_caller]
    pub fn assert_role(&self, sel: &str, role: &str) -> &Self {
        assert_eq!(
            self.role(sel).as_deref(),
            Some(role),
            "expected {sel:?} role={role:?}"
        );
        self
    }

    /// Assert exactly `n` elements match `sel`.
    #[track_caller]
    pub fn assert_count(&self, sel: &str, n: usize) -> &Self {
        assert_eq!(
            self.count(sel),
            n,
            "expected {n} elements matching {sel:?}, found {}",
            self.count(sel)
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn renders_and_queries_structure() {
        let view = ComponentTest::render(|| {
            view! {
                <div class="card" role="dialog" aria-label="Hello">
                    <h1 class="title">"Title"</h1>
                    <button type="button" disabled=true>
                        "Save"
                    </button>
                </div>
            }
        });

        view.assert_selector("div.card");
        view.assert_selector("h1.title");
        view.assert_text("Title");
        view.assert_text("Save");
        view.assert_role("div", "dialog");
        view.assert_attr("div", "aria-label", "Hello");
        view.assert_attr("button", "type", "button");
        view.assert_class("h1", "title");
        view.assert_count("button", 1);
        assert_eq!(view.count("span"), 0);
    }

    #[test]
    fn marker_stripping_applies() {
        let view = ComponentTest::from_html(
            "<!--hot-reload|a|open--><p \
             class=\"x\">hi</p><!--hot-reload|a|close-->",
        );
        assert_eq!(view.html(), "<p class=\"x\">hi</p>");
        view.assert_selector("p.x");
        view.assert_text("hi");
    }

    #[test]
    fn negation_assertions() {
        let view =
            ComponentTest::render(|| view! { <div class="only">"x"</div> });
        view.assert_no_selector(".missing");
        assert_eq!(view.attr(".only", "role"), None);
    }
}
