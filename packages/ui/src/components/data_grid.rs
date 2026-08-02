use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn DataGrid(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("grid gap-4", class.get());
    view! {
        <div class=merged data-name="DataGrid">
            {children()}
        </div>
    }
}

#[component]
pub fn DataGridColumn(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] header: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());
    view! {
        <div class=merged data-name="DataGridColumn">
            {header.map(|h| view! { <div class="text-sm font-medium text-muted-foreground">{h}</div> })}
            {children()}
        </div>
    }
}