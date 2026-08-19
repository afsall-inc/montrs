//! SaaS web app library entry point.

use leptos::view;
use montrs_command::{Command, CommandRegistry};
use montrs_state::Store;
use montrs_table_core::{Row, Table};

/// The root application component.
pub fn app() -> impl leptos::IntoView {
    let mut registry = CommandRegistry::default();
    registry.register(Command::new("dashboard", "Dashboard"));
    let _store = Store::new(0_u32, |state: &u32, event: &u32| Ok(state + event));
    let _table = Table::new(vec![Row { id: "workspace".into(), value: "SaaS" }]);
    view! { <div>{"MontRS SaaS template"}</div> }
}