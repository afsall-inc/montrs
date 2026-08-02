use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Sonner(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] message: Option<String>,
    #[prop(optional)] visible: bool,
) -> impl IntoView {
    let merged = move || {
        let base = "fixed bottom-4 right-4 z-50 rounded-lg border bg-background px-4 py-3 shadow-lg transition-all";
        let state = if visible { "opacity-100 translate-y-0" } else { "opacity-0 translate-y-2 pointer-events-none" };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged role="status" data-name="Sonner">
            {message}
        </div>
    }
}