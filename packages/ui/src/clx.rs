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

pub use crate::utils::Utils;
pub use leptos::prelude::*;
pub use paste;
pub use tw_merge::*;

/// Creates a component with Tailwind class merging.
///
/// # Example
/// ```rust,ignore
/// use montrs_ui::clx;
///
/// clx! {Card, div, "rounded-lg p-4", "bg-sky-500"}
///
/// view! { <Card>"Default: bg-sky-500"</Card> }
/// view! { <Card class="bg-orange-500">"Override"</Card> }
/// ```
#[macro_export]
macro_rules! clx {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[::leptos::component]
        pub fn $name(
            #[prop(into, optional)] class: ::leptos::prelude::MaybeSignal<String>,
            children: ::leptos::prelude::Children,
        ) -> impl ::leptos::prelude::IntoView {
            let merged_classes = ::leptos::prelude::Memo::new(move |_| {
                $crate::tw_merge::tw_merge!($crate::tw_merge::tw_join!($($base_class),+), class.get())
            });

            ::leptos::prelude::view! {
                <$element
                    class=merged_classes
                    data-name=stringify!($name)
                >
                    {children()}
                </$element>
            }
        }
    };
}

/// Creates a self-closing component with Tailwind class merging.
///
/// # Example
/// ```rust,ignore
/// use montrs_ui::void;
///
/// void! {MyImage, img, "rounded-lg border"}
/// void! {MyInput, input, "px-3 py-2 border rounded"}
/// ```
#[macro_export]
macro_rules! void {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[::leptos::component]
        pub fn $name(
            #[prop(into, optional)] class: ::leptos::prelude::MaybeSignal<String>,
        ) -> impl ::leptos::prelude::IntoView {
            let merged_classes = ::leptos::prelude::Memo::new(move |_| {
                $crate::tw_merge::tw_merge!($crate::tw_merge::tw_join!($($base_class),+), class.get())
            });

            ::leptos::prelude::view! {
                <$element
                    class=merged_classes
                    data-name=stringify!($name)
                />
            }
        }
    };
}

/// Creates a component with a random CSS transition name for view transitions.
#[macro_export]
macro_rules! transition {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[::leptos::component]
        pub fn $name(
            #[prop(into, optional)] class: ::leptos::prelude::MaybeSignal<String>,
            children: ::leptos::prelude::Children,
        ) -> impl ::leptos::prelude::IntoView {
            let merged_classes = ::leptos::prelude::Memo::new(move |_| {
                $crate::tw_merge::tw_merge!($crate::tw_merge::tw_join!($($base_class),+), class.get())
            });

            let transition_name = $crate::utils::Utils::use_random_transition_name();

            ::leptos::prelude::view! {
                <$element
                    class=merged_classes
                    style=transition_name
                    data-name=stringify!($name)
                >
                    {children()}
                </$element>
            }
        }
    };
}
