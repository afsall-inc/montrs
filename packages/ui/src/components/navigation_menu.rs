use leptos::prelude::*;
use crate::cn::*;

/// Navigation menu component.
///
/// Renders a horizontal navigation bar with optional dropdown items.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <NavigationMenu>
///         <NavigationMenuList>
///             <NavigationMenuItem>
///                 <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
///                 <NavigationMenuContent>
///                     <a href="/product-a">"Product A"</a>
///                 </NavigationMenuContent>
///             </NavigationMenuItem>
///         </NavigationMenuList>
///     </NavigationMenu>
/// }
/// ```
#[component]
pub fn NavigationMenu(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "relative z-10 flex max-w-max flex-1 items-center justify-center",
        class.get()
    );

    view! {
        <nav class=merged data-name="NavigationMenu">
            {children()}
        </nav>
    }
}

/// Navigation menu list.
#[component]
pub fn NavigationMenuList(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "group flex flex-1 list-none items-center justify-center space-x-1",
        class.get()
    );

    view! {
        <ul class=merged data-name="NavigationMenuList">
            {children()}
        </ul>
    }
}

/// Navigation menu item.
#[component]
pub fn NavigationMenuItem(
    children: Children,
) -> impl IntoView {
    view! {
        <li data-name="NavigationMenuItem">
            {children()}
        </li>
    }
}

/// Navigation menu trigger with dropdown.
#[component]
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "group inline-flex h-10 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-accent/50 data-[state=open]:bg-accent/50",
        class.get()
    );

    view! {
        <button type="button" class=merged data-name="NavigationMenuTrigger">
            {children()}
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24" height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180"
            >
                <path d="m6 9 6 6 6-6" />
            </svg>
        </button>
    }
}

/// Navigation menu content.
#[component]
pub fn NavigationMenuContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "left-0 top-0 w-full data-[motion^=from-]:animate-in data-[motion^=to-]:animate-out data-[motion^=from-]:fade-in data-[motion^=to-]:fade-out data-[motion=from-end]:slide-in-from-right-52 data-[motion=from-start]:slide-in-from-left-52 data-[motion=to-end]:slide-out-to-right-52 data-[motion=to-start]:slide-out-to-left-52 md:absolute md:w-auto",
        class.get()
    );

    view! {
        <div class=merged data-name="NavigationMenuContent">
            {children()}
        </div>
    }
}

/// Navigation menu link.
#[component]
pub fn NavigationMenuLink(
    href: String,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
        class.get()
    );

    view! {
        <a href=href class=merged data-name="NavigationMenuLink">
            {children()}
        </a>
    }
}

/// Navigation menu indicator.
#[component]
pub fn NavigationMenuIndicator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!(
        "top-full z-[1] flex h-1.5 items-end justify-center overflow-hidden data-[state=visible]:animate-in data-[state=hidden]:animate-out data-[state=hidden]:fade-out data-[state=visible]:fade-in",
        class.get()
    );

    view! {
        <div class=merged data-name="NavigationMenuIndicator">
            <div class="relative top-[60%] h-2 w-2 rotate-45 rounded-tl-sm bg-border shadow-md" />
        </div>
    }
}

/// Navigation menu viewport.
#[component]
pub fn NavigationMenuViewport(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!(
        "origin-top-center relative mt-1.5 h-[var(--radix-navigation-menu-viewport-height)] w-full overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-lg data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-90 md:w-[var(--radix-navigation-menu-viewport-width)]",
        class.get()
    );

    view! {
        <div class=merged data-name="NavigationMenuViewport" />
    }
}