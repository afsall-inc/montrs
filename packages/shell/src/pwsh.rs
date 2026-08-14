use crate::{ActivateOptions, Shell};
use std::fmt;

#[derive(Default)]
pub struct Pwsh;

impl fmt::Display for Pwsh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pwsh")
    }
}

impl Shell for Pwsh {
    fn activate(&self, opts: &ActivateOptions) -> String {
        let exe = opts.exe.to_string_lossy();
        let shims = std::env::var("MONTRS_SHIMS_DIR").unwrap_or_else(|_| {
            montrs_tool::backend::default_shims_dir()
                .display()
                .to_string()
        });
        format!(
            r#"$env:MONTRS_SHELL = "pwsh"
Invoke-Expression "$([string]::Join(' ', '{exe}', 'hook-env', 'pwsh'))" *>$null
$env:PATH = "{shims};$env:PATH"
"#,
        )
    }

    fn deactivate(&self) -> String {
        String::from(
            "Remove-Item Env:MONTRS_SHELL -ErrorAction SilentlyContinue\n",
        )
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        format!("$env:{k} = \"{v}\"\n")
    }

    fn unset_env(&self, k: &str) -> String {
        format!("Remove-Item Env:{k} -ErrorAction SilentlyContinue\n")
    }

    fn prepend_path(&self, dir: &str) -> String {
        format!("$env:PATH = \"{dir};$env:PATH\"\n")
    }
}
