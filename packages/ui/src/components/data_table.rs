use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn DataTable(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("w-full text-sm", class.get());
    view! {
        <div class="relative w-full overflow-auto rounded-md border">
            <table class=merged data-name="DataTable">
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn DataTableHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("bg-muted/50 [&_tr]:border-b", class.get());
    view! {
        <thead class=merged data-name="DataTableHeader">
            {children()}
        </thead>
    }
}

#[component]
pub fn DataTableBody(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("[&_tr:last-child]:border-0", class.get());
    view! {
        <tbody class=merged data-name="DataTableBody">
            {children()}
        </tbody>
    }
}

#[component]
pub fn DataTableRow(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!("border-b transition-colors hover:bg-muted/50", class.get())
    };
    view! {
        <tr class=merged data-name="DataTableRow">
            {children()}
        </tr>
    }
}

#[component]
pub fn DataTableHead(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] sortable: bool,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        let base = "h-10 px-4 text-left align-middle font-medium \
                    text-muted-foreground";
        let sort = if sortable {
            "cursor-pointer hover:text-foreground"
        } else {
            ""
        };
        cn!(base, sort, class.get())
    };
    view! {
        <th class=merged data-name="DataTableHead">
            {children()}
        </th>
    }
}

#[component]
pub fn DataTableCell(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("p-4 align-middle", class.get());
    view! {
        <td class=merged data-name="DataTableCell">
            {children()}
        </td>
    }
}
