use leptos::prelude::*;
use crate::cn::*;

/// Form label component.
///
/// Renders a styled label for form inputs.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Label for="email">"Email"</Label>
/// }
/// ```
#[component]
pub fn Label(
    #[prop(into, optional)] for_id: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
        class.get()
    );

    view! {
        <label for=for_id class=merged data-name="Label">
            {children()}
        </label>
    }
}