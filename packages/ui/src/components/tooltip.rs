use leptos::prelude::*;
use crate::cn::*;

#[component]
pub fn Tooltip(
    #[prop(into, optional)] text: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let merged = move || cn!("relative inline-flex", class.get());

    let tooltip_id = crate::utils::Utils::use_random_id();

    let show = move |_| open.set(true);
    let hide = move |_| open.set(false);

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            open.set(false);
        }
    };

    let on_focus = move |_| open.set(true);
    let on_blur = move |_| open.set(false);

    view! {
        <div
            class=merged
            on:mouseenter=show
            on:mouseleave=hide
            on:focusin=on_focus
            on:focusout=on_blur
            on:keydown=on_key_down
            data-name="Tooltip"
        >
            <div
                aria-describedby=tooltip_id.clone()
                data-name="TooltipTrigger"
            >
                {children()}
            </div>
            <div
                role="tooltip"
                id=tooltip_id.clone()
                class="z-50 overflow-hidden rounded-md border bg-popover px-3 py-1.5 text-sm text-popover-foreground shadow-md animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95"
                data-state=move || if open.get() { "open" } else { "closed" }
                hidden=move || !open.get()
                data-name="TooltipContent"
            >
                {text.clone()}
            </div>
        </div>
    }
}