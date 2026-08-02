use leptos::prelude::*;
use crate::cn::*;

/// Horizontal or vertical separator.
///
/// Renders a thematic break, either horizontal or vertical.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Separator />
///     <Separator orientation="vertical" class="h-10" />
/// }
/// ```
#[component]
pub fn Separator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] orientation: &'static str,
) -> impl IntoView {
    let is_horizontal = orientation != "vertical";
    let merged = move || {
        let base = "shrink-0 bg-border";
        let dir = if is_horizontal { "h-[1px] w-full" } else { "h-full w-[1px]" };
        cn!(base, dir, class.get())
    };

    view! {
        <div
            class=merged
            role="separator"
            aria-orientation=if is_horizontal { "horizontal" } else { "vertical" }
            data-name="Separator"
        />
    }
}