use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Marquee(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("overflow-hidden whitespace-nowrap", class.get());
    view! {
        <div class=merged data-name="Marquee">
            <div class="inline-block animate-marquee">
                {children()}
            </div>
        </div>
    }
}