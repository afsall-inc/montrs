//! Structured notifications and the MontRS toaster viewport.

use crate::cn::*;
use leptos::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_NOTIFICATION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationLevel {
    #[default]
    Normal,
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub id: u64,
    pub level: NotificationLevel,
    pub title: String,
    pub description: Option<String>,
    pub dismissible: bool,
}

impl Notification {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: NEXT_NOTIFICATION_ID.fetch_add(1, Ordering::Relaxed),
            level: NotificationLevel::Normal,
            title: title.into(),
            description: None,
            dismissible: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct NotificationCenter {
    notifications: RwSignal<Vec<Notification>>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self { notifications: RwSignal::new(Vec::new()) }
    }

    pub fn notifications(&self) -> RwSignal<Vec<Notification>> {
        self.notifications
    }

    pub fn push(&self, notification: Notification) -> u64 {
        let id = notification.id;
        self.notifications.update(|items| items.push(notification));
        id
    }

    pub fn dismiss(&self, id: u64) {
        self.notifications.update(|items| items.retain(|item| item.id != id));
    }

    pub fn clear(&self) {
        self.notifications.set(Vec::new());
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}

pub fn provide_notification_center() -> NotificationCenter {
    let center = NotificationCenter::new();
    provide_context(center);
    center
}

pub fn use_notification_center() -> NotificationCenter {
    expect_context::<NotificationCenter>()
}

#[component]
pub fn Toaster(
    #[prop(optional, default = "bottom-right")]
    position: &'static str,
    #[prop(optional, default = 5)]
    visible_toasts: usize,
) -> impl IntoView {
    let center = use_notification_center();
    let notifications = center.notifications();
    let position_class = move || match position {
        "top-left" => "left-4 top-4",
        "top-right" => "right-4 top-4",
        "bottom-left" => "bottom-4 left-4",
        _ => "bottom-4 right-4",
    };

    view! {
        <div
            class=move || cn!("fixed z-50 flex w-[min( calc(100vw-2rem),24rem)] flex-col gap-2", position_class())
            aria-live="polite"
            aria-atomic="false"
            data-name="Toaster"
        >
            <For
                each=move || visible_notifications(notifications.get(), visible_toasts)
                key=|notification| notification.id
                children=move |notification| {
                    let id = notification.id;
                    let center = center;
                    view! {
                        <div
                            class=move || notification_class(notification.level)
                            role="status"
                            data-name="Notification"
                        >
                            <div class="min-w-0 flex-1">
                                <div class="font-medium">{notification.title.clone()}</div>
                                {notification.description.clone().map(|description| view! {
                                    <div class="mt-1 text-sm opacity-80">{description}</div>
                                })}
                            </div>
                            {notification.dismissible.then(|| view! {
                                <button
                                    type="button"
                                    class="ml-3 rounded px-2 py-1 text-sm opacity-70 hover:opacity-100"
                                    aria-label="Dismiss notification"
                                    on:click=move |_| center.dismiss(id)
                                >"×"</button>
                            })}
                        </div>
                    }
                }
            />
        </div>
    }
}

fn visible_notifications(
    notifications: Vec<Notification>,
    visible_toasts: usize,
) -> Vec<Notification> {
    notifications.into_iter().take(visible_toasts).collect()
}

fn notification_class(level: NotificationLevel) -> String {
    let tone = match level {
        NotificationLevel::Normal => "border-border bg-background text-foreground",
        NotificationLevel::Success => "border-green-500/40 bg-green-50 text-green-950 dark:bg-green-950 dark:text-green-50",
        NotificationLevel::Info => "border-blue-500/40 bg-blue-50 text-blue-950 dark:bg-blue-950 dark:text-blue-50",
        NotificationLevel::Warning => "border-yellow-500/40 bg-yellow-50 text-yellow-950 dark:bg-yellow-950 dark:text-yellow-50",
        NotificationLevel::Error => "border-red-500/40 bg-red-50 text-red-950 dark:bg-red-950 dark:text-red-50",
    };
    cn!("flex items-start rounded-lg border px-4 py-3 shadow-lg", tone)
}
