use leptos::prelude::*;
use crate::cn::*;

/// Tooltip component that shows content on hover.
///
/// Renders a tooltip that appears when hovering over the trigger element.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Tooltip text="Helpful info">
///         <span>"Hover me"</span>
///     </Tooltip>
/// }
/// ```
#[component]
pub fn Tooltip(
    #[prop(into, optional)] text: Option<String>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let merged = move || cn!("", class.get());

    view! {
        <div
            class=merged
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| open.set(false)
            data-name="Tooltip"
        >
            {children()}
            <div
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