use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Switch(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    let merged = move || {
        let base = "peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50";
        let state = if checked.get() {
            "bg-primary"
        } else {
            "bg-input"
        };
        cn!(base, state, class.get())
    };

    let toggle = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == " " {
            ev.prevent_default();
            checked.update(|v| *v = !*v);
        }
    };

    let click = move |_| checked.update(|v| *v = !*v);

    let id = crate::utils::Utils::use_random_id();

    let thumb_class = move || {
        let base = "pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform";
        let translate = if checked.get() { "translate-x-5" } else { "translate-x-0" };
        cn!(base, translate)
    };

    view! {
        <div class="flex items-center space-x-2">
            <button
                type="button"
                role="switch"
                id=id.clone()
                class=merged
                aria-checked=move || checked.get()
                aria-label=aria_label
                aria-disabled=disabled.then_some("true")
                data-state=move || if checked.get() { "checked" } else { "unchecked" }
                disabled=disabled
                on:click=click
                on:keydown=toggle
                data-name="Switch"
            >
                <span class=thumb_class data-name="SwitchThumb" />
            </button>
            {label.map(move |l| {
                let label_id = id.clone();
                view! {
                    <label
                        for=label_id
                        class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                    >
                        {l}
                    </label>
                }
            })}
        </div>
    }
}