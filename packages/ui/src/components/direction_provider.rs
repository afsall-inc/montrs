use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn DirectionProvider(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] dir: &'static str,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());
    view! {
        <div class=merged dir=dir data-name="DirectionProvider">
            {children()}
        </div>
    }
}