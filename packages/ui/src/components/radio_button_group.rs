use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn RadioButtonGroup(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] _value: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex flex-col gap-2", class.get());
    view! {
        <div class=merged role="radiogroup" data-name="RadioButtonGroup">
            {children()}
        </div>
    }
}
