use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Shimmer(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("animate-pulse rounded-md bg-gradient-to-r from-muted via-muted/50 to-muted bg-[length:200%_100%] animate-shimmer", class.get());
    view! {
        <div class=merged data-name="Shimmer" />
    }
}