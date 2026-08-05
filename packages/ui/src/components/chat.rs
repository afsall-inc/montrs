use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Chat(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-col gap-4", class.get());
    view! {
        <div class=merged data-name="Chat">
            {children()}
        </div>
    }
}

#[component]
pub fn ChatMessage(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] align: &'static str,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "flex gap-3";
        let dir = if align == "end" {
            "flex-row-reverse"
        } else {
            ""
        };
        cn!(base, dir, class.get())
    };
    view! {
        <div class=merged data-name="ChatMessage">
            {children()}
        </div>
    }
}
