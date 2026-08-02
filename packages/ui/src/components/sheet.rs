use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Sheet(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] open: bool,
    #[prop(into, optional)] on_close: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "fixed inset-y-0 right-0 z-50 flex w-full max-w-md flex-col border-l bg-background shadow-xl transition-transform";
        let state = if open { "translate-x-0" } else { "translate-x-full" };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged data-name="Sheet" hidden=!open>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center justify-between border-b px-6 py-4", class.get());
    view! {
        <div class=merged data-name="SheetHeader">
            {children()}
        </div>
    }
}

#[component]
pub fn SheetContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex-1 overflow-y-auto px-6 py-4", class.get());
    view! {
        <div class=merged data-name="SheetContent">
            {children()}
        </div>
    }
}

#[component]
pub fn SheetFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center justify-end gap-2 border-t px-6 py-4", class.get());
    view! {
        <div class=merged data-name="SheetFooter">
            {children()}
        </div>
    }
}