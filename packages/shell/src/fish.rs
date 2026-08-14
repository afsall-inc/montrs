use crate::{ActivateOptions, Shell};
use std::fmt;

#[derive(Default)]
pub struct Fish;

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fish")
    }
}

impl Shell for Fish {
    fn activate(&self, opts: &ActivateOptions) -> String {
        let exe = opts.exe.to_string_lossy();
        let shims = std::env::var("MONTRS_SHIMS_DIR").unwrap_or_else(|_| {
            montrs_tool::backend::default_shims_dir()
                .display()
                .to_string()
        });
        format!(
            r#"set -gx MONTRS_SHELL fish
function _montrs_hook --on-event fish_prompt
    set -l ret $status
    eval "$([string join ' ' {exe} hook-env fish])" 2>/dev/null
    return $ret
end
fish_add_path {shims}
"#,
        )
    }

    fn deactivate(&self) -> String {
        String::from("set -e MONTRS_SHELL\nfunctions -e _montrs_hook\n")
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        format!("set -gx {k} \"{v}\"\n")
    }

    fn unset_env(&self, k: &str) -> String {
        format!("set -e {k}\n")
    }

    fn prepend_path(&self, dir: &str) -> String {
        format!("fish_add_path \"{dir}\"\n")
    }
}
