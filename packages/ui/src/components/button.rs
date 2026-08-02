use leptos::prelude::*;
use crate::cn::*;

crate::variants! {
    Button {
        base: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        variants: {
            variant: {
                Default: "bg-primary text-primary-foreground hover:bg-primary/90",
                Secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                Outline: "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
                Ghost: "hover:bg-accent hover:text-accent-foreground",
                Destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            },
            size: {
                Default: "h-10 px-4 py-2",
                Sm: "h-9 rounded-md px-3",
                Lg: "h-11 rounded-md px-8",
                Icon: "h-10 w-10",
            }
        },
        component: {
            element: button
        }
    }
}