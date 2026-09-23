// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Hermetic responsive testing.
//!
//! [`ResponsiveCheck`] evaluates a rendered component across viewport widths and
//! reports the **exact pixel** at which it stops behaving. Rules are geometric
//! and deterministic — no browser, no layout engine, no flakiness.
//!
//! ## Rules
//!
//! - [`Rule::HorizontalOverflow`] — a box's right edge exceeds the viewport.
//! - [`Rule::OffScreen`] — a box starts left of the viewport.
//! - [`Rule::TapTarget`] — a `button`/`input` smaller than 44×44 CSS px.
//! - [`Rule::ClippedText`] — text wider than a clipping box (`overflow-hidden`
//!   without `truncate`/`overflow-auto`). Estimated from a deterministic text
//!   metric (approximate; see the module docs for limits).
//!
//! ## Limits
//!
//! Text measurement is a deterministic approximation, not a real font. Use the
//! CDP backend when pixel-accurate text flow is required.

use crate::{
    devices::DeviceProfile,
    layout::{Breakpoints, SimLayout, Viewport},
};
use scraper::{Html, Selector};

/// A responsive rule that can be violated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    /// A box extends past the right edge of the viewport.
    HorizontalOverflow,
    /// A box starts left of the viewport.
    OffScreen,
    /// An interactive target is smaller than 44×44 CSS px.
    TapTarget,
    /// Text overflows a clipping box.
    ClippedText,
}

impl Rule {
    /// Stable rule name (used in reports and JSON).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HorizontalOverflow => "horizontal-overflow",
            Self::OffScreen => "off-screen",
            Self::TapTarget => "tap-target",
            Self::ClippedText => "clipped-text",
        }
    }
}

/// A single responsive violation.
#[derive(Debug, Clone, PartialEq)]
pub struct Violation {
    /// Which rule was violated.
    pub rule: Rule,
    /// Element descriptor (`tag` plus class attribute).
    pub element: String,
    /// Human-readable detail.
    pub detail: String,
    /// The offending distance in pixels.
    pub px: f32,
    /// The viewport width the violation occurred at.
    pub width: f32,
}

/// Violations found at one viewport width.
#[derive(Debug, Clone, PartialEq)]
pub struct ResponsiveReport {
    /// The viewport width.
    pub width: f32,
    /// Every violation found at this width.
    pub violations: Vec<Violation>,
}

impl ResponsiveReport {
    /// True when this width is clean.
    pub fn is_responsive(&self) -> bool {
        self.violations.is_empty()
    }

    /// A human-readable rendering of the report.
    pub fn render(&self) -> String {
        if self.violations.is_empty() {
            return format!("{}px: ok", self.width);
        }
        self.violations
            .iter()
            .map(|v| {
                format!(
                    "{}px: [{}] {} — {} ({:.2}px)",
                    v.width,
                    v.rule.as_str(),
                    v.element,
                    v.detail,
                    v.px
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Serialize the report as JSON (for baseline/regression storage).
    pub fn to_json(&self) -> String {
        let items = self
            .violations
            .iter()
            .map(|v| {
                format!(
                    "    {{\"rule\":\"{}\",\"element\":{},\"detail\":{},\"px\"\
                     :{:.2}}}",
                    v.rule.as_str(),
                    json_str(&v.element),
                    json_str(&v.detail),
                    v.px
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        format!(
            "{{\n  \"width\": {},\n  \"violations\": [\n{}\n  ]\n}}",
            self.width.round(),
            items
        )
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Checks a component across viewport widths.
pub struct ResponsiveCheck {
    html: String,
    breakpoints: Breakpoints,
    container: Option<f32>,
}

impl ResponsiveCheck {
    /// Start checking the given HTML (markers stripped).
    pub fn new(html: &str) -> Self {
        Self {
            html: crate::strip_hot_reload_markers(html),
            breakpoints: Breakpoints::default(),
            container: None,
        }
    }

    /// Start checking a component rendered in-process.
    pub fn from_component<F, IV>(f: F) -> Self
    where
        F: FnOnce() -> IV + 'static,
        IV: leptos::prelude::IntoView + 'static,
    {
        let html = f().to_html();
        Self::new(&html)
    }

    /// Override the responsive breakpoints.
    pub fn with_breakpoints(mut self, breakpoints: Breakpoints) -> Self {
        self.breakpoints = breakpoints;
        self
    }

    /// Declare a container width (used to resolve `@container` variants).
    pub fn with_container_width(mut self, width: f32) -> Self {
        self.container = Some(width);
        self
    }

    /// Collect violations at one viewport width.
    pub fn violations_at(&self, width: f32) -> Vec<Violation> {
        let viewport = Viewport::width(width);
        let layout = SimLayout::compute_with(
            &self.html,
            viewport,
            self.breakpoints,
            self.container,
        );
        let mut out = Vec::new();
        for bx in layout.boxes() {
            if bx.tag.is_empty() {
                continue;
            }
            if bx.width <= 0.0 && bx.height <= 0.0 {
                continue;
            }
            if bx.right() > width + 0.5 {
                out.push(Violation {
                    rule: Rule::HorizontalOverflow,
                    element: element_desc(&bx.tag, &bx.class),
                    detail: format!(
                        "right edge {}px > viewport {}px",
                        bx.right(),
                        width
                    ),
                    px: bx.right() - width,
                    width,
                });
            }
            if bx.x < -0.5 {
                out.push(Violation {
                    rule: Rule::OffScreen,
                    element: element_desc(&bx.tag, &bx.class),
                    detail: format!("left edge {}px < 0", bx.x),
                    px: -bx.x,
                    width,
                });
            }
            if is_tap_target(&bx.tag)
                && bx.displayed()
                && bx.sized
                && (bx.width < 44.0 || bx.height < 44.0)
            {
                let px = (44.0 - bx.width.min(bx.height)).max(0.0);
                out.push(Violation {
                    rule: Rule::TapTarget,
                    element: element_desc(&bx.tag, &bx.class),
                    detail: format!(
                        "{}×{}px below the 44px minimum target",
                        bx.width, bx.height
                    ),
                    px,
                    width,
                });
            }
            if clips_text(&bx.class)
                && let Some((text, overflow_px)) = clipped_text(&self.html, bx)
            {
                out.push(Violation {
                    rule: Rule::ClippedText,
                    element: element_desc(&bx.tag, &bx.class),
                    detail: format!("{:?} exceeds the clipping box", text),
                    px: overflow_px,
                    width,
                });
            }
        }
        out
    }

    /// The full report for one width.
    pub fn report(&self, width: f32) -> ResponsiveReport {
        ResponsiveReport {
            width,
            violations: self.violations_at(width),
        }
    }

    /// True when the layout has no violations at `width`.
    pub fn fits_at(&self, width: f32) -> bool {
        self.violations_at(width).is_empty()
    }

    /// Sweep a range of widths (inclusive) and return only the widths with
    /// violations, plus the total violation count.
    ///
    /// `step` is the granularity in px — `1` for a pixel-exact sweep.
    pub fn sweep(
        &self,
        range: std::ops::RangeInclusive<f32>,
        step: f32,
    ) -> Vec<ResponsiveReport> {
        let step = step.max(1.0);
        let mut out = Vec::new();
        let mut width = *range.start();
        while width <= *range.end() {
            let report = self.report(width);
            if !report.is_responsive() {
                out.push(report);
            }
            width += step;
        }
        out
    }

    /// The smallest width in `range` at which the layout is clean, found by
    /// bisection.
    ///
    /// Returns `None` if it never becomes clean inside the range. Assumes the
    /// fit predicate is monotone (true for large widths) — which holds for the
    /// overflow and off-screen rules, the common failure modes.
    pub fn required_width(
        &self,
        range: std::ops::RangeInclusive<f32>,
    ) -> Option<f32> {
        let (lo, hi) = (*range.start(), *range.end());
        if !self.fits_at(hi) {
            return None;
        }
        if self.fits_at(lo) {
            return Some(lo);
        }
        // Monotone: does not fit at `lo`, fits at `hi` — find the boundary.
        let mut lo_bad = lo;
        let mut hi_good = hi;
        while (hi_good - lo_bad) > 1.0 {
            let mid = ((lo_bad + hi_good) / 2.0).floor();
            if self.fits_at(mid) {
                hi_good = mid;
            } else {
                lo_bad = mid;
            }
        }
        Some(hi_good)
    }

    /// Assert the layout is clean at every width in `range` (step `1` by
    /// default). On failure the report names the exact offending width, rule,
    /// element, and pixel delta.
    #[track_caller]
    pub fn assert_no_irresponsive_px(
        &self,
        range: std::ops::RangeInclusive<f32>,
    ) {
        self.assert_no_irresponsive_px_step(range, 1.0);
    }

    /// [`assert_no_irresponsive_px`] with an explicit step (1 = exact).
    #[track_caller]
    pub fn assert_no_irresponsive_px_step(
        &self,
        range: std::ops::RangeInclusive<f32>,
        step: f32,
    ) {
        let (start, end) = (*range.start(), *range.end());
        let bad = self.sweep(range, step);
        assert!(
            bad.is_empty(),
            "responsive violations across {}..={}px (step {}):\n{}",
            start,
            end,
            step,
            bad.iter()
                .map(ResponsiveReport::render)
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    /// Assert the layout is clean on every standard device.
    #[track_caller]
    pub fn assert_responsive(&self) {
        self.assert_responsive_on(&crate::devices::standard_devices());
    }

    /// Assert the layout is clean on the given devices (including their
    /// rotated variants).
    #[track_caller]
    pub fn assert_responsive_on(&self, devices: &[DeviceProfile]) {
        let mut failures = Vec::new();
        for device in devices {
            let mut checked = vec![device.clone()];
            checked.push(device.rotated());
            for d in &checked {
                let report = self.report(d.width);
                if !report.is_responsive() {
                    failures.push(format!(
                        "device {} ({}): {}",
                        d.name,
                        d.id,
                        report.render()
                    ));
                }
            }
        }
        assert!(
            failures.is_empty(),
            "responsive violations on devices:\n{}",
            failures.join("\n")
        );
    }
}

fn element_desc(tag: &str, class: &Option<String>) -> String {
    match class {
        Some(c) if !c.trim().is_empty() => {
            format!("<{tag} class=\"{}\">", c.trim())
        }
        _ => format!("<{tag}>"),
    }
}

fn is_tap_target(tag: &str) -> bool {
    matches!(tag, "button" | "input" | "select" | "textarea")
}

/// Whether an element clips its content.
fn clips_text(class: &Option<String>) -> bool {
    let Some(c) = class else {
        return false;
    };
    let tokens: Vec<&str> = c.split_whitespace().collect();
    let clips = tokens.iter().any(|t| {
        *t == "overflow-hidden"
            || t.starts_with("overflow-clip")
            || *t == "overflow-x-hidden"
            || *t == "overflow-y-hidden"
    });
    let escapes = tokens.iter().any(|t| {
        *t == "truncate"
            || *t == "overflow-auto"
            || *t == "overflow-x-auto"
            || *t == "overflow-y-auto"
            || *t == "overflow-scroll"
    });
    clips && !escapes
}

fn clipped_text(
    html: &str,
    bx: &crate::layout::LayoutBox,
) -> Option<(String, f32)> {
    // Approximate: take the element's text and compare it against the box width.
    // This is a deterministic estimate, not a real font.
    let class = bx.class.as_ref()?;
    let text = text_of(html, class)?;
    if text.is_empty() {
        return None;
    }
    let est = approx_text_width(&text, 16.0);
    if est > bx.width + 0.5 {
        Some((text, est - bx.width))
    } else {
        None
    }
}

impl crate::layout::LayoutBox {
    /// Visible according to the resolved display (unknown elements count).
    fn displayed(&self) -> bool {
        self.width > 0.0 || self.height > 0.0
    }
}

fn text_of(html: &str, class: &str) -> Option<String> {
    let doc = Html::parse_fragment(html);
    let selector =
        Selector::parse("body, div, span, p, h1, h2, h3, h4, h5, h6, td, th")
            .map_err(|_| ())
            .ok()?;
    let want: Vec<&str> = class.split_whitespace().collect();
    for el in doc.select(&selector) {
        if let Some(c) = el.value().attr("class") {
            let has: Vec<&str> = c.split_whitespace().collect();
            if !want.is_empty() && want.iter().all(|t| has.contains(t)) {
                let text = el.text().collect::<Vec<_>>().join(" ");
                let text =
                    text.split_whitespace().collect::<Vec<_>>().join(" ");
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }
    }
    None
}

/// Deterministic, font-free text width estimate (approximate).
fn approx_text_width(text: &str, font_size: f32) -> f32 {
    text.chars().map(|c| glyph_factor(c) * font_size).sum()
}

fn glyph_factor(c: char) -> f32 {
    match c {
        ' ' => 0.25,
        '.' | ',' | ':' | ';' | '!' | '\'' | '(' | ')' | '[' | ']' | '|'
        | 'i' | 'l' | 't' | 'f' | 'j' => 0.30,
        'm' | 'w' | 'M' | 'W' => 0.85,
        c if c.is_ascii_uppercase() => 0.62,
        c if c.is_ascii_digit() => 0.56,
        _ => 0.52,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    fn render<IV: IntoView + 'static>(
        f: impl FnOnce() -> IV + 'static,
    ) -> String {
        f().to_html()
    }

    #[test]
    fn clean_layout_has_no_violations() {
        let html = render(|| view! { <div class="w-[200px]">"ok"</div> });
        let check = ResponsiveCheck::new(&html);
        let report = check.report(320.0);
        assert!(report.is_responsive(), "{}", report.render());
        assert!(report.to_json().contains("\"violations\""));
    }

    #[test]
    fn reports_exact_overflow_pixels() {
        let html = render(|| view! { <div style="width: 600px" /> });
        let check = ResponsiveCheck::new(&html);
        let report = check.report(320.0);
        assert!(!report.is_responsive());
        let v = &report.violations[0];
        assert_eq!(v.rule, Rule::HorizontalOverflow);
        assert!((v.px - 280.0).abs() < 1.0, "px = {}", v.px);
        assert_eq!(v.width, 320.0);
    }

    #[test]
    fn off_screen_boxes_are_flagged() {
        let html = render(|| {
            view! { <div style="width: 200px; margin-left: -50px" /> }
        });
        let check = ResponsiveCheck::new(&html);
        let report = check.report(400.0);
        assert!(
            report.violations.iter().any(|v| v.rule == Rule::OffScreen),
            "{}",
            report.render()
        );
    }

    #[test]
    fn small_buttons_violate_the_tap_target_rule() {
        let html = render(|| view! { <button class="w-4 h-4">"x"</button> });
        let check = ResponsiveCheck::new(&html);
        let report = check.report(400.0);
        assert!(
            report.violations.iter().any(|v| v.rule == Rule::TapTarget),
            "{}",
            report.render()
        );
    }

    #[test]
    fn responsive_component_passes_the_standard_matrix() {
        let html = render(|| {
            view! {
                <div class="flex flex-wrap gap-2">
                    <button class="px-3 py-2">"One"</button>
                    <button class="px-3 py-2">"Two"</button>
                    <button class="px-3 py-2">"Three"</button>
                </div>
            }
        });
        ResponsiveCheck::new(&html).assert_responsive();
    }

    #[test]
    fn off_screen_trigger_reports_the_first_bad_width() {
        let html = render(|| view! { <div class="w-[600px]" /> });
        let check = ResponsiveCheck::new(&html);
        let reports = check.sweep(320.0..=800.0, 1.0);
        assert!(!reports.is_empty());
        // Clean widths start at 600px; the last violating width is 599px.
        let last_bad = reports.last().unwrap().width;
        assert!((last_bad - 599.0).abs() < 1.0, "last bad = {last_bad}");
    }

    #[test]
    fn required_width_finds_the_exact_breakpoint() {
        let html = render(|| view! { <div class="w-[600px]" /> });
        let check = ResponsiveCheck::new(&html);
        let required = check.required_width(320.0..=1920.0).unwrap();
        assert!((required - 600.0).abs() <= 2.0, "required = {required}");
        assert!(check.fits_at(required));
        assert!(!check.fits_at(required - 1.0));
    }

    #[test]
    fn sweep_is_empty_for_clean_layout() {
        let html = render(|| view! { <div class="w-full">"ok"</div> });
        let check = ResponsiveCheck::new(&html);
        assert!(check.sweep(320.0..=1280.0, 1.0).is_empty());
        check.assert_no_irresponsive_px(320.0..=1280.0);
    }

    #[test]
    fn json_contains_rule_and_element() {
        let html = render(|| view! { <div class="w-[600px]" /> });
        let json = ResponsiveCheck::new(&html).report(320.0).to_json();
        assert!(json.contains("horizontal-overflow"));
        assert!(json.contains("\"element\""));
    }
}
