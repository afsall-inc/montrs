use crate::{ActivateOptions, Shell};
use std::fmt;

#[derive(Default)]
pub struct Zsh;

impl fmt::Display for Zsh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "zsh")
    }
}

impl Shell for Zsh {
    fn activate(&self, opts: &ActivateOptions) -> String {
        let exe = opts.exe.to_string_lossy();
        let mut out = String::new();
        out.push_str(&format!("export MONTRS_SHELL=\"zsh\"\n"));
        out.push_str(&format!(
            r#"_montrs_hook() {{
    local ret=$?
    eval "$("{exe}" hook-env zsh)" 2>/dev/null
    return $ret
}}
autoload -Uz add-zsh-hook
add-zsh-hook precmd _montrs_hook
"#,
        ));
        let shims = std::env::var("MONTRS_SHIMS_DIR").unwrap_or_else(|_| {
            montrs_tool::backend::default_shims_dir()
                .display()
                .to_string()
        });
        out.push_str(&format!("export PATH=\"{shims}\":$PATH\n"));
        out
    }

    fn deactivate(&self) -> String {
        String::from(
            "unset MONTRS_SHELL\nunfunction _montrs_hook\nadd-zsh-hook -d \
             precmd _montrs_hook\n",
        )
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        format!("export {k}=\"{v}\"\n")
    }

    fn unset_env(&self, k: &str) -> String {
        format!("unset {k}\n")
    }

    fn prepend_path(&self, dir: &str) -> String {
        format!("export PATH=\"{dir}\":$PATH\n")
    }
}
