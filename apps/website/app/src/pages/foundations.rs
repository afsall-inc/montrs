use leptos::prelude::*;
use montrs_table_core::{Row, SortDirection, Table};
use montrs_ui::{
    components::{
        button::Button,
        card::Card,
        toaster::{
            Notification, NotificationLevel, Toaster,
            provide_notification_center,
        },
    },
    prelude::*,
};

#[component]
pub fn Foundations() -> impl IntoView {
    let center = provide_notification_center();
    let count = RwSignal::new(0_u32);
    let mut table = Table::new(vec![
        Row {
            id: "state".into(),
            value: "montrs-state",
        },
        Row {
            id: "content".into(),
            value: "montrs-content",
        },
        Row {
            id: "table".into(),
            value: "montrs-table-core",
        },
        Row {
            id: "hotkeys".into(),
            value: "montrs-hotkeys-core",
        },
    ]);
    table.sort_by("package".into(), SortDirection::Ascending, |left, right| {
        left.cmp(right)
    });
    let packages = table.rows().to_vec();

    view! {
        <div class="mx-auto max-w-5xl px-6 py-12 lg:px-8">
            <div class="mb-10">
                <p class="text-sm font-medium text-primary">"MontRS foundations"</p>
                <h1 class="mt-2 text-3xl font-bold tracking-tight">"State, content, tables, and hotkeys"</h1>
                <p class="mt-3 max-w-2xl text-muted-foreground">
                    "These examples use the Rust-native foundation packages that power MontRS applications."
                </p>
            </div>

            <div class="grid gap-6 md:grid-cols-2">
                <Card>
                    <div class="p-6">
                        <h2 class="text-xl font-semibold">"Notification center"</h2>
                        <p class="mt-2 text-sm text-muted-foreground">"Structured state rendered by the MontRS toaster."</p>
                        <Button class="mt-4" on:click=move |_| {
                            let next = count.get() + 1;
                            count.set(next);
                            let mut notification = Notification::new(format!("Notification {next}"));
                            notification.level = NotificationLevel::Success;
                            notification.description = Some("Created from a typed Rust notification model.".into());
                            center.push(notification);
                        }>
                            "Show notification"
                        </Button>
                    </div>
                </Card>

                <Card>
                    <div class="p-6">
                        <h2 class="text-xl font-semibold">"Headless table model"</h2>
                        <p class="mt-2 text-sm text-muted-foreground">"Stable IDs and deterministic row state."</p>
                        <div class="mt-4 overflow-hidden rounded-md border">
                            <TableHeader />
                            {packages.into_iter().map(|row| view! {
                                <div class="grid grid-cols-2 border-t px-3 py-2 text-sm">
                                    <span>{row.id}</span>
                                    <span class="text-muted-foreground">{row.value}</span>
                                </div>
                            }).collect_view()}
                        </div>
                    </div>
                </Card>
            </div>
        </div>
        <Toaster />
    }
}

#[component]
fn TableHeader() -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 bg-muted px-3 py-2 text-xs font-medium uppercase tracking-wide">
            <span>"ID"</span>
            <span>"Package"</span>
        </div>
    }
}
