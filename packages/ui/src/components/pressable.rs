use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Pressable(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] on_press: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("cursor-pointer select-none transition-opacity active:opacity-70", class.get());
    let handle = move |_| {
        if let Some(cb) = on_press {
            cb.run(());
        }
    };
    view! {
        <div class=merged role="button" tabindex="0" on:click=handle data-name="Pressable">
            {children()}
        </div>
    }
}