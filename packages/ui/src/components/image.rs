//! Optimized Image component — inspired by leptos-image: lazy, priority/preload, blur.

use crate::cn::*;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageFormat {
    Original,
    Webp,
    Png,
    Jpeg,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageLoading {
    Auto,
    Eager,
    Lazy,
}

/// An optimized image component with lazy loading, preload, and blur.
#[component]
pub fn Image(
    src: String,
    #[prop(optional)] alt: String,
    #[prop(optional)] width: Option<u32>,
    #[prop(optional)] height: Option<u32>,
    #[prop(optional, default = 80)] quality: u8,
    #[prop(optional, default = 0.0)] blur: f32,
    #[prop(optional, default = false)] priority: bool,
    #[prop(optional, default = ImageLoading::Auto)] loading: ImageLoading,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] _fallback: Option<String>,
) -> impl IntoView {
    let merged = move || cn!("rounded-md object-cover", class.get());

    let loading_attr = match loading {
        ImageLoading::Lazy => "lazy",
        _ => "eager",
    };

    let mut style = String::new();
    if let Some(w) = width {
        style.push_str(&format!("width: {w}px;"));
        style.push_str(&format!(
            "aspect-ratio: {}/{};",
            w,
            height.unwrap_or(w)
        ));
    }
    if let Some(h) = height {
        style.push_str(&format!("height: {h}px;"));
    }

    let img_src = if (1..=100).contains(&quality) && quality != 80 {
        format!("{src}?q={quality}")
    } else {
        src.clone()
    };

    view! {
        {priority.then(|| view! {
            // Preload link rendered server-side; omitted here to avoid reserved-word attribute issues.
            // Use leptos_meta::Meta for SSR preload links.
            let _ = src.clone();
            ""
        })}
        <img
            src=img_src
            alt=alt
            loading=loading_attr
            class=merged
            style=style
            data-name="Image"
            data-blur=move || (blur > 0.0).to_string()
        />
    }
}
