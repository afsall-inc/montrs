use leptos::prelude::*;
use crate::cn::*;

/// Menu bar with items.
///
/// Renders a horizontal menu bar with dropdown triggers.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Menubar>
///         <MenubarMenu>
///             <MenubarTrigger>"File"</MenubarTrigger>
///             <MenubarContent>
///                 <MenubarItem>"New"</MenubarItem>
///             </MenubarContent>
///         </MenubarMenu>
///     </Menubar>
/// }
/// ```
#[component]
pub fn Menubar(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "flex h-10 items-center space-x-1 rounded-md border bg-background p-1",
        class.get()
    );

    view! {
        <div class=merged data-name="Menubar">
            {children()}
        </div>
    }
}

/// Individual menu in the menubar.
#[component]
pub fn MenubarMenu(
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="MenubarMenu">
            {children()}
        </div>
    }
}

/// Trigger button for a menu.
#[component]
pub fn MenubarTrigger(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("MenubarTrigger must be inside MenubarMenu");
    let toggle = move |_| open.update(|v| *v = !*v);

    let merged = move || cn!(
        "flex cursor-default select-none items-center rounded-sm px-3 py-1.5 text-sm font-medium outline-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground",
        class.get()
    );

    view! {
        <button type="button" class=merged on:click=toggle data-name="MenubarTrigger">
            {children()}
        </button>
    }
}

/// Menu content dropdown.
#[component]
pub fn MenubarContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<RwSignal<bool>>()
        .expect("MenubarContent must be inside MenubarMenu");

    let merged = move || cn!(
        "z-50 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95",
        class.get()
    );

    let close = move |_| open.set(false);

    view! {
        <div
            class=merged
            data-state=move || if open.get() { "open" } else { "closed" }
            hidden=move || !open.get()
            data-name="MenubarContent"
        >
            {children()}
        </div>
        {move || if open.get() {
            view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=close
                    data-name="MenubarBackdrop"
                ></div>
            }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

/// Menubar item.
#[component]
pub fn MenubarItem(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!(
        "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        class.get()
    );

    view! {
        <div class=merged role="menuitem" data-name="MenubarItem">
            {children()}
        </div>
    }
}

/// Menubar separator.
#[component]
pub fn MenubarSeparator(
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let merged = move || cn!("-mx-1 my-1 h-px bg-border", class.get());

    view! {
        <div class=merged data-name="MenubarSeparator" />
    }
}

/// Menubar shortcut.
#[component]
pub fn MenubarShortcut(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("ml-auto text-xs tracking-widest text-muted-foreground", class.get());

    view! {
        <span class=merged data-name="MenubarShortcut">
            {children()}
        </span>
    }
}