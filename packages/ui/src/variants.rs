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

pub use leptos;
pub use paste;
pub use tw_merge;

/// Generates type-safe Tailwind CSS variant/size component classes.
///
/// Generates variant/size enums with `Default` and `with_class` method.
///
/// # Examples
///
/// ```rust,ignore
/// use montrs_ui::variants;
///
/// // Variant + size + component
/// variants! {
///     Badge {
///         base: "inline-flex items-center font-semibold rounded-md",
///         variants: {
///             variant: {
///                 Default: "bg-primary text-primary-foreground",
///                 Secondary: "bg-secondary text-secondary-foreground",
///                 Outline: "border border-border",
///             },
///             size: {
///                 Default: "px-2.5 py-0.5 text-xs",
///                 Sm: "px-1.5 py-0.5 text-[10px]",
///                 Lg: "px-3 py-1 text-sm",
///             }
///         },
///         component: {
///             element: span
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! variants {
    // Variant + size + component
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            },
            component: {
                element: $element:ident
            }
        }
    ) => {
        $crate::paste::paste! {
            pub enum [<$component Variant>] {
                $first_variant,
                $($variant_key,)*
            }

            impl Default for [<$component Variant>] {
                fn default() -> Self {
                    Self::$first_variant
                }
            }

            impl Clone for [<$component Variant>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Variant>] {}

            impl [<$component Variant>] {
                fn class(&self) -> &'static str {
                    match self {
                        Self::$first_variant => $first_variant_class,
                        $(Self::$variant_key => $variant_class,)*
                    }
                }
            }

            pub enum [<$component Size>] {
                $first_size,
                $($size_key,)*
            }

            impl Default for [<$component Size>] {
                fn default() -> Self {
                    Self::$first_size
                }
            }

            impl Clone for [<$component Size>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Size>] {}

            impl [<$component Size>] {
                fn class(&self) -> &'static str {
                    match self {
                        Self::$first_size => $first_size_class,
                        $(Self::$size_key => $size_class,)*
                    }
                }
            }

            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
                pub size: [<$component Size>],
            }

            impl [<$component Class>] {
                pub fn with_class(&self, extra: String) -> String {
                    let base = $crate::tw_merge::tw_merge!($base_class, self.variant.class(), self.size.class());
                    $crate::tw_merge::tw_merge!(base, extra)
                }
            }

            impl Clone for [<$component Class>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Class>] {}

            #[::leptos::component]
            pub fn $component(
                #[prop(into, optional)] variant: ::leptos::prelude::Signal<[<$component Variant>]>,
                #[prop(into, optional)] size: ::leptos::prelude::Signal<[<$component Size>]>,
                #[prop(into, optional)] class: ::leptos::prelude::Signal<String>,
                #[prop(into, optional)] data_name: Option<String>,
                children: ::leptos::prelude::Children,
            ) -> impl ::leptos::prelude::IntoView {
                use ::leptos::prelude::*;

                let computed_class = move || {
                    let variant = variant.try_get().unwrap_or_default();
                    let size = size.try_get().unwrap_or_default();
                    let component_class = [<$component Class>] { variant, size };
                    component_class.with_class(class.try_get().unwrap_or_default())
                };

                let data_name = data_name.unwrap_or_else(|| stringify!($component).to_string());

                view! {
                    <$element class=computed_class data-name=data_name>
                        {children()}
                    </$element>
                }
            }
        }
    };

    // Variant + size only (no component)
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            pub enum [<$component Variant>] {
                $first_variant,
                $($variant_key,)*
            }

            impl Default for [<$component Variant>] {
                fn default() -> Self {
                    Self::$first_variant
                }
            }

            impl Clone for [<$component Variant>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Variant>] {}

            impl [<$component Variant>] {
                fn class(&self) -> &'static str {
                    match self {
                        Self::$first_variant => $first_variant_class,
                        $(Self::$variant_key => $variant_class,)*
                    }
                }
            }

            pub enum [<$component Size>] {
                $first_size,
                $($size_key,)*
            }

            impl Default for [<$component Size>] {
                fn default() -> Self {
                    Self::$first_size
                }
            }

            impl Clone for [<$component Size>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Size>] {}

            impl [<$component Size>] {
                fn class(&self) -> &'static str {
                    match self {
                        Self::$first_size => $first_size_class,
                        $(Self::$size_key => $size_class,)*
                    }
                }
            }

            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
                pub size: [<$component Size>],
            }

            impl [<$component Class>] {
                pub fn with_class(&self, extra: String) -> String {
                    let base = $crate::tw_merge::tw_merge!($base_class, self.variant.class(), self.size.class());
                    $crate::tw_merge::tw_merge!(base, extra)
                }
            }

            impl Clone for [<$component Class>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Class>] {}
        }
    };

    // Variant only (no size)
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            pub enum [<$component Variant>] {
                $first_variant,
                $($variant_key,)*
            }

            impl Default for [<$component Variant>] {
                fn default() -> Self {
                    Self::$first_variant
                }
            }

            impl Clone for [<$component Variant>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Variant>] {}

            impl [<$component Variant>] {
                fn class(&self) -> &'static str {
                    match self {
                        Self::$first_variant => $first_variant_class,
                        $(Self::$variant_key => $variant_class,)*
                    }
                }
            }

            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
            }

            impl [<$component Class>] {
                pub fn with_class(&self, extra: String) -> String {
                    let base = $crate::tw_merge::tw_merge!($base_class, self.variant.class());
                    $crate::tw_merge::tw_merge!(base, extra)
                }
            }

            impl Clone for [<$component Class>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Class>] {}
        }
    };

    // Size only (no variant)
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            pub enum [<$component Size>] {
                $first_size,
                $($size_key,)*
            }

            impl Default for [<$component Size>] {
                fn default() -> Self {
                    Self::$first_size
                }
            }

            impl Clone for [<$component Size>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Size>] {}

            impl [<$component Size>] {
                fn class(&self) -> &'static str {
                    match self {
                        Self::$first_size => $first_size_class,
                        $(Self::$size_key => $size_class,)*
                    }
                }
            }

            pub struct [<$component Class>] {
                pub size: [<$component Size>],
            }

            impl [<$component Class>] {
                pub fn with_class(&self, extra: String) -> String {
                    let base = $crate::tw_merge::tw_merge!($base_class, self.size.class());
                    $crate::tw_merge::tw_merge!(base, extra)
                }
            }

            impl Clone for [<$component Class>] {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for [<$component Class>] {}
        }
    };
}
