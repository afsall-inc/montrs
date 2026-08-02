use leptos::prelude::*;
use crate::cn::*;

crate::variants! {
    Callout {
        base: "relative w-full rounded-lg border-l-4 p-4",
        variants: {
            variant: {
                Default: "border-l-primary bg-muted/50 text-foreground",
                Info: "border-l-blue-500 bg-blue-50/50 text-foreground dark:bg-blue-950/20",
                Warning: "border-l-yellow-500 bg-yellow-50/50 text-foreground dark:bg-yellow-950/20",
                Error: "border-l-destructive bg-destructive/10 text-foreground",
                Success: "border-l-green-500 bg-green-50/50 text-foreground dark:bg-green-950/20",
            }
        }
    }
}

#[component]
pub fn Callout(
    #[prop(into, optional)] variant: Signal<CalloutVariant>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = CalloutClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };
    view! {
        <div class=merged data-name="Callout">
            {children()}
        </div>
    }
}