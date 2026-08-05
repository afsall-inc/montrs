use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Marker(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] label: Option<String>,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex h-8 w-8 items-center justify-center rounded-full bg-primary \
             text-primary-foreground text-xs font-bold shadow",
            class.get()
        )
    };
    view! {
        <div class=merged data-name="Marker">
            {label}
        </div>
    }
}
