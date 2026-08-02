use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn ActionBar(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center gap-2 rounded-lg border bg-background p-2 shadow-sm", class.get());
    view! {
        <div class=merged data-name="ActionBar">
            {children()}
        </div>
    }
}