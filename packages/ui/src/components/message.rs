use leptos::prelude::*;
use crate::cn::*;

crate::variants! {
    Message {
        base: "rounded-lg border px-4 py-3 text-sm",
        variants: {
            variant: {
                Default: "bg-background text-foreground",
                Info: "border-blue-200 bg-blue-50 text-blue-800 dark:border-blue-800 dark:bg-blue-950 dark:text-blue-200",
                Success: "border-green-200 bg-green-50 text-green-800 dark:border-green-800 dark:bg-green-950 dark:text-green-200",
                Warning: "border-yellow-200 bg-yellow-50 text-yellow-800 dark:border-yellow-800 dark:bg-yellow-950 dark:text-yellow-200",
                Error: "border-destructive/50 bg-destructive/10 text-destructive",
            }
        }
    }
}

#[component]
pub fn Message(
    #[prop(into, optional)] variant: Signal<MessageVariant>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = MessageClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };
    view! {
        <div class=merged data-name="Message">
            {children()}
        </div>
    }
}