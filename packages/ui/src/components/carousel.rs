use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Carousel(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged =
        move || cn!("relative overflow-hidden rounded-lg", class.get());
    view! {
        <div class=merged data-name="Carousel">
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex", class.get());
    view! {
        <div class=merged data-name="CarouselContent">
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselItem(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("min-w-0 shrink-0 grow-0 basis-full", class.get());
    view! {
        <div class=merged data-name="CarouselItem">
            {children()}
        </div>
    }
}
