use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Image(
    src: String,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] alt: String,
    #[prop(into, optional)] fallback: Option<String>,
) -> impl IntoView {
    let merged = move || cn!("rounded-md object-cover", class.get());
    view! {
        <img src=src class=merged alt=alt data-name="Image" />
    }
}
