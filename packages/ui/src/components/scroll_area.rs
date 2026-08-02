use leptos::prelude::*;
use crate::cn::*;

/// Scroll area with custom scrollbar styling.
///
/// Renders a scrollable container with themed scrollbars.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <ScrollArea class="h-[200px]">
///         <p>"Long content..."</p>
///     </ScrollArea>
/// }
/// ```
#[component]
pub fn ScrollArea(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "relative overflow-hidden",
        class.get()
    );

    let viewport_class = "h-full w-full rounded-[inherit] [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-border [&::-webkit-scrollbar-track]:bg-transparent";

    view! {
        <div class=merged data-name="ScrollArea">
            <div class=viewport_class data-name="ScrollAreaViewport">
                {children()}
            </div>
            <div
                class="flex touch-none select-none transition-colors"
                data-name="ScrollBar"
            >
                <div class="relative flex-1 rounded-full bg-border" />
            </div>
        </div>
    }
}