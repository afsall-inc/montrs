use leptos::prelude::*;
use crate::value::MotionValue;

/// Animated wrapper component for HTML elements.
///
/// Like Framer Motion's `motion.div` — wraps a child element with
/// hover-triggered spring animations for scale, opacity, and rotation.
#[component]
pub fn Animated(
    children: Children,
    #[prop(optional, into)]
    hover_scale: Option<f64>,
    #[prop(optional, into)]
    hover_opacity: Option<f64>,
    #[prop(optional, into)]
    hover_rotate: Option<f64>,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let scale = MotionValue::new(1.0);
    let opacity = MotionValue::new(1.0);
    let rotate = MotionValue::new(0.0);

    let target_scale = hover_scale.unwrap_or(1.0);
    let target_opacity = hover_opacity.unwrap_or(1.0);
    let target_rotate = hover_rotate.unwrap_or(0.0);

    let on_enter = {
        let scale = scale.clone();
        let opacity = opacity.clone();
        let rotate = rotate.clone();
        move |_: leptos::ev::MouseEvent| {
            if target_scale != 1.0 {
                scale.animate_to(target_scale, 300.0, 20.0, 1.0);
            }
            if target_opacity != 1.0 {
                opacity.animate_to(target_opacity, 300.0, 20.0, 1.0);
            }
            if target_rotate != 0.0 {
                rotate.animate_to(target_rotate, 300.0, 20.0, 1.0);
            }
        }
    };

    let on_leave = {
        let scale = scale.clone();
        let opacity = opacity.clone();
        let rotate = rotate.clone();
        move |_: leptos::ev::MouseEvent| {
            scale.animate_to(1.0, 300.0, 20.0, 1.0);
            opacity.animate_to(1.0, 300.0, 20.0, 1.0);
            rotate.animate_to(0.0, 300.0, 20.0, 1.0);
        }
    };

    let class_val = class.unwrap_or_default();
    let style = move || {
        format!(
            "transform: translate3d(0px, 0px, 0px) scale({}) rotate({}deg); opacity: {}; transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.2s ease; will-change: transform, opacity;",
            scale.get(),
            rotate.get(),
            opacity.get()
        )
    };

    view! {
        <div
            class=class_val
            style=style
            on:mouseenter=on_enter
            on:mouseleave=on_leave
        >
            {children()}
        </div>
    }
}