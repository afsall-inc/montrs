use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn MultiSelect(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] value: RwSignal<Vec<String>>,
    _children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let merged = move || {
        cn!(
            "flex h-10 w-full items-center justify-between rounded-md border \
             border-input bg-background px-3 py-2 text-sm \
             ring-offset-background",
            class.get()
        )
    };
    let toggle = move |_| open.update(|v| *v = !*v);
    view! {
        <div class="relative" data-name="MultiSelect">
            <button type="button" class=merged on:click=toggle data-name="MultiSelectTrigger">
                <span>{move || format!("{} selected", value.get().len())}</span>
            </button>
        </div>
    }
}

#[component]
pub fn MultiSelectItem(
    _value: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "flex cursor-pointer items-center gap-2 rounded-sm px-2 py-1.5 \
             text-sm hover:bg-accent",
            class.get()
        )
    };
    view! {
        <div class=merged data-name="MultiSelectItem">
            {children()}
        </div>
    }
}
