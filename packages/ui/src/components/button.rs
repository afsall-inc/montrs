use crate::cn::*;
use leptos::prelude::*;

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
        }
    }
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] data_name: Option<String>,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, optional)] aria_pressed: Option<bool>,
    #[prop(optional)] disabled: bool,
    children: Children,
) -> impl IntoView {
    let computed_class = move || {
        let v = variant.try_get().unwrap_or_default();
        let s = size.try_get().unwrap_or_default();
        let component_class = ButtonClass {
            variant: v,
            size: s,
        };
        component_class.with_class(class.try_get().unwrap_or_default())
    };

    let data_name = data_name.unwrap_or_else(|| "Button".to_string());

    view! {
        <button
            type="button"
            class=computed_class
            data-name=data_name
            disabled=disabled
            aria-disabled=disabled
            aria-label=aria_label
            aria-pressed=aria_pressed
        >
            {children()}
        </button>
    }
}
