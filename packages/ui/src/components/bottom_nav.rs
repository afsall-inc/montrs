use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn BottomNav(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("fixed bottom-0 left-0 right-0 z-50 flex items-center justify-around border-t bg-background px-4 py-2", class.get());
    view! {
        <nav class=merged data-name="BottomNav">
            {children()}
        </nav>
    }
}

#[component]
pub fn BottomNavItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] active: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "flex flex-col items-center gap-1 px-3 py-1 text-xs font-medium transition-colors";
        let state = if active { "text-primary" } else { "text-muted-foreground hover:text-foreground" };
        cn!(base, state, class.get())
    };
    view! {
        <button class=merged data-name="BottomNavItem">
            {children()}
        </button>
    }
}