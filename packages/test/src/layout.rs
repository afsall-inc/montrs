// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Hermetic layout and overflow evaluation.
//!
//! [`SimLayout`] builds a real box-model tree with [`taffy`] from a rendered
//! component and computes positions and sizes at a given viewport width, so a
//! test can assert that nothing overflows horizontally without a browser.
//!
//! ## Supported style subset
//!
//! Layout is derived from inline `style` attributes plus a practical subset of
//! Tailwind utilities:
//!
//! - `display`: `block`, `flex`, `inline-flex`, `grid`, `none`
//! - `flex-direction`, `flex-wrap`, `justify-*`, `items-*`
//! - `gap-*`, `p-*`/`px-*`/`py-*`, `m-*`/`mx-*`/`my-*`
//! - `w-*`/`w-full`/`w-[Npx]`, `h-*`, `min-w-*`, `max-w-*`
//!
//! Responsive variants are evaluated against the current viewport:
//! `sm:` `md:` `lg:` `xl:` `2xl:`, `max-<bp>:`, and arbitrary
//! `min-[Npx]:`/`max-[Npx]:`. Container variants (`@md:`) are evaluated against
//! a container width supplied to [`SimLayout::compute_with`].
//!
//! Text metrics are not modelled (text nodes contribute no intrinsic size), so
//! use the CDP backend when pixel-accurate text flow matters.

use scraper::{ElementRef, Html};
use taffy::prelude::*;

/// Responsive breakpoints (Tailwind CSS v4 defaults, overridable).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Breakpoints {
    /// `sm` — minimum width in px.
    pub sm: f32,
    /// `md` — minimum width in px.
    pub md: f32,
    /// `lg` — minimum width in px.
    pub lg: f32,
    /// `xl` — minimum width in px.
    pub xl: f32,
    /// `2xl` — minimum width in px.
    pub two_xl: f32,
}

impl Default for Breakpoints {
    fn default() -> Self {
        // Tailwind v4 defaults.
        Self {
            sm: 640.0,
            md: 768.0,
            lg: 1024.0,
            xl: 1280.0,
            two_xl: 1536.0,
        }
    }
}

impl Breakpoints {
    fn min_for(&self, name: &str) -> Option<f32> {
        match name {
            "sm" => Some(self.sm),
            "md" => Some(self.md),
            "lg" => Some(self.lg),
            "xl" => Some(self.xl),
            "2xl" => Some(self.two_xl),
            _ => None,
        }
    }
}

/// Context used to resolve responsive utilities.
#[derive(Debug, Clone, Copy)]
struct StyleCtx<'a> {
    width: f32,
    breakpoints: &'a Breakpoints,
    container: Option<f32>,
}

/// A viewport size in CSS pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// Width in pixels.
    pub width: f32,
    /// Height in pixels.
    pub height: f32,
}

impl Viewport {
    /// A viewport of the given width and height.
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// A viewport of the given width with a tall default height.
    pub fn width(width: f32) -> Self {
        Self {
            width,
            height: 2000.0,
        }
    }
}

/// A laid-out box for one element.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBox {
    /// Element tag name (lowercase).
    pub tag: String,
    /// The element's `class` attribute, if any.
    pub class: Option<String>,
    /// Left offset in pixels.
    pub x: f32,
    /// Top offset in pixels.
    pub y: f32,
    /// Computed width in pixels.
    pub width: f32,
    /// Computed height in pixels.
    pub height: f32,
    /// Whether the element declared an explicit size (`w-*`/`h-*` or inline
    /// `width`/`height`). Intrinsic (content-driven) sizes are not modelled, so
    /// size-based rules should only trust declared sizes.
    pub sized: bool,
}

impl LayoutBox {
    /// The right edge (`x + width`).
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// The bottom edge (`y + height`).
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

/// The computed layout of a rendered component.
pub struct SimLayout {
    viewport: Viewport,
    boxes: Vec<LayoutBox>,
    breakpoints: Breakpoints,
}

impl SimLayout {
    /// Compute the layout of `html` at `viewport` with default breakpoints.
    pub fn compute(html: &str, viewport: Viewport) -> Self {
        Self::compute_with(html, viewport, Breakpoints::default(), None)
    }

    /// Compute the layout with explicit breakpoints and an optional container
    /// width (used to resolve `@container` variants).
    pub fn compute_with(
        html: &str,
        viewport: Viewport,
        breakpoints: Breakpoints,
        container: Option<f32>,
    ) -> Self {
        let cleaned = crate::strip_hot_reload_markers(html);
        let document = Html::parse_fragment(&cleaned);
        let ctx = StyleCtx {
            width: viewport.width,
            breakpoints: &breakpoints,
            container,
        };
        let mut builder = TreeBuilder::new(ctx);
        let root = builder.build(document.root_element(), viewport);

        let boxes = match root {
            Some(root) => {
                let _ = builder.tree.compute_layout(
                    root,
                    Size {
                        width: AvailableSpace::Definite(viewport.width),
                        height: AvailableSpace::Definite(viewport.height),
                    },
                );
                builder.collect(root, 0.0, 0.0)
            }
            None => Vec::new(),
        };

        Self {
            viewport,
            boxes,
            breakpoints,
        }
    }

    /// The breakpoints used for this computation.
    pub fn breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    /// All laid-out boxes.
    pub fn boxes(&self) -> &[LayoutBox] {
        &self.boxes
    }

    /// The viewport the layout was computed at.
    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    /// Every box whose right edge exceeds the viewport width.
    pub fn overflowing(&self) -> Vec<&LayoutBox> {
        let limit = self.viewport.width + 0.5;
        self.boxes.iter().filter(|b| b.right() > limit).collect()
    }

    /// True when no box exceeds the viewport width.
    pub fn fits_horizontally(&self) -> bool {
        self.overflowing().is_empty()
    }

    /// Assert no box overflows the viewport horizontally.
    #[track_caller]
    pub fn assert_no_horizontal_overflow(&self) {
        let overflowing = self.overflowing();
        assert!(
            overflowing.is_empty(),
            "horizontal overflow at width {}px: {}",
            self.viewport.width,
            overflowing
                .iter()
                .map(|b| format!(
                    "<{} class={:?}> right={}px",
                    b.tag,
                    b.class.as_deref().unwrap_or(""),
                    b.right()
                ))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    /// Assert every box fits within the viewport width.
    #[track_caller]
    pub fn assert_fits(&self) {
        self.assert_no_horizontal_overflow();
    }
}

struct TreeBuilder<'a> {
    tree: TaffyTree<()>,
    meta: std::collections::HashMap<NodeId, (String, Option<String>, bool)>,
    ctx: StyleCtx<'a>,
}

fn has_declared_size(class: Option<&str>, inline: Option<&str>) -> bool {
    if let Some(inline) = inline
        && (inline.contains("width") || inline.contains("height"))
    {
        return true;
    }
    if let Some(class) = class {
        for token in class.split_whitespace() {
            let (_, base) = split_variants(token);
            if let Some((prefix, _)) = split_tailwind(base)
                && matches!(prefix, "w" | "h" | "size" | "min-w" | "min-h")
            {
                return true;
            }
        }
    }
    false
}

impl<'a> TreeBuilder<'a> {
    fn new(ctx: StyleCtx<'a>) -> Self {
        Self {
            tree: TaffyTree::new(),
            meta: std::collections::HashMap::new(),
            ctx,
        }
    }

    fn build(
        &mut self,
        element: ElementRef<'_>,
        viewport: Viewport,
    ) -> Option<NodeId> {
        let children: Vec<NodeId> = element
            .children()
            .filter_map(ElementRef::wrap)
            .filter_map(|child| self.build(child, viewport))
            .collect();

        let mut style = style_for(element, self.ctx);
        if element.parent().is_none() {
            style.size.width = Dimension::from_length(viewport.width);
        }

        let node = if children.is_empty() {
            self.tree.new_leaf(style).ok()
        } else {
            self.tree.new_with_children(style, &children).ok()
        }?;

        let tag = element.value().name().to_ascii_lowercase();
        let class = element.value().attr("class").map(|s| s.to_string());
        let sized =
            has_declared_size(class.as_deref(), element.value().attr("style"));
        self.meta.insert(node, (tag, class, sized));
        Some(node)
    }

    fn collect(
        &self,
        node: NodeId,
        parent_x: f32,
        parent_y: f32,
    ) -> Vec<LayoutBox> {
        let Ok(layout) = self.tree.layout(node) else {
            return Vec::new();
        };
        let x = parent_x + layout.location.x;
        let y = parent_y + layout.location.y;

        let (tag, class, sized) = self
            .meta
            .get(&node)
            .cloned()
            .unwrap_or_else(|| (String::new(), None, false));

        let mut boxes = vec![LayoutBox {
            tag,
            class,
            x,
            y,
            width: layout.size.width,
            height: layout.size.height,
            sized,
        }];

        for child in self.tree.children(node).unwrap_or_default() {
            boxes.extend(self.collect(child, x, y));
        }
        boxes
    }
}

fn style_for(element: ElementRef<'_>, ctx: StyleCtx<'_>) -> Style {
    let tag = element.value().name().to_ascii_lowercase();
    let mut style = Style {
        display: default_display(&tag),
        ..Default::default()
    };

    if let Some(class) = element.value().attr("class") {
        apply_classes(&mut style, class, ctx);
    }
    if let Some(inline) = element.value().attr("style") {
        apply_inline(&mut style, inline);
    }

    style
}

fn default_display(tag: &str) -> Display {
    match tag {
        "div" | "section" | "header" | "nav" | "main" | "footer" | "ul"
        | "ol" | "li" | "form" | "article" | "aside" => Display::Flex,
        _ => Display::Block,
    }
}

/// Split a class token into its variant prefixes and the base utility.
///
/// `md:hover:flex` → `(["md", "hover"], "flex")`. Only `:` at bracket depth 0
/// separates variants, so arbitrary values (`w-[10px:odd]`,
/// `min-[500px]:w-[600px]`) are handled correctly.
fn split_variants(token: &str) -> (Vec<&str>, &str) {
    let mut depth = 0i32;
    let mut cols = Vec::new();
    for (i, ch) in token.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => depth -= 1,
            ':' if depth == 0 => cols.push(i),
            _ => {}
        }
    }
    let Some(&last) = cols.last() else {
        return (Vec::new(), token);
    };
    let mut variants = Vec::with_capacity(cols.len());
    let mut start = 0;
    for &i in &cols {
        variants.push(&token[start..i]);
        start = i + 1;
    }
    (variants, &token[last + 1..])
}

/// Whether every variant in the chain currently applies.
fn variants_active(variants: &[&str], ctx: StyleCtx<'_>) -> bool {
    variants.iter().all(|variant| variant_active(variant, ctx))
}

fn variant_active(variant: &str, ctx: StyleCtx<'_>) -> bool {
    let v: &str = variant;
    // min-width breakpoints.
    if let Some(min) = ctx.breakpoints.min_for(v) {
        return ctx.width >= min;
    }
    // max-<bp>: (strictly below the breakpoint).
    if let Some(name) = v.strip_prefix("max-")
        && let Some(min) = ctx.breakpoints.min_for(name)
    {
        return ctx.width < min;
    }
    // Arbitrary min-[Npx] / max-[Npx].
    if let Some(inner) = v.strip_prefix("min-[")
        && let Some(px) = inner.strip_suffix(']').and_then(parse_px)
    {
        return ctx.width >= px;
    }
    if let Some(inner) = v.strip_prefix("max-[")
        && let Some(px) = inner.strip_suffix(']').and_then(parse_px)
    {
        return ctx.width < px;
    }
    // Container query variants (@md, @max-md, @[600px]) use the container width.
    if let Some(rest) = v.strip_prefix('@') {
        let width = ctx.container.unwrap_or(ctx.width);
        if let Some(min) = ctx.breakpoints.min_for(rest.trim_start_matches('@'))
        {
            return width >= min;
        }
        if let Some(name) = rest.strip_prefix("max-")
            && let Some(min) = ctx.breakpoints.min_for(name)
        {
            return width < min;
        }
        if let Some(inner) = rest.strip_prefix('[')
            && let Some(px) = inner.strip_suffix(']').and_then(parse_px)
        {
            return width >= px;
        }
        return true;
    }
    // Interaction / color-scheme variants are unknown at layout time; apply the
    // base utility so layout-affecting ones are never silently dropped.
    true
}

fn apply_classes(style: &mut Style, class: &str, ctx: StyleCtx<'_>) {
    for token in class.split_whitespace() {
        let (variants, base) = split_variants(token);
        if variants_active(&variants, ctx) {
            apply_class(style, base, ctx);
        }
    }
}

fn apply_class(style: &mut Style, token: &str, _ctx: StyleCtx<'_>) {
    match token {
        "hidden" => style.display = Display::None,
        "block" => style.display = Display::Block,
        "flex" => style.display = Display::Flex,
        "inline-flex" => style.display = Display::Flex,
        "grid" => style.display = Display::Grid,
        "flex-col" => style.flex_direction = FlexDirection::Column,
        "flex-row" => style.flex_direction = FlexDirection::Row,
        "flex-wrap" => style.flex_wrap = FlexWrap::Wrap,
        "flex-nowrap" => style.flex_wrap = FlexWrap::NoWrap,
        "items-center" => style.align_items = Some(AlignItems::Center),
        "items-start" => style.align_items = Some(AlignItems::Start),
        "items-end" => style.align_items = Some(AlignItems::End),
        "justify-center" => {
            style.justify_content = Some(JustifyContent::Center)
        }
        "justify-between" => {
            style.justify_content = Some(JustifyContent::SpaceBetween)
        }
        "justify-end" => style.justify_content = Some(JustifyContent::End),
        _ => apply_tailwind_size(style, token),
    }
}

fn apply_tailwind_size(style: &mut Style, token: &str) {
    let Some((prefix, value)) = split_tailwind(token) else {
        return;
    };
    let px = || scale_px(value);
    let len = || length(scale_px(value).unwrap_or(0.0));

    match prefix {
        "gap" => {
            if let Some(p) = px() {
                style.gap = Size {
                    width: length(p),
                    height: length(p),
                };
            }
        }
        "p" => {
            if let Some(p) = px() {
                style.padding = Rect {
                    left: length(p),
                    right: length(p),
                    top: length(p),
                    bottom: length(p),
                };
            }
        }
        "px" => {
            if let Some(p) = px() {
                style.padding.left = length(p);
                style.padding.right = length(p);
            }
        }
        "py" => {
            if let Some(p) = px() {
                style.padding.top = length(p);
                style.padding.bottom = length(p);
            }
        }
        "m" => {
            if let Some(p) = px() {
                style.margin = Rect {
                    left: length(p),
                    right: length(p),
                    top: length(p),
                    bottom: length(p),
                };
            }
        }
        "mx" => {
            if let Some(p) = px() {
                style.margin.left = length(p);
                style.margin.right = length(p);
            }
        }
        "my" => {
            if let Some(p) = px() {
                style.margin.top = length(p);
                style.margin.bottom = length(p);
            }
        }
        "w" => {
            style.size.width = width_value(value).unwrap_or_else(len);
        }
        "h" => {
            if let Some(v) = height_value(value) {
                style.size.height = v;
            }
        }
        "min-w" => {
            if let Some(v) = width_value(value) {
                style.min_size.width = v;
            }
        }
        "max-w" => {
            if let Some(v) = max_width_value(value) {
                style.max_size.width = v;
            }
        }
        "min-h" => {
            if let Some(v) = height_value(value) {
                style.min_size.height = v;
            }
        }
        _ => {}
    }
}

fn split_tailwind(token: &str) -> Option<(&str, &str)> {
    let (prefix, value) = token.rsplit_once('-')?;
    Some((prefix, value))
}

fn scale_px(value: &str) -> Option<f32> {
    if let Some(inner) =
        value.strip_prefix('[').and_then(|v| v.strip_suffix(']'))
    {
        return parse_px(inner);
    }
    value.parse::<f32>().ok().map(|n| n * 4.0)
}

fn parse_px(value: &str) -> Option<f32> {
    let raw = value.trim();
    let (neg, unsigned) = match raw.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, raw),
    };
    let num = unsigned
        .strip_suffix("px")
        .and_then(|v| v.parse::<f32>().ok())
        .or_else(|| unsigned.parse::<f32>().ok())?;
    Some(if neg { -num } else { num })
}

fn width_value(value: &str) -> Option<Dimension> {
    match value {
        "full" => Some(Dimension::from_percent(1.0)),
        "screen" => Some(Dimension::from_percent(1.0)),
        "auto" => Some(Dimension::AUTO),
        _ => parse_px(value).map(length),
    }
}

fn height_value(value: &str) -> Option<Dimension> {
    match value {
        "full" => Some(Dimension::from_percent(1.0)),
        "auto" => Some(Dimension::AUTO),
        _ => parse_px(value).map(length),
    }
}

fn max_width_value(value: &str) -> Option<Dimension> {
    match value {
        "full" => Some(Dimension::from_percent(1.0)),
        "xs" => Some(length(320.0)),
        "sm" => Some(length(384.0)),
        "md" => Some(length(448.0)),
        "lg" => Some(length(512.0)),
        "xl" => Some(length(576.0)),
        "2xl" => Some(length(672.0)),
        "none" => None,
        _ => parse_px(value).map(length),
    }
}

fn apply_inline(style: &mut Style, inline: &str) {
    for decl in inline.split(';') {
        let Some((prop, value)) = decl.split_once(':') else {
            continue;
        };
        let prop = prop.trim();
        let value = value.trim();
        match prop {
            "display" => match value {
                "flex" => style.display = Display::Flex,
                "grid" => style.display = Display::Grid,
                "block" => style.display = Display::Block,
                "none" => style.display = Display::None,
                _ => {}
            },
            "flex-direction" => match value {
                "column" => style.flex_direction = FlexDirection::Column,
                "row" => style.flex_direction = FlexDirection::Row,
                _ => {}
            },
            "gap" => {
                if let Some(px) = parse_px(value) {
                    style.gap = Size {
                        width: length(px),
                        height: length(px),
                    };
                }
            }
            "width" => {
                if let Some(d) = parse_dimension(value) {
                    style.size.width = d;
                }
            }
            "height" => {
                if let Some(d) = parse_dimension(value) {
                    style.size.height = d;
                }
            }
            "max-width" => {
                if let Some(d) = parse_dimension(value) {
                    style.max_size.width = d;
                }
            }
            "min-width" => {
                if let Some(d) = parse_dimension(value) {
                    style.min_size.width = d;
                }
            }
            "margin" => {
                if let Some(px) = parse_px(value) {
                    style.margin = Rect {
                        left: length(px),
                        right: length(px),
                        top: length(px),
                        bottom: length(px),
                    };
                }
            }
            "margin-left" => {
                if let Some(px) = parse_px(value) {
                    style.margin.left = length(px);
                }
            }
            "margin-right" => {
                if let Some(px) = parse_px(value) {
                    style.margin.right = length(px);
                }
            }
            "margin-top" => {
                if let Some(px) = parse_px(value) {
                    style.margin.top = length(px);
                }
            }
            "margin-bottom" => {
                if let Some(px) = parse_px(value) {
                    style.margin.bottom = length(px);
                }
            }
            "padding" => {
                if let Some(px) = parse_px(value) {
                    style.padding = Rect {
                        left: length(px),
                        right: length(px),
                        top: length(px),
                        bottom: length(px),
                    };
                }
            }
            _ => {}
        }
    }
}

fn parse_dimension(value: &str) -> Option<Dimension> {
    if value.ends_with('%') {
        return value
            .trim_end_matches('%')
            .parse::<f32>()
            .ok()
            .map(|n| percent(n / 100.0));
    }
    parse_px(value).map(length)
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

    fn div_box(layout: &SimLayout) -> &LayoutBox {
        layout
            .boxes()
            .iter()
            .find(|b| b.tag == "div")
            .expect("a <div> box")
    }

    #[test]
    fn responsive_variant_applies_only_above_breakpoint() {
        let html = render(|| {
            view! { <div class="w-[200px] md:w-[800px]" /> }
        });

        let mobile = SimLayout::compute(&html, Viewport::width(320.0));
        assert!(mobile.fits_horizontally());
        assert_eq!(div_box(&mobile).width, 200.0);

        let tablet = SimLayout::compute(&html, Viewport::width(768.0));
        assert!(!tablet.fits_horizontally());
        assert_eq!(div_box(&tablet).width, 800.0);
    }

    #[test]
    fn max_variant_applies_only_below_breakpoint() {
        let html = render(|| {
            view! { <div class="w-[800px] max-md:w-[200px]" /> }
        });
        assert!(
            SimLayout::compute(&html, Viewport::width(500.0))
                .fits_horizontally()
        );
        assert!(
            !SimLayout::compute(&html, Viewport::width(768.0))
                .fits_horizontally()
        );
    }

    #[test]
    fn arbitrary_breakpoint_variants() {
        let html = render(|| {
            view! { <div class="w-[100px] min-[500px]:w-[600px]" /> }
        });
        assert!(
            SimLayout::compute(&html, Viewport::width(400.0))
                .fits_horizontally()
        );
        assert!(
            !SimLayout::compute(&html, Viewport::width(500.0))
                .fits_horizontally()
        );
    }

    #[test]
    fn container_variant_uses_container_width() {
        let html = render(|| {
            view! { <div class="w-[100px] @md:w-[600px]" /> }
        });
        // Container narrower than md: base width wins.
        let narrow = SimLayout::compute_with(
            &html,
            Viewport::width(800.0),
            Breakpoints::default(),
            Some(400.0),
        );
        assert_eq!(div_box(&narrow).width, 100.0);

        // Container at/above md: the container variant wins.
        let wide = SimLayout::compute_with(
            &html,
            Viewport::width(900.0),
            Breakpoints::default(),
            Some(768.0),
        );
        assert_eq!(div_box(&wide).width, 600.0);
    }

    #[test]
    fn split_variants_handles_chains_and_arbitrary_values() {
        let (variants, base) = split_variants("md:hover:flex");
        assert_eq!(variants, vec!["md", "hover"]);
        assert_eq!(base, "flex");

        let (variants, base) = split_variants("w-[10px:odd]");
        assert!(variants.is_empty());
        assert_eq!(base, "w-[10px:odd]");
    }

    #[test]
    fn fixed_child_wider_than_viewport_overflows() {
        let html = render(|| view! { <div style="width: 500px" /> });
        let layout = SimLayout::compute(&html, Viewport::width(320.0));
        assert!(!layout.fits_horizontally());
        assert!(layout.overflowing().iter().any(|b| b.right() > 320.0));
    }

    #[test]
    fn row_of_children_that_fits_does_not_overflow() {
        let html = render(|| {
            view! {
                <div style="display: flex; flex-direction: row; gap: 8px">
                    <div style="width: 100px" />
                    <div style="width: 100px" />
                </div>
            }
        });
        SimLayout::compute(&html, Viewport::width(320.0))
            .assert_no_horizontal_overflow();
    }

    #[test]
    fn tailwind_width_utility_is_honoured() {
        let html = render(|| view! { <div class="w-[500px]" /> });
        let layout = SimLayout::compute(&html, Viewport::width(320.0));
        assert!(!layout.fits_horizontally());
    }

    #[test]
    fn max_width_caps_a_full_width_child() {
        let html = render(|| {
            view! { <div class="w-full max-w-[200px]" /> }
        });
        SimLayout::compute(&html, Viewport::width(320.0))
            .assert_no_horizontal_overflow();
    }

    #[test]
    fn hidden_elements_take_no_space() {
        let html = render(|| view! { <div class="hidden w-[999px]" /> });
        let layout = SimLayout::compute(&html, Viewport::width(320.0));
        assert!(layout.fits_horizontally());
    }
}
