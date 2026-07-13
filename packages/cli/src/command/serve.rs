use crate::{config::MontrsConfig, utils::run_cargo_leptos};

pub async fn run() -> anyhow::Result<()> {
    let mut config = MontrsConfig::load()?;

    // Handle tailwind.toml
    if let Ok(Some(js_path)) = crate::config::tailwind::ensure_tailwind_config(
        std::path::Path::new("."),
        config.project.tailwind_style.unwrap_or_default(),
    ) && config.build.tailwind_config_file.is_none()
    {
        config.build.tailwind_config_file =
            Some(js_path.to_string_lossy().into_owned());
    }

    // "serve" in montrs usually implies watching/running the server.
    // We map it to "watch" as cargo-leptos doesn't have a standalone "serve" command exposed clearly via CLI
    // other than running the binary, but "watch" is safer for dev.
    run_cargo_leptos("watch", &[], &config).await
}
