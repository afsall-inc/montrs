use crate::cn::*;
use leptos::prelude::*;

/// Loading skeleton placeholder.
///
/// Renders an animated placeholder for loading content.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Skeleton class="h-4 w-[250px]" />
/// }
/// ```
#[component]
pub fn Skeleton(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("animate-pulse rounded-md bg-muted", class.get());

    view! {
        <div class=merged data-name="Skeleton" />
    }
}
