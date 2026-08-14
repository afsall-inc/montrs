use montrs_shell::{ActivateOptions, ShellType};

pub async fn activate(shell_name: &str) -> anyhow::Result<()> {
    let shell_type: ShellType = if shell_name == "auto" {
        ShellType::detect()
    } else {
        shell_name.parse().map_err(|e: String| anyhow::anyhow!(e))?
    };
    let shell = shell_type.as_shell();
    let exe = std::env::current_exe().unwrap_or_else(|_| "montrs".into());
    let opts = ActivateOptions {
        exe,
        flags: String::new(),
        no_hook: false,
    };
    print!("{}", shell.activate(&opts));
    Ok(())
}

pub async fn deactivate(shell_name: &str) -> anyhow::Result<()> {
    let shell_type: ShellType = if shell_name == "auto" {
        ShellType::detect()
    } else {
        shell_name.parse().map_err(|e: String| anyhow::anyhow!(e))?
    };
    let shell = shell_type.as_shell();
    print!("{}", shell.deactivate());
    Ok(())
}

pub async fn reshim() -> anyhow::Result<()> {
    let count = montrs_shell::shims::reshim_all_default()?;
    let shims_dir = montrs_shell::shims::default_shims_dir();
    println!("Regenerated {count} shims in {}", shims_dir.display());
    Ok(())
}
