use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Mask(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] visible: bool,
) -> impl IntoView {
    let merged = move || {
        let base = "fixed inset-0 z-50 bg-black/50 transition-opacity";
        let state = if visible {
            "opacity-100"
        } else {
            "opacity-0 pointer-events-none"
        };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged data-name="Mask" />
    }
}
