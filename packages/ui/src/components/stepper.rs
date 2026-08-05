use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn Stepper(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] current: usize,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("flex items-center gap-2", class.get());
    view! {
        <div class=merged data-name="Stepper">
            {children()}
        </div>
    }
}

#[component]
pub fn Step(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] index: usize,
    #[prop(optional)] active: bool,
    #[prop(optional)] completed: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "flex items-center gap-2 text-sm";
        let state = if completed {
            "text-primary"
        } else if active {
            "text-foreground font-medium"
        } else {
            "text-muted-foreground"
        };
        cn!(base, state, class.get())
    };
    view! {
        <div class=merged data-name="Step">
            <div class=move || {
                let base = "flex h-8 w-8 shrink-0 items-center justify-center rounded-full border-2 text-xs font-medium";
                let state = if completed { "bg-primary border-primary text-primary-foreground" } else if active { "border-primary text-primary" } else { "border-muted-foreground/30" };
                cn!(base, state)
            }>
                {if completed { "✓".to_string() } else { (index + 1).to_string() }}
            </div>
            {children()}
        </div>
    }
}
