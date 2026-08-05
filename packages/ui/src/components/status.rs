use crate::cn::*;
use leptos::prelude::*;

crate::variants! {
    Status {
        base: "inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium",
        variants: {
            variant: {
                Active: "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400",
                Inactive: "bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400",
                Pending: "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400",
                Error: "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400",
            }
        }
    }
}

#[component]
pub fn Status(
    #[prop(into, optional)] variant: Signal<StatusVariant>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] label: Option<String>,
) -> impl IntoView {
    let merged = move || {
        let v = variant.try_get().unwrap_or_default();
        let c = StatusClass { variant: v };
        c.with_class(class.try_get().unwrap_or_default())
    };
    view! {
        <span class=merged data-name="Status">
            <span class="h-1.5 w-1.5 rounded-full bg-current" />
            {label}
        </span>
    }
}
