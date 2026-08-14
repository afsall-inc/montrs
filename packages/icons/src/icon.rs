use crate::glyph::Glyph;
use leptos::{prelude::*, text_prop::TextProp};

pub const DEFAULT_SIZE: &str = "24";
pub const DEFAULT_FILL: &str = "none";
pub const DEFAULT_STROKE: &str = "currentColor";
pub const DEFAULT_STROKE_WIDTH: &str = "1.5";

#[component]
pub fn Icon(
    #[prop(into)] glyph: Signal<Glyph>,
    #[prop(into, optional)] class: Option<TextProp>,
    #[prop(into, optional)] size: Option<TextProp>,
    #[prop(into, optional)] fill: Option<TextProp>,
    #[prop(into, optional)] stroke: Option<TextProp>,
    #[prop(into, optional)] stroke_width: Option<TextProp>,
) -> impl IntoView {
    let svg = TextProp::from(move || glyph.get().svg());
    render_svg(svg, class, size, fill, stroke, stroke_width)
}

#[component]
pub fn CustomIcon(
    #[prop(into)] svg: TextProp,
    #[prop(into, optional)] class: Option<TextProp>,
    #[prop(into, optional)] size: Option<TextProp>,
    #[prop(into, optional)] fill: Option<TextProp>,
    #[prop(into, optional)] stroke: Option<TextProp>,
    #[prop(into, optional)] stroke_width: Option<TextProp>,
) -> impl IntoView {
    render_svg(svg, class, size, fill, stroke, stroke_width)
}

pub fn render_svg(
    svg: TextProp,
    class: Option<TextProp>,
    size: Option<TextProp>,
    fill: Option<TextProp>,
    stroke: Option<TextProp>,
    stroke_width: Option<TextProp>,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "".into());
    let size = size.unwrap_or_else(|| DEFAULT_SIZE.into());
    let size2 = size.clone();
    let fill = fill.unwrap_or_else(|| DEFAULT_FILL.into());
    let stroke = stroke.unwrap_or_else(|| DEFAULT_STROKE.into());
    let stroke_width =
        stroke_width.unwrap_or_else(|| DEFAULT_STROKE_WIDTH.into());

    view! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class=move || class.get()
          width=move || size.get()
          height=move || size2.get()
          viewBox="0 0 24 24"
          fill=move || fill.get()
          stroke=move || stroke.get()
          stroke-width=move || stroke_width.get()
          stroke-linecap="round"
          stroke-linejoin="round"
          inner_html=move || svg.get()
        />
    }
}
