use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Drawer(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] open: bool,
    #[prop(into, optional)] _on_close: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "fixed inset-y-0 right-0 z-50 flex w-full max-w-md \
                    flex-col border-l bg-background shadow-xl \
                    transition-transform";
        let state = if open {
            "translate-x-0"
        } else {
            "translate-x-full"
        };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged data-name="Drawer" hidden=!open>
            {children()}
        </div>
    }
}
