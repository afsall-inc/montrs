use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Mask2(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] visible: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "fixed inset-0 z-40 bg-black/40";
        let state = if visible {
            "opacity-100"
        } else {
            "opacity-0 pointer-events-none"
        };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged data-name="Mask2">
            {children()}
        </div>
    }
}
