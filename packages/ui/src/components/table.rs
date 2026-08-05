use crate::cn::*;
use leptos::prelude::*;

/// Table component with header, body, row, and cell.
///
/// Renders a styled HTML table.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Table>
///         <TableHeader>
///             <TableRow>
///                 <TableHead>"Name"</TableHead>
///                 <TableHead>"Email"</TableHead>
///             </TableRow>
///         </TableHeader>
///         <TableBody>
///             <TableRow>
///                 <TableCell>"John"</TableCell>
///                 <TableCell>"john@example.com"</TableCell>
///             </TableRow>
///         </TableBody>
///     </Table>
/// }
/// ```
#[component]
pub fn Table(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("w-full caption-bottom text-sm", class.get());

    view! {
        <div class="relative w-full overflow-auto">
            <table class=merged data-name="Table">
                {children()}
            </table>
        </div>
    }
}

/// Table header section.
#[component]
pub fn TableHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("[&_tr]:border-b", class.get());

    view! {
        <thead class=merged data-name="TableHeader">
            {children()}
        </thead>
    }
}

/// Table body section.
#[component]
pub fn TableBody(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("[&_tr:last-child]:border-0", class.get());

    view! {
        <tbody class=merged data-name="TableBody">
            {children()}
        </tbody>
    }
}

/// Table footer section.
#[component]
pub fn TableFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0",
            class.get()
        )
    };

    view! {
        <tfoot class=merged data-name="TableFooter">
            {children()}
        </tfoot>
    }
}

/// Table row.
#[component]
pub fn TableRow(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "border-b transition-colors hover:bg-muted/50 \
             data-[state=selected]:bg-muted",
            class.get()
        )
    };

    view! {
        <tr class=merged data-name="TableRow">
            {children()}
        </tr>
    }
}

/// Table head cell.
#[component]
pub fn TableHead(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "h-12 px-4 text-left align-middle font-medium \
             text-muted-foreground [&:has([role=checkbox])]:pr-0",
            class.get()
        )
    };

    view! {
        <th class=merged data-name="TableHead">
            {children()}
        </th>
    }
}

/// Table cell.
#[component]
pub fn TableCell(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || {
        cn!(
            "p-4 align-middle [&:has([role=checkbox])]:pr-0",
            class.get()
        )
    };

    view! {
        <td class=merged data-name="TableCell">
            {children()}
        </td>
    }
}

/// Table caption.
#[component]
pub fn TableCaption(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let merged = move || cn!("mt-4 text-sm text-muted-foreground", class.get());

    view! {
        <caption class=merged data-name="TableCaption">
            {children()}
        </caption>
    }
}
