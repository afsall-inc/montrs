use crate::cn::*;
use leptos::prelude::*;

/// Progress bar component.
///
/// Renders a horizontal progress indicator.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Progress value=75 max=100 />
/// }
/// ```
#[component]
pub fn Progress(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] value: f64,
    #[prop(optional)] max: f64,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "relative h-4 w-full overflow-hidden rounded-full bg-secondary",
            class.get()
        )
    };

    let pct = if max > 0.0 {
        (value / max * 100.0).min(100.0)
    } else {
        0.0
    };
    let indicator_style = format!("transform: translateX(-{}%)", 100.0 - pct);

    view! {
        <div
            class=merged
            role="progressbar"
            aria-valuenow=value as i64
            aria-valuemin=0
            aria-valuemax=max as i64
            data-name="Progress"
        >
            <div
                class="h-full w-full flex-1 bg-primary transition-all duration-300 ease-in-out"
                style=indicator_style
                data-name="ProgressIndicator"
            />
        </div>
    }
}
