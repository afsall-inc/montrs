use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn BentoGrid(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("mx-auto grid max-w-7xl grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3", class.get());
    view! {
        <div class=merged data-name="BentoGrid">
            {children()}
        </div>
    }
}

#[component]
pub fn BentoItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] col_span: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let span = col_span.unwrap_or("");
        cn!("rounded-xl border bg-card p-6 shadow-sm", span, class.get())
    };
    view! {
        <div class=merged data-name="BentoItem">
            {children()}
        </div>
    }
}