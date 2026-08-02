use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Header(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("sticky top-0 z-40 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60", class.get());
    view! {
        <header class=merged data-name="Header">
            <div class="flex h-14 items-center px-6">
                {children()}
            </div>
        </header>
    }
}