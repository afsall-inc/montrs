pub use tw_merge::*;

/// Merges Tailwind CSS classes, resolving conflicts using `tw_merge`.
///
/// Accepts a variable number of arguments: strings, `Option<String>`, or
/// conditional expressions. Similar to shadcn's `cn()` utility.
///
/// # Example
/// ```rust,ignore
/// use montrs_ui::cn;
///
/// cn!("px-4 py-2", "bg-red-500")
/// cn!("px-4", Some("bg-red-500"))
/// cn!("px-4", cond.then_some("text-sm"))
/// ```
#[macro_export]
macro_rules! cn {
    ($($class:expr),+ $(,)?) => {
        $crate::tw_merge::tw_merge!($($crate::tw_merge::tw_join!($class)),+)
    };
}

pub use cn;
