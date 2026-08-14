use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn SelectNative(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] value: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex h-10 w-full rounded-md border border-input bg-background \
             px-3 py-2 text-sm ring-offset-background \
             focus-visible:outline-none focus-visible:ring-2 \
             focus-visible:ring-ring focus-visible:ring-offset-2",
            class.get()
        )
    };
    let on_change =
        move |ev: leptos::ev::Event| value.set(event_target_value(&ev));
    view! {
        <select class=merged prop:value=move || value.get() on:change=on_change data-name="SelectNative">
            {children()}
        </select>
    }
}
