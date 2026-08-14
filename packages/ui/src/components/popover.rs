use crate::cn::*;
use leptos::prelude::*;

/// Popover with trigger and content.
///
/// Shows a floating content panel when the trigger is clicked.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Popover>
///         <PopoverTrigger>"Open"</PopoverTrigger>
///         <PopoverContent>"Popover content"</PopoverContent>
///     </Popover>
/// }
/// ```
#[component]
pub fn Popover(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="Popover">
            {children()}
        </div>
    }
}

/// Trigger that opens the popover.
#[component]
pub fn PopoverTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("PopoverTrigger must be inside Popover");
    let toggle = move |_| open.update(|v| *v = !*v);

    let merged = move || cn!("", class.get());

    view! {
        <button type="button" class=merged on:click=toggle data-name="PopoverTrigger">
            {children()}
        </button>
    }
}

/// Popover content panel.
#[component]
pub fn PopoverContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("PopoverContent must be inside Popover");

    let merged = move || {
        cn!(
            "z-50 w-72 rounded-md border bg-popover p-4 \
             text-popover-foreground shadow-md outline-none \
             data-[state=open]:animate-in data-[state=closed]:animate-out \
             data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 \
             data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
            class.get()
        )
    };

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="PopoverContent"
        >
            {children()}
        </div>
        {move || if open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="PopoverBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}
