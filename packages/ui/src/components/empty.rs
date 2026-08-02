use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Empty(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
) -> impl IntoView {
    let merged = move || cn!("flex flex-col items-center justify-center gap-2 py-12 text-center", class.get());
    view! {
        <div class=merged data-name="Empty">
            {title.map(|t| view! { <h3 class="text-lg font-semibold">{t}</h3> })}
            {description.map(|d| view! { <p class="text-sm text-muted-foreground">{d}</p> })}
        </div>
    }
}