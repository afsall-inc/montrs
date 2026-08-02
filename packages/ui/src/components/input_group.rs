use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn InputGroup(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center rounded-md border border-input bg-background ring-offset-background focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2", class.get());
    view! {
        <div class=merged data-name="InputGroup">
            {children()}
        </div>
    }
}

#[component]
pub fn InputAddon(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center px-3 text-sm text-muted-foreground bg-muted/50 border-r border-input", class.get());
    view! {
        <div class=merged data-name="InputAddon">
            {children()}
        </div>
    }
}