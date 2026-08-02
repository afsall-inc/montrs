use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn AspectRatio(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] ratio: f64,
    children: Children,
) -> impl IntoView {
    let padding = format!("{}%", (1.0 / ratio) * 100.0);
    view! {
        <div class=move || cn!("relative w-full", class.get()) data-name="AspectRatio">
            <div class="absolute inset-0">{children()}</div>
            <div class="w-full" style=format!("padding-bottom: {}", padding) />
        </div>
    }
}