use leptos::prelude::*;
use crate::cn::*;

/// Avatar component with image fallback to initials.
///
/// Renders an image if available, otherwise shows initials or a fallback icon.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Avatar>
///         <AvatarImage src="https://github.com/avatar.png" />
///         <AvatarFallback>"JD"</AvatarFallback>
///     </Avatar>
/// }
/// ```
#[component]
pub fn Avatar(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full",
        class.get()
    );

    view! {
        <div class=merged data-name="Avatar">
            {children()}
        </div>
    }
}

/// Avatar image element.
#[component]
pub fn AvatarImage(
    src: String,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] alt: String,
) -> impl IntoView {
    let merged = move || cn!("aspect-square h-full w-full", class.get());

    view! {
        <img src=src class=merged alt=alt data-name="AvatarImage" />
    }
}

/// Fallback content shown when avatar image is not available.
#[component]
pub fn AvatarFallback(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "flex h-full w-full items-center justify-center rounded-full bg-muted",
        class.get()
    );

    view! {
        <div class=merged data-name="AvatarFallback">
            {children()}
        </div>
    }
}