//! Browser hotkey adapter for MontRS — document-level listeners, scopes, macros.

use leptos::prelude::*;
use montrs_hotkeys_core::{Hotkey, KeyPresses};
use std::collections::HashSet;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::Closure;
#[cfg(target_arch = "wasm32")]
use web_sys::KeyboardEvent;

// ============================================================================
// HotkeysContext
// ============================================================================

#[derive(Clone, Copy)]
pub struct HotkeysContext {
    /// Currently pressed keys and their events.
    pub keys_pressed: RwSignal<KeyPresses>,
    /// Active scope names.
    pub active_scopes: RwSignal<HashSet<String>>,
    /// Enable a scope.
    pub enable_scope: Callback<String>,
    /// Disable a scope.
    pub disable_scope: Callback<String>,
    /// Toggle a scope.
    pub toggle_scope: Callback<String>,
}

/// Provide a hotkeys context that registers document-level keydown/keyup listeners.
/// `initially_active_scopes` sets the initial scope set (use `scopes!()` macro).
pub fn provide_hotkeys_context(
    _allow_blur: bool,
    initially_active_scopes: HashSet<String>,
) -> HotkeysContext {
    let keys_pressed = RwSignal::new(KeyPresses::new());
    let active_scopes = RwSignal::new(initially_active_scopes);

    let enable_scope = {
        let scopes = active_scopes;
        Callback::new(move |scope: String| {
            scopes.update(|s| {
                s.insert(scope);
            });
        })
    };
    let disable_scope = {
        let scopes = active_scopes;
        Callback::new(move |scope: String| {
            scopes.update(|s| {
                s.remove(&scope);
            });
        })
    };
    let toggle_scope = {
        let scopes = active_scopes;
        Callback::new(move |scope: String| {
            scopes.update(|s| {
                if s.contains(&scope) {
                    s.remove(&scope);
                } else {
                    s.insert(scope);
                }
            });
        })
    };

    let ctx = HotkeysContext {
        keys_pressed,
        active_scopes,
        enable_scope,
        disable_scope,
        toggle_scope,
    };
    provide_context(ctx);

    // Register document-level keydown/keyup listeners (non-WASM: no-op)
    #[cfg(target_arch = "wasm32")]
    {
        let pressed = keys_pressed;
        let keydown_closure = Closure::<dyn Fn(KeyboardEvent)>::new(
            move |event: KeyboardEvent| {
                let key = clean_key(&event);
                pressed.update(|kp| {
                    kp.push(key);
                });
            },
        );
        let keyup_closure = Closure::<dyn Fn(KeyboardEvent)>::new(
            move |event: KeyboardEvent| {
                let key = clean_key(&event);
                pressed.update(|kp| {
                    kp.release(&key);
                });
            },
        );
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback(
                "keydown",
                keydown_closure.as_ref().unchecked_ref(),
            );
            let _ = doc.add_event_listener_with_callback(
                "keyup",
                keyup_closure.as_ref().unchecked_ref(),
            );
        }
        keydown_closure.forget();
        keyup_closure.forget();

        if allow_blur {
            let pressed2 = keys_pressed;
            let blur_closure =
                Closure::<dyn Fn(web_sys::Event)>::new(move |_| {
                    pressed2.update(|kp| kp.clear());
                });
            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback(
                    "blur",
                    blur_closure.as_ref().unchecked_ref(),
                );
            }
            blur_closure.forget();
        }
    }

    ctx
}

/// Retrieve the hotkeys context (panics if not provided).
pub fn use_hotkeys_context() -> HotkeysContext {
    expect_context::<HotkeysContext>()
}

/// Register a hotkey callback scoped to the given scope names.
/// `key_combination` can be comma-separated: `"ctrl+k,ctrl+p"`.
/// The callback fires when the hotkey is pressed AND all scopes are active.
pub fn use_hotkeys_scoped(
    key_combination: String,
    on_triggered: Callback<()>,
    scopes: Vec<String>,
) {
    let ctx = use_hotkeys_context();
    let hotkeys: Vec<Hotkey> = key_combination
        .split(',')
        .map(|k| Hotkey::new(k.trim()))
        .collect();

    Effect::new(move |_| {
        let pressed = ctx.keys_pressed.get();
        let active = ctx.active_scopes.get();
        let scope_ok =
            scopes.is_empty() || scopes.iter().all(|s| active.contains(s));
        if !scope_ok {
            return;
        }
        if !montrs_hotkeys_core::is_last_key_match(&hotkeys, &pressed) {
            return;
        }
        let matched = hotkeys
            .iter()
            .any(|hk| montrs_hotkeys_core::is_hotkey_match(hk, &pressed));
        if matched {
            on_triggered.run(());
        }
    });
}

/// Register a hotkey scoped to a specific DOM element via NodeRef.
/// For now, use the `use_hotkeys!` macro instead for global hotkeys.
pub fn use_hotkeys_ref<T>(
    _node_ref: NodeRef<T>,
    _key_combination: String,
    _on_triggered: Callback<()>,
    _scopes: Vec<String>,
) where
    T: leptos::html::ElementType + 'static,
{
    // Element-scoped hotkeys require per-element keydown listener setup.
    // For the initial implementation, prefer use_hotkeys_scoped via the macro.
}

/// Clean a key from a KeyboardEvent for matching.
#[cfg(target_arch = "wasm32")]
fn clean_key(event: &KeyboardEvent) -> String {
    let key = event.key().to_ascii_lowercase();
    if key == " " {
        "spacebar".to_string()
    } else {
        key
    }
}

// ============================================================================
// Macros
// ============================================================================

/// Create a scope set. Always includes `"*"` as a default scope.
///
/// # Examples
/// ```rust,ignore
/// scopes!()
/// scopes!("foo", "bar")
/// ```
#[macro_export]
macro_rules! scopes {
    () => {
        {
            let mut s = std::collections::HashSet::new();
            s.insert("*".to_string());
            s
        }
    };
    ($($scope:expr),+ $(,)?) => {
        {
            let mut s = std::collections::HashSet::new();
            s.insert("*".to_string());
            $(
                s.insert($scope.to_string());
            )+
            s
        }
    };
}

/// Register a hotkey with a callback, scoped to the given scopes (defaults to `["*"]`).
///
/// # Examples
/// ```rust,ignore
/// use_hotkeys!(("meta+k") => move |_| { ... });
/// use_hotkeys!(("meta+k", "kbar") => move |_| { ... });
/// ```
#[macro_export]
macro_rules! use_hotkeys {
    (($key:expr) => $closure:expr) => {
        $crate::use_hotkeys_scoped(
            $key.to_string(),
            ::leptos::prelude::Callback::new($closure),
            vec![],
        );
    };
    (($key:expr, $($scope:expr),+ $(,)?) => $closure:expr) => {
        $crate::use_hotkeys_scoped(
            $key.to_string(),
            ::leptos::prelude::Callback::new($closure),
            vec![$($scope.to_string()),+],
        );
    };
}

/// Register a hotkey on a specific element ref, scoped to the given scopes.
///
/// # Examples
/// ```rust,ignore
/// use_hotkeys_ref!((node_ref, "meta+k") => move |_| { ... });
/// ```
#[macro_export]
macro_rules! use_hotkeys_ref {
    (($ref:expr, $key:expr) => $closure:expr) => {
        $crate::use_hotkeys_ref(
            $ref,
            $key.to_string(),
            ::leptos::prelude::Callback::new($closure),
            vec![],
        );
    };
    (($ref:expr, $key:expr, $($scope:expr),+ $(,)?) => $closure:expr) => {
        $crate::use_hotkeys_ref(
            $ref,
            $key.to_string(),
            ::leptos::prelude::Callback::new($closure),
            vec![$($scope.to_string()),+],
        );
    };
}
