use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn ButtonAction(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] loading: bool,
    #[prop(optional)] disabled: bool,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "inline-flex items-center justify-center gap-2 \
                    whitespace-nowrap rounded-md text-sm font-medium \
                    ring-offset-background transition-colors \
                    focus-visible:outline-none focus-visible:ring-2 \
                    focus-visible:ring-ring focus-visible:ring-offset-2 \
                    disabled:pointer-events-none disabled:opacity-50 \
                    bg-primary text-primary-foreground hover:bg-primary/90 \
                    h-10 px-4 py-2";
        cn!(base, class.get())
    };
    let handle_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };
    view! {
        <button
            type="button"
            class=merged
            disabled=disabled || loading
            on:click=handle_click
            data-name="ButtonAction"
        >
            {children()}
        </button>
    }
}
