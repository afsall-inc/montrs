// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Shared UI components for the workspace.

use leptos::prelude::*;
use tailwind_fuse::*;

/// A reusable button component with type-safe variants.
#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeSignal<ButtonVariant>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    on_click: impl Fn(ev::MouseEvent) + 'static,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        let btn = ButtonClass {
            variant: variant.get(),
            size: size.get(),
        };
        btn.with_class(class.get())
    });

    view! {
        <button class=class on:click=on_click>
            {children()}
        </button>
    }
}

#[derive(TwClass, Clone, Copy)]
#[tw(class = "px-6 py-2 rounded-lg font-medium transition-colors focus:outline-none focus:ring-2")]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(TwVariant, Clone, Copy, Default)]
pub enum ButtonVariant {
    #[tw(default, class = "bg-blue-600 hover:bg-blue-500 text-white focus:ring-blue-400")]
    Primary,
    #[tw(class = "bg-gray-600 hover:bg-gray-500 text-white focus:ring-gray-400")]
    Secondary,
    #[tw(class = "bg-transparent border border-gray-500 hover:bg-gray-800 text-white focus:ring-gray-400")]
    Outline,
}

#[derive(TwVariant, Clone, Copy, Default)]
pub enum ButtonSize {
    #[tw(default, class = "text-base")]
    Medium,
    #[tw(class = "text-sm px-4 py-1")]
    Small,
    #[tw(class = "text-lg px-8 py-3")]
    Large,
}
