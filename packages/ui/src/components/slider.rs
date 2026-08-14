use crate::cn::*;
use leptos::prelude::*;

/// Range slider component.
///
/// Renders a styled range input slider.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Slider min=0 max=100 value=slider_value />
/// }
/// ```
#[component]
pub fn Slider(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] min: f64,
    #[prop(optional)] max: f64,
    #[prop(optional)] step: f64,
    #[prop(into, optional)] value: RwSignal<f64>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "relative flex w-full touch-none select-none items-center",
            class.get()
        )
    };

    let pct = move || {
        let range = max - min;
        if range > 0.0 {
            ((value.get() - min) / range * 100.0).min(100.0)
        } else {
            0.0
        }
    };

    let on_input = move |ev: leptos::ev::Event| {
        let target = event_target_value(&ev);
        if let Ok(v) = target.parse::<f64>() {
            value.set(v);
        }
    };

    let _track_style = move || format!("left: 0%; right: {}%;", 100.0 - pct());
    let range_style = move || format!("left: 0%; width: {}%;", pct());

    view! {
        <div class=merged data-name="Slider">
            <div class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary">
                <div
                    class="absolute h-full bg-primary"
                    style=range_style
                    data-name="SliderRange"
                />
            </div>
            <input
                type="range"
                min=min
                max=max
                step=step
                value=move || value.get()
                disabled=disabled
                on:input=on_input
                class="absolute inset-0 h-full w-full cursor-pointer opacity-0"
                data-name="SliderInput"
            />
            <div
                class="absolute h-5 w-5 rounded-full border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
                style=move || format!("left: calc({}% - 10px);", pct())
                data-name="SliderThumb"
            />
        </div>
    }
}
