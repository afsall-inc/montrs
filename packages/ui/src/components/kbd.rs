use crate::cn::*;
use leptos::prelude::*;

/// Keyboard shortcut display component.
///
/// Renders a styled keyboard shortcut badge.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Kbd>"Ctrl + K"</Kbd>
/// }
/// ```
#[component]
pub fn Kbd(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "pointer-events-none inline-flex h-5 select-none items-center \
             gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] \
             font-medium text-muted-foreground opacity-100",
            class.get()
        )
    };

    view! {
        <kbd class=merged data-name="Kbd">
            {children()}
        </kbd>
    }
}
