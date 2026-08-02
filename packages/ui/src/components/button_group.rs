use leptos::prelude::*;
use crate::cn::*;

/// Button group for grouping related buttons together.
///
/// Renders a horizontal container that visually joins buttons.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <ButtonGroup>
///         <Button>{"Left"}</Button>
///         <Button>{"Center"}</Button>
///         <Button>{"Right"}</Button>
///     </ButtonGroup>
/// }
/// ```
#[component]
pub fn ButtonGroup(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "inline-flex items-center justify-center [&>button]:rounded-none [&>button:first-child]:rounded-l-md [&>button:last-child]:rounded-r-md [&>button:focus-visible]:z-10 [&>button]:border-r [&>button:last-child]:border-r-0",
        class.get()
    );

    view! {
        <div class=merged data-name="ButtonGroup" role="group">
            {children()}
        </div>
    }
}