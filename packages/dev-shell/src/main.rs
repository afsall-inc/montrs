// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `montrs-dev-shell` — dev-only HTTP shell that hosts a hot-swappable app dylib.

use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("MONTRS_LOG").unwrap_or_else(|_| "info".to_string()),
        )
        .init();

    let addr = std::env::var("MONTRS_SITE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let dylib = std::env::var("MONTRS_APP_DYLIB")
        .map(PathBuf::from)
        .map_err(|_| anyhow::anyhow!("MONTRS_APP_DYLIB must be set"))?;

    montrs_dev_shell::serve(&addr, &dylib).await
}
