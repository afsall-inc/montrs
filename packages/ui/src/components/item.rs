use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Item(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors hover:bg-muted/50", class.get());
    view! {
        <div class=merged data-name="Item">
            {children()}
        </div>
    }
}