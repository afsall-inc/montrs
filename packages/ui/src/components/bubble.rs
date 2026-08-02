use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Bubble(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] is_user: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "max-w-[80%] rounded-2xl px-4 py-2 text-sm";
        let align = if is_user { "ml-auto" } else { "mr-auto" };
        let style = if is_user { "bg-primary text-primary-foreground" } else { "bg-muted" };
        cn!(base, align, style, class.get())
    };
    view! {
        <div class=merged data-name="Bubble">
            {children()}
        </div>
    }
}