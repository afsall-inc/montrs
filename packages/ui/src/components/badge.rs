use leptos::prelude::*;

crate::variants! {
    Badge {
        base: "inline-flex items-center font-semibold rounded-md",
        variants: {
            variant: {
                Default: "bg-primary text-primary-foreground",
                Secondary: "bg-secondary text-secondary-foreground",
                Outline: "border border-border",
                Destructive: "bg-destructive text-destructive-foreground",
            },
            size: {
                Default: "px-2.5 py-0.5 text-xs",
                Sm: "px-1.5 py-0.5 text-[10px]",
                Lg: "px-3 py-1 text-sm",
            }
        },
        component: {
            element: span
        }
    }
}
