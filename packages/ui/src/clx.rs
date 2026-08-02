pub use leptos::prelude::*;
pub use paste;
pub use tw_merge::*;

pub use crate::utils::Utils;

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