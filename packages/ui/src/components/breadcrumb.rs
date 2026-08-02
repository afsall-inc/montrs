use leptos::prelude::*;
use crate::cn::*;

/// Breadcrumb navigation component.
///
/// Renders a navigation trail with separators between items.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Breadcrumb>
///         <BreadcrumbList>
///             <BreadcrumbItem><BreadcrumbLink href="/">"Home"</BreadcrumbLink></BreadcrumbItem>
///             <BreadcrumbSeparator />
///             <BreadcrumbItem><BreadcrumbLink href="/docs">"Docs"</BreadcrumbLink></BreadcrumbItem>
///         </BreadcrumbList>
///     </Breadcrumb>
/// }
/// ```
#[component]
pub fn Breadcrumb(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("", class.get());

    view! {
        <nav aria-label="breadcrumb" class=merged data-name="Breadcrumb">
            {children()}
        </nav>
    }
}

/// Breadcrumb list container.
#[component]
pub fn BreadcrumbList(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5",
        class.get()
    );

    view! {
        <ol class=merged data-name="BreadcrumbList">
            {children()}
        </ol>
    }
}

/// Breadcrumb item.
#[component]
pub fn BreadcrumbItem(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("inline-flex items-center gap-1.5", class.get());

    view! {
        <li class=merged data-name="BreadcrumbItem">
            {children()}
        </li>
    }
}

/// Breadcrumb link.
#[component]
pub fn BreadcrumbLink(
    href: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("transition-colors hover:text-foreground", class.get());

    view! {
        <a href=href class=merged data-name="BreadcrumbLink">
            {children()}
        </a>
    }
}

/// Separator between breadcrumb items.
#[component]
pub fn BreadcrumbSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("[&>svg]:size-3.5", class.get());

    view! {
        <li class=merged data-name="BreadcrumbSeparator" aria-hidden="true">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24" height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="m9 18 6-6-6-6" />
            </svg>
        </li>
    }
}

/// Ellipsis for truncated breadcrumb trails.
#[component]
pub fn BreadcrumbEllipsis(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("flex h-9 w-9 items-center justify-center", class.get());

    view! {
        <li class=merged data-name="BreadcrumbEllipsis" aria-hidden="true">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24" height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="h-4 w-4"
            >
                <circle cx="12" cy="12" r="1" />
                <circle cx="19" cy="12" r="1" />
                <circle cx="5" cy="12" r="1" />
            </svg>
            <span class="sr-only">"More"</span>
        </li>
    }
}

/// Current page indicator in breadcrumb.
#[component]
pub fn BreadcrumbPage(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("font-normal text-foreground", class.get());

    view! {
        <span role="link" aria-disabled="true" aria-current="page" class=merged data-name="BreadcrumbPage">
            {children()}
        </span>
    }
}