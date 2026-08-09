//! Invariant tests for montrs-shell.

use montrs_shell::*;

#[test]
fn test_shell_type_bash() {
    let st: ShellType = "bash".parse().unwrap();
    assert_eq!(st, ShellType::Bash);
    assert_eq!(st.to_string(), "bash");
}

#[test]
fn test_shell_type_zsh() {
    let st: ShellType = "zsh".parse().unwrap();
    assert_eq!(st, ShellType::Zsh);
}

#[test]
fn test_shell_type_fish() {
    let st: ShellType = "fish".parse().unwrap();
    assert_eq!(st, ShellType::Fish);
}

#[test]
fn test_shell_type_pwsh() {
    assert_eq!("pwsh".parse::<ShellType>().unwrap(), ShellType::Pwsh);
    assert_eq!("powershell".parse::<ShellType>().unwrap(), ShellType::Pwsh);
}

#[test]
fn test_shell_type_unknown() {
    assert!("unknown".parse::<ShellType>().is_err());
}

#[test]
fn test_bash_activate_contains_path() {
    let shell = ShellType::Bash.as_shell();
    let opts = ActivateOptions {
        exe: "montrs".into(),
        flags: String::new(),
        no_hook: false,
    };
    let script = shell.activate(&opts);
    assert!(script.contains("PATH"));
    assert!(script.contains("MONTRS_SHELL"));
    assert!(script.contains("_montrs_hook"));
}

#[test]
fn test_zsh_activate_contains_hook() {
    let shell = ShellType::Zsh.as_shell();
    let opts = ActivateOptions {
        exe: "montrs".into(),
        flags: String::new(),
        no_hook: false,
    };
    let script = shell.activate(&opts);
    assert!(script.contains("precmd"));
    assert!(script.contains("_montrs_hook"));
}

#[test]
fn test_fish_activate_contains_fish_add_path() {
    let shell = ShellType::Fish.as_shell();
    let opts = ActivateOptions {
        exe: "montrs".into(),
        flags: String::new(),
        no_hook: false,
    };
    let script = shell.activate(&opts);
    assert!(script.contains("fish_add_path"));
}

#[test]
fn test_pwsh_activate_contains_env() {
    let shell = ShellType::Pwsh.as_shell();
    let opts = ActivateOptions {
        exe: "montrs".into(),
        flags: String::new(),
        no_hook: false,
    };
    let script = shell.activate(&opts);
    assert!(script.contains("MONTRS_SHELL"));
}

#[test]
fn test_set_env() {
    let shell = ShellType::Bash.as_shell();
    let cmd = shell.set_env("FOO", "bar");
    assert!(cmd.contains("FOO"));
    assert!(cmd.contains("bar"));
}

#[test]
fn test_unset_env() {
    let shell = ShellType::Bash.as_shell();
    let cmd = shell.unset_env("FOO");
    assert!(cmd.contains("unset"));
}

#[test]
fn test_prepend_path() {
    let shell = ShellType::Bash.as_shell();
    let cmd = shell.prepend_path("/tmp/bin");
    assert!(cmd.contains("/tmp/bin"));
}

#[test]
fn test_deactivate() {
    let shell = ShellType::Bash.as_shell();
    let cmd = shell.deactivate();
    assert!(cmd.contains("unset MONTRS_SHELL"));
}

#[test]
fn test_shell_detect() {
    let shell = ShellType::detect();
    // Should always parse to a valid shell type
    let _ = shell.to_string();
}

#[test]
fn test_shim_create_file() {
    let dir = tempfile::tempdir().unwrap();
    let shims_dir = dir.path().join("shims");
    let bin_path = dir.path().join("bin");
    std::fs::create_dir_all(&bin_path).unwrap();
    std::fs::write(bin_path.join("test-tool"), b"#!/bin/sh\necho test")
        .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            bin_path.join("test-tool"),
            std::fs::Permissions::from_mode(0o755),
        )
        .unwrap();
    }
    shims::create_shim(
        &shims_dir,
        "test-tool",
        "test-tool",
        &bin_path.join("test-tool"),
    )
    .unwrap();
    assert!(shims_dir.join("test-tool").exists());
}

#[test]
fn test_shim_list_empty() {
    let dir = tempfile::tempdir().unwrap();
    let shims = shims::list_shims(dir.path()).unwrap();
    assert!(shims.is_empty());
}

#[test]
fn test_reshim_all_empty() {
    let dir = tempfile::tempdir().unwrap();
    let count =
        shims::reshim_all(dir.path(), dir.path().join("shims").as_path())
            .unwrap();
    assert_eq!(count, 0);
}
