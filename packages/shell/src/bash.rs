use crate::{ActivateOptions, Shell};
use std::fmt;

#[derive(Default)]
pub struct Bash;

impl fmt::Display for Bash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bash")
    }
}

impl Shell for Bash {
    fn activate(&self, opts: &ActivateOptions) -> String {
        let exe = opts.exe.to_string_lossy();
        let mut out = String::new();
        out.push_str(&format!(
            r#"export MONTRS_SHELL="bash"
_montrs_hook() {{
    local ret=$?
    eval "$("{exe}" hook-env bash)" 2>/dev/null
    return $ret
}}
"#,
        ));
        let shims = std::env::var("MONTRS_SHIMS_DIR").unwrap_or_else(|_| {
            montrs_tool::backend::default_shims_dir()
                .display()
                .to_string()
        });
        out.push_str(&format!("export PATH=\"{shims}\":$PATH\n"));
        out.push_str(
            "export PROMPT_COMMAND=\"_montrs_hook${PROMPT_COMMAND:+;\
             $PROMPT_COMMAND}\"\n",
        );
        out
    }

    fn deactivate(&self) -> String {
        String::from(
            "unset MONTRS_SHELL\nunset -f _montrs_hook\nunset PROMPT_COMMAND\n",
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
