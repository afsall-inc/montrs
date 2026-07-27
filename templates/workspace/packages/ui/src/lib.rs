//! Shared UI components for the workspace.

use leptos::prelude::*;
use tailwind_fuse::*;

/// A reusable button component with type-safe variants.
#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeSignal<ButtonVariant>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    on_click: impl Fn(ev::MouseEvent) + 'static,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        let btn = ButtonClass {
            variant: variant.get(),
            size: size.get(),
        };
        btn.with_class(class.get())
    });

    view! {
        <button class=class on:click=on_click>
            {children()}
        </button>
    }
}

#[derive(TwClass, Clone, Copy)]
#[tw(class = "px-6 py-2 rounded-lg font-medium transition-colors focus:outline-none focus:ring-2")]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(TwVariant, Clone, Copy, Default)]
pub enum ButtonVariant {
    #[tw(default, class = "bg-blue-600 hover:bg-blue-500 text-white focus:ring-blue-400")]
    Primary,
    #[tw(class = "bg-gray-600 hover:bg-gray-500 text-white focus:ring-gray-400")]
    Secondary,
    #[tw(class = "bg-transparent border border-gray-500 hover:bg-gray-800 text-white focus:ring-gray-400")]
    Outline,
}

#[derive(TwVariant, Clone, Copy, Default)]
pub enum ButtonSize {
    #[tw(default, class = "text-base")]
    Medium,
    #[tw(class = "text-sm px-4 py-1")]
    Small,
    #[tw(class = "text-lg px-8 py-3")]
    Large,
}
