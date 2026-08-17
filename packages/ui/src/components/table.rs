// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
