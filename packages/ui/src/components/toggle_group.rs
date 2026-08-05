use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn ToggleGroup(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "inline-flex items-center rounded-md border bg-muted/50 p-1",
            class.get()
        )
    };
    view! {
        <div class=merged role="group" data-name="ToggleGroup">
            {children()}
        </div>
    }
}

#[component]
pub fn ToggleGroupItem(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] selected: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "inline-flex items-center justify-center whitespace-nowrap \
                    rounded-sm px-3 py-1.5 text-sm font-medium \
                    transition-colors focus-visible:outline-none \
                    focus-visible:ring-2 focus-visible:ring-ring";
        let state = if selected {
            "bg-background text-foreground shadow-sm"
        } else {
            "text-muted-foreground hover:text-foreground"
        };
        cn!(base, state, class.get())
    };
    view! {
        <button type="button" class=merged data-name="ToggleGroupItem">
            {children()}
        </button>
    }
}
