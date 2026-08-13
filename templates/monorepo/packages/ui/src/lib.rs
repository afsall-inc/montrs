use leptos::prelude::*;
use montrs_ui::prelude::*;
use montrs_icons::*;

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
        cn!(btn.to_class(), class.get())
    });

    view! {
        <button class=class on:click=on_click>
            {children()}
        </button>
    }
}

variants! {
    Button {
        base: "inline-flex items-center justify-center rounded-lg font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
        variants: {
            variant: {
                Primary: "bg-primary text-primary-foreground hover:bg-primary/90",
                Secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                Outline: "border border-border bg-transparent hover:bg-accent hover:text-accent-foreground",
            },
            size: {
                Default: "h-10 px-4 py-2 text-sm",
                Sm: "h-9 rounded-md px-3 text-xs",
                Lg: "h-11 rounded-md px-8 text-base",
            }
        },
        component: {
            element: button
        }
    }
}