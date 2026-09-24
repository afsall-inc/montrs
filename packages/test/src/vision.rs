// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Agent-facing observation of a rendered component.
//!
//! [`AgentVision::observe`] packages the things an agent normally needs a
//! `computer-use` screenshot for — the accessibility tree, the laid-out boxes,
//! the responsive violations, and a deterministic layout fingerprint — into one
//! JSON-serializable [`Observation`]. An agent can inspect, diff, or assert on
//! it purely by code.

use crate::{
    devices::DeviceProfile,
    layout::SimLayout,
    responsive::{ResponsiveCheck, Violation},
    strip_hot_reload_markers,
};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

/// An accessible node discovered in the rendered markup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A11yNode {
    /// Element tag name (lowercase).
    pub tag: String,
    /// Explicit or implicit ARIA role.
    pub role: String,
    /// Accessible name (`aria-label` or visible text).
    pub name: String,
}

/// A laid-out box summary for the observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservedBox {
    pub tag: String,
    pub class: Option<String>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// A structured, machine-readable snapshot of a component on a device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    /// Device identifier.
    pub device_id: String,
    /// Viewport width in px.
    pub width: f32,
    /// Viewport height in px.
    pub height: f32,
    /// All accessible nodes in document order.
    pub a11y: Vec<A11yNode>,
    /// Laid-out boxes.
    pub boxes: Vec<ObservedBox>,
    /// Human-readable responsive violations (`rule: element — detail`).
    pub violations: Vec<String>,
    /// Deterministic hash of the laid-out geometry (useful for regression diffs).
    pub layout_hash: u64,
}

impl Observation {
    /// True when no responsive violations were found.
    pub fn is_responsive(&self) -> bool {
        self.violations.is_empty()
    }

    /// Serialize to pretty JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Whether an accessible node with `role` and `name` exists.
    pub fn has_a11y(&self, role: &str, name: &str) -> bool {
        self.a11y
            .iter()
            .any(|n| n.role == role && n.name.contains(name))
    }

    /// Assert no responsive violations were recorded.
    #[track_caller]
    pub fn assert_responsive(&self) {
        assert!(
            self.is_responsive(),
            "observation on {} ({}px) had violations:\n{}",
            self.device_id,
            self.width,
            self.violations.join("\n")
        );
    }

    /// Assert an accessible node with `(role, name)` exists.
    #[track_caller]
    pub fn assert_a11y(&self, role: &str, name: &str) {
        assert!(
            self.has_a11y(role, name),
            "expected a11y node role={role:?} name containing {name:?}; \
             found: {:?}",
            self.a11y
        );
    }
}

/// Entry point for building an [`Observation`].
pub struct AgentVision;

impl AgentVision {
    /// Observe `html` rendered on `device`.
    pub fn observe(html: &str, device: &DeviceProfile) -> Observation {
        let cleaned = strip_hot_reload_markers(html);
        let a11y = extract_a11y(&cleaned);
        let layout = SimLayout::compute(&cleaned, device.viewport());
        let boxes: Vec<ObservedBox> = layout
            .boxes()
            .iter()
            .filter(|b| !b.tag.is_empty())
            .map(|b| ObservedBox {
                tag: b.tag.clone(),
                class: b.class.clone(),
                x: b.x,
                y: b.y,
                width: b.width,
                height: b.height,
            })
            .collect();
        let violations: Vec<String> = ResponsiveCheck::new(&cleaned)
            .violations_at(device.width)
            .into_iter()
            .map(format_violation)
            .collect();
        let layout_hash = hash_boxes(&boxes);
        Observation {
            device_id: device.id.clone(),
            width: device.width,
            height: device.height,
            a11y,
            boxes,
            violations,
            layout_hash,
        }
    }
}

fn format_violation(v: Violation) -> String {
    format!(
        "[{}] {} — {} ({:.2}px)",
        v.rule.as_str(),
        v.element,
        v.detail,
        v.px
    )
}

fn hash_boxes(boxes: &[ObservedBox]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for b in boxes {
        b.tag.hash(&mut h);
        b.class.hash(&mut h);
        (b.x.round() as i64).hash(&mut h);
        (b.y.round() as i64).hash(&mut h);
        (b.width.round() as i64).hash(&mut h);
        (b.height.round() as i64).hash(&mut h);
    }
    h.finish()
}

fn extract_a11y(html: &str) -> Vec<A11yNode> {
    let doc = Html::parse_fragment(html);
    let Ok(sel) = Selector::parse(
        "button, a, input, select, textarea, h1, h2, h3, h4, nav, main, \
         header, footer, [role], [aria-label]",
    ) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for el in doc.select(&sel) {
        let tag = el.value().name().to_ascii_lowercase();
        let role = el
            .value()
            .attr("role")
            .map(str::to_string)
            .unwrap_or_else(|| implicit_role(&tag).to_string());
        let name = el
            .value()
            .attr("aria-label")
            .map(str::to_string)
            .or_else(|| el.value().attr("placeholder").map(str::to_string))
            .unwrap_or_else(|| {
                el.text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            });
        out.push(A11yNode { tag, role, name });
    }
    out
}

fn implicit_role(tag: &str) -> &'static str {
    match tag {
        "button" => "button",
        "a" => "link",
        "input" => "textbox",
        "select" => "combobox",
        "textarea" => "textbox",
        "h1" | "h2" | "h3" | "h4" => "heading",
        "nav" => "navigation",
        "main" => "main",
        "header" => "banner",
        "footer" => "contentinfo",
        _ => "generic",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::iphone_15;
    use leptos::prelude::*;

    #[test]
    fn observation_captures_a11y_boxes_and_clean_status() {
        let html = view! {
            <main class="w-full p-4">
                <h1>"Dashboard"</h1>
                <button type="button">"Save"</button>
            </main>
        }
        .to_html();
        let obs = AgentVision::observe(&html, &iphone_15());
        obs.assert_responsive();
        obs.assert_a11y("heading", "Dashboard");
        obs.assert_a11y("button", "Save");
        assert!(!obs.boxes.is_empty());
        assert!(obs.to_json().contains("\"layout_hash\""));
    }

    #[test]
    fn observation_hash_is_deterministic() {
        let html = view! { <div class="w-[200px]">"x"</div> }.to_html();
        let a = AgentVision::observe(&html, &iphone_15());
        let b = AgentVision::observe(&html, &iphone_15());
        assert_eq!(a.layout_hash, b.layout_hash);
    }

    #[test]
    fn observation_records_overflow_violation() {
        let html = view! { <div class="w-[900px]">"wide"</div> }.to_html();
        let obs = AgentVision::observe(&html, &iphone_15());
        assert!(!obs.is_responsive());
        assert!(obs.violations[0].contains("horizontal-overflow"));
    }
}
