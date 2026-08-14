use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Expandable(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] expanded: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "overflow-hidden transition-all duration-200 ease-in-out";
        let state = if expanded { "max-h-96" } else { "max-h-0" };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged data-name="Expandable">
            {children()}
        </div>
    }
}
