use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn AutoForm(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] fields: Vec<(&'static str, &'static str)>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("space-y-4", class.get());
    view! {
        <div class=merged data-name="AutoForm">
            {children()}
        </div>
    }
}
