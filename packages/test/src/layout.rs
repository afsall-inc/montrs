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
//! Text metrics are not modelled (text nodes contribute no intrinsic size), so
//! use the CDP backend when pixel-accurate text flow matters.

use scraper::{ElementRef, Html};
use taffy::prelude::*;

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
}

impl SimLayout {
    /// Compute the layout of `html` at `viewport`.
    pub fn compute(html: &str, viewport: Viewport) -> Self {
        let cleaned = crate::strip_hot_reload_markers(html);
        let document = Html::parse_fragment(&cleaned);
        let mut builder = TreeBuilder::new();
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

        Self { viewport, boxes }
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

struct TreeBuilder {
    tree: TaffyTree<()>,
    meta: std::collections::HashMap<NodeId, (String, Option<String>)>,
}

impl TreeBuilder {
    fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
            meta: std::collections::HashMap::new(),
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

        let mut style = style_for(element);
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
        self.meta.insert(node, (tag, class));
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

        let (tag, class) = self
            .meta
            .get(&node)
            .cloned()
            .unwrap_or_else(|| (String::new(), None));

        let mut boxes = vec![LayoutBox {
            tag,
            class,
            x,
            y,
            width: layout.size.width,
            height: layout.size.height,
        }];

        for child in self.tree.children(node).unwrap_or_default() {
            boxes.extend(self.collect(child, x, y));
        }
        boxes
    }
}

fn style_for(element: ElementRef<'_>) -> Style {
    let tag = element.value().name().to_ascii_lowercase();
    let mut style = Style {
        display: default_display(&tag),
        ..Default::default()
    };

    if let Some(class) = element.value().attr("class") {
        apply_classes(&mut style, class);
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

fn apply_classes(style: &mut Style, class: &str) {
    for token in class.split_whitespace() {
        apply_class(style, token);
    }
}

fn apply_class(style: &mut Style, token: &str) {
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
    value
        .strip_suffix("px")
        .and_then(|v| v.parse::<f32>().ok())
        .or_else(|| value.parse::<f32>().ok())
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
