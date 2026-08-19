//! Structured notifications, ToastId, and the MontRS toaster.
//! Inspired by leptos_toaster (sonner-style) — structured state, no ViewFn in storage.

use crate::cn::*;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

// ============================================================================
// ToastId
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ToastId([u8; 64]);
static TOAST_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl ToastId {
    pub fn new() -> Self {
        let count = TOAST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut bytes = [0u8; 64];
        for (i, b) in count.to_string().bytes().enumerate().take(64) { bytes[i] = b; }
        Self(bytes)
    }
    pub fn from_usize(number: usize) -> Self {
        let mut bytes = [0u8; 64];
        for (i, b) in number.to_string().bytes().enumerate().take(64) { bytes[i] = b; }
        Self(bytes)
    }
    pub fn to_decodable_string(&self) -> String {
        self.0.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(",")
    }
    pub fn decode_string(s: &str) -> Self {
        let mut bytes = [0u8; 64];
        for (i, part) in s.split(',').enumerate().take(64) {
            if let Ok(n) = part.parse::<u8>() { bytes[i] = n; }
        }
        Self(bytes)
    }
}
impl Default for ToastId { fn default() -> Self { Self::new() } }

// ============================================================================
// ToasterPosition
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum ToasterPosition {
    TopLeft, TopCenter, #[default] TopRight, BottomRight, BottomCenter, BottomLeft,
}
impl ToasterPosition {
    pub fn x(&self) -> &'static str {
        match self { Self::TopLeft | Self::BottomLeft => "left", Self::TopCenter | Self::BottomCenter => "center", _ => "right" }
    }
    pub fn y(&self) -> &'static str {
        match self { Self::TopLeft | Self::TopCenter | Self::TopRight => "top", _ => "bottom" }
    }
}

// ============================================================================
// ToastOptions
// ============================================================================

#[derive(Clone, Debug)]
pub struct ToastOptions {
    pub dismissible: bool,
    pub duration: Option<Duration>,
    pub position: Option<ToasterPosition>,
}
impl Default for ToastOptions {
    fn default() -> Self { Self { dismissible: true, duration: None, position: None } }
}

// ============================================================================
// Notification model
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotificationLevel { Normal, Success, Info, Warning, Error }

#[derive(Clone, Debug)]
pub struct Notification {
    pub id: ToastId,
    pub level: NotificationLevel,
    pub title: String,
    pub description: Option<String>,
    pub dismissible: bool,
    pub duration: Duration,
    pub position: ToasterPosition,
}
impl Notification {
    pub fn new(title: impl Into<String>) -> Self {
        Self { id: ToastId::new(), level: NotificationLevel::Normal, title: title.into(), description: None, dismissible: true, duration: Duration::from_secs(4), position: ToasterPosition::default() }
    }
    pub fn with_level(mut self, level: NotificationLevel) -> Self { self.level = level; self }
    pub fn with_description(mut self, desc: impl Into<String>) -> Self { self.description = Some(desc.into()); self }
    pub fn with_duration(mut self, duration: Duration) -> Self { self.duration = duration; self }
}

// ============================================================================
// NotificationCenter
// ============================================================================

#[derive(Clone, Copy)]
pub struct NotificationCenter { notifications: RwSignal<Vec<Notification>> }
impl NotificationCenter {
    pub fn new() -> Self { Self { notifications: RwSignal::new(Vec::new()) } }
    pub fn notifications(&self) -> RwSignal<Vec<Notification>> { self.notifications }
    pub fn push(&self, notification: Notification) -> ToastId {
        let id = notification.id;
        self.notifications.update(|items| items.push(notification));
        id
    }
    pub fn dismiss(&self, id: ToastId) {
        self.notifications.update(|items| items.retain(|item| item.id != id));
    }
    pub fn clear(&self) { self.notifications.set(Vec::new()); }
}
impl Default for NotificationCenter { fn default() -> Self { Self::new() } }

pub fn provide_notification_center() -> NotificationCenter {
    let center = NotificationCenter::new();
    provide_context(center);
    center
}
pub fn use_notification_center() -> NotificationCenter { expect_context::<NotificationCenter>() }
pub fn dismiss_toast(id: ToastId) {
    if let Some(center) = use_context::<NotificationCenter>() { center.dismiss(id); }
}

// ============================================================================
// Toaster
// ============================================================================

#[allow(unused_variables)]
#[component]
pub fn Toaster(
    #[prop(optional, default = ToasterPosition::default())] position: ToasterPosition,
    #[prop(optional, default = false)] expand: bool,
    #[prop(optional, default = 4000u64)] duration_ms: u64,
    #[prop(optional, default = 14usize)] gap: usize,
    #[prop(optional, default = 3usize)] visible_toasts: usize,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let center = use_notification_center();
    let notifications = center.notifications();
    let is_expanded = RwSignal::new(expand);
    #[allow(unused_variables)]
    let default_duration = Duration::from_millis(duration_ms);

    Effect::new(move |_| { if notifications.get().len() <= 1 { is_expanded.set(false); } });

let x_style = move || -> String {
        match position.x() {
            "center" => "left: 50%; transform: translateX(-50%)".to_string(),
            "left" => "left: 16px".to_string(),
            _ => "right: 16px".to_string(),
        }
    };
    let y_style = move || if position.y() == "top" { "top: 16px" } else { "bottom: 16px" };

    let style = move || format!("{}; {}; --gap: {}px; --width: 356px; z-index: 9999", x_style(), y_style(), gap);

    view! {
        <div
            style=style
            class="fixed flex flex-col"
            role="list"
            aria-live="polite"
            data-name="Toaster"
            on:mouseenter=move |_| is_expanded.set(true)
            on:mouseleave=move |_| is_expanded.set(false)
        >
            {children.map(|c| c())}
            <For
                each=move || notifications.get()
                key=|notification| notification.id
                children=move |notification| {
                    let id = notification.id;
                    let level = notification.level.clone();
                    let title = notification.title.clone();
                    let description = notification.description.clone();
                    let is_dismissible = notification.dismissible;
                    view! {
                        <ToastItem
                            id=id
                            level=level
                            title=title
                            description=description
                            is_dismissible=is_dismissible
                            remove_toast=Callback::new(move |_| center.dismiss(id))
                        />
                    }
                }
            />
        </div>
    }
}

// ============================================================================
// ToastItem
// ============================================================================

#[component]
fn ToastItem(
    id: ToastId,
    level: NotificationLevel,
    title: String,
    description: Option<String>,
    is_dismissible: bool,
    remove_toast: Callback<ToastId>,
) -> impl IntoView {
    let removed = RwSignal::new(false);

    let bg = match level {
        NotificationLevel::Normal => "bg-background text-foreground border-border",
        NotificationLevel::Success => "bg-green-50 dark:bg-green-950 text-green-900 dark:text-green-100 border-green-400",
        NotificationLevel::Info => "bg-blue-50 dark:bg-blue-950 text-blue-900 dark:text-blue-100 border-blue-400",
        NotificationLevel::Warning => "bg-yellow-50 dark:bg-yellow-950 text-yellow-900 dark:text-yellow-100 border-yellow-400",
        NotificationLevel::Error => "bg-red-50 dark:bg-red-950 text-red-900 dark:text-red-100 border-red-400",
    };

    let merged = move || {
        let base = "flex items-start rounded-lg border px-4 py-3 shadow-lg transition-all duration-300 w-[356px]";
        let state = if removed.get() { "opacity-0 h-0 overflow-hidden py-0 border-0" } else { "opacity-100" };
        cn!(base, bg, state)
    };

    let handle_dismiss = move |_| {
        removed.set(true);
        // delay removal to allow animation
        let cb = remove_toast;
        set_timeout(move || cb.run(id), Duration::from_millis(200));
    };

    view! {
        <div
            class=merged
            role="listitem"
            data-name="ToastItem"
        >
            <div class="min-w-0 flex-1">
                <div class="text-sm font-medium">{title}</div>
                {description.map(|d| view! { <div class="mt-1 text-sm opacity-80">{d}</div> })}
            </div>
            {is_dismissible.then(|| view! {
                <button
                    type="button"
                    class="ml-3 shrink-0 rounded-full p-1 opacity-60 hover:opacity-100 transition-opacity"
                    aria-label="Dismiss"
                    on:click=handle_dismiss
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
                </button>
            })}
        </div>
    }
}
