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
