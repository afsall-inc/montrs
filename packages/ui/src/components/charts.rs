use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Chart(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] title: Option<String>,
) -> impl IntoView {
    let merged = move || cn!("flex h-48 w-full items-center justify-center rounded-lg border bg-muted/20 text-muted-foreground", class.get());
    view! {
        <div class=merged data-name="Chart">
            {title.map(|t| view! { <span class="text-sm">{t}</span> })}
        </div>
    }
}