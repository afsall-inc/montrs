use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Sidenav(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex h-full w-64 flex-col border-r bg-background", class.get());
    view! {
        <nav class=merged data-name="Sidenav">
            {children()}
        </nav>
    }
}

#[component]
pub fn SidenavItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] active: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors";
        let state = if active { "bg-muted text-foreground" } else { "text-muted-foreground hover:bg-muted/50 hover:text-foreground" };
        cn!(base, state, class.get())
    };
    view! {
        <a class=merged data-name="SidenavItem">
            {children()}
        </a>
    }
}