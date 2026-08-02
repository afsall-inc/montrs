use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn CardCarousel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex gap-4 overflow-x-auto pb-4 snap-x snap-mandatory scrollbar-hide", class.get());
    view! {
        <div class=merged data-name="CardCarousel">
            {children()}
        </div>
    }
}