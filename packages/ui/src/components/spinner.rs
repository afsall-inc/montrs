use crate::cn::*;
use leptos::prelude::*;

/// Loading spinner component.
///
/// Renders an animated spinner for loading states.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Spinner />
///     <Spinner class="h-8 w-8 text-primary" />
/// }
/// ```
#[component]
pub fn Spinner(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    let merged =
        move || cn!("animate-spin h-4 w-4 text-muted-foreground", class.get());

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24" height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=merged
            data-name="Spinner"
        >
            <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
    }
}
