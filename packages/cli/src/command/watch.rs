use crate::{config::MontrsConfig, utils::run_cargo_leptos};

pub async fn run() -> anyhow::Result<()> {
    let config = MontrsConfig::load()?;

    run_cargo_leptos("watch", &[], &config).await
}