use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Field(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] error: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("grid gap-1.5", class.get());
    view! {
        <div class=merged data-name="Field">
            {label.map(|l| view! { <label class="text-sm font-medium leading-none">{l}</label> })}
            {children()}
            {error.filter(|e| !e.is_empty()).map(|e| view! { <p class="text-sm font-medium text-destructive">{e}</p> })}
        </div>
    }
}