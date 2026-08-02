use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn DatePickerDualState(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] start: RwSignal<String>,
    #[prop(into, optional)] end: RwSignal<String>,
) -> impl IntoView {
    let merged = move || cn!("flex items-center gap-2", class.get());
    let on_start = move |ev: leptos::ev::Event| start.set(event_target_value(&ev));
    let on_end = move |ev: leptos::ev::Event| end.set(event_target_value(&ev));
    view! {
        <div class=merged data-name="DatePickerDualState">
            <input type="date" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value=move || start.get() on:input=on_start />
            <span class="text-muted-foreground">"to"</span>
            <input type="date" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value=move || end.get() on:input=on_end />
        </div>
    }
}