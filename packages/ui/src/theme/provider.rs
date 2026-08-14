use leptos::prelude::*;

/// Theme mode for the application.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        match self {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => {
                #[cfg(target_arch = "wasm32")]
                {
                    web_sys::window()
                        .and_then(|w| {
                            w.match_media("(prefers-color-scheme: dark)")
                                .ok()?
                        })
                        .map(|m| m.matches())
                        .unwrap_or(false)
                }
                #[cfg(not(target_arch = "wasm32"))]
                false
            }
        }
    }
}

/// Provides theme context and dark mode toggling.
///
/// Wraps the application and applies the `.dark` class to `<html>`.
/// Supports `localStorage` persistence for user preference.
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(load_theme_preference());

    let is_dark = Memo::new(move |_| theme.get().is_dark());

    Effect::new(move |_| {
        if let Some(document) = document()
            && let Some(html) = document.document_element()
        {
            if is_dark.get() {
                let _ = html.class_list().add_1("dark");
            } else {
                let _ = html.class_list().remove_1("dark");
            }
        }
    });

    Effect::new(move |_| {
        save_theme_preference(theme.get());
    });

    provide_context(theme);

    view! {
        {children()}
    }
}

/// Reads the current theme mode from the reactive context.
pub fn use_theme() -> RwSignal<ThemeMode> {
    use_context::<RwSignal<ThemeMode>>().expect("ThemeProvider not found")
}

/// Toggles between light/dark/system modes.
pub fn toggle_theme() {
    let theme = use_theme();
    theme.update(|t| {
        *t = match t {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::System,
            ThemeMode::System => ThemeMode::Light,
        }
    });
}

fn load_theme_preference() -> ThemeMode {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) =
            web_sys::window().and_then(|w| w.local_storage().ok()?)
        {
            if let Ok(Some(value)) = storage.get_item("montrs-theme") {
                match value.as_str() {
                    "light" => return ThemeMode::Light,
                    "dark" => return ThemeMode::Dark,
                    _ => {}
                }
            }
        }
    }
    ThemeMode::System
}

#[allow(unused_variables)]
fn save_theme_preference(mode: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) =
            web_sys::window().and_then(|w| w.local_storage().ok()?)
        {
            let value = match mode {
                ThemeMode::Light => "light",
                ThemeMode::Dark => "dark",
                ThemeMode::System => "system",
            };
            let _ = storage.set_item("montrs-theme", value);
        }
    }
}

fn document() -> Option<web_sys::Document> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()?.document()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}
