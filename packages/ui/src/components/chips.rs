use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Chips(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-wrap gap-2", class.get());
    view! {
        <div class=merged data-name="Chips">
            {children()}
        </div>
    }
}

#[component]
pub fn Chip(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_remove: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("inline-flex items-center gap-1 rounded-full border bg-muted/50 px-2.5 py-0.5 text-xs font-medium", class.get());
    view! {
        <span class=merged data-name="Chip">
            {children()}
        </span>
    }
}