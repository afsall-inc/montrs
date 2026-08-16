// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! CLI commands for service management (`montrs services`).

use montrs_services::{
    config::ServiceConfig, service::ServiceStatus, service_id::ServiceId,
    supervisor::Supervisor,
};
use std::collections::HashMap;

/// Load the service configs from the current project's montrs.toml.
fn load_service_configs() -> anyhow::Result<HashMap<String, ServiceConfig>> {
    use crate::config::MontrsConfig;
    let config = MontrsConfig::load()?;
    let raw = config.meta.services.clone();
    if raw.is_empty() {
        anyhow::bail!(
            "No services defined in [services] section of montrs.toml"
        );
    }
    ServiceConfig::from_toml_map(&raw)
}

/// Create a supervisor from the current project config.
fn create_supervisor() -> anyhow::Result<Supervisor> {
    let configs = load_service_configs()?;
    let data_dir = montrs_services::supervisor::default_data_dir();
    Supervisor::new(configs, data_dir)
}

/// Display a service status with color.
fn format_status(status: &ServiceStatus) -> String {
    match status {
        ServiceStatus::Running => console::style("running").green().to_string(),
        ServiceStatus::Starting => {
            console::style("starting").yellow().to_string()
        }
        ServiceStatus::Stopping => {
            console::style("stopping").yellow().to_string()
        }
        ServiceStatus::Stopped => console::style("stopped").dim().to_string(),
        ServiceStatus::Failed => console::style("failed").red().to_string(),
        ServiceStatus::Waiting => console::style("waiting").cyan().to_string(),
    }
}

/// `montrs services list`
pub async fn list() -> anyhow::Result<()> {
    let supervisor = create_supervisor()?;
    let services = supervisor.list().await;
    if services.is_empty() {
        println!("No services defined");
        return Ok(());
    }
    println!("{}", console::style("Services:").bold());
    for (id, status, pid) in &services {
        let pid_str = pid
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string());
        println!(
            "  {:<20} {}  pid={}",
            console::style(id).cyan(),
            format_status(status),
            pid_str,
        );
    }
    Ok(())
}

/// `montrs services start <name>`
pub async fn start(name: &str) -> anyhow::Result<()> {
    let supervisor = create_supervisor()?;
    let id = ServiceId::from_name(name);
    println!("Starting {id}...");
    supervisor.start(&id).await?;
    println!("{} Service {id} started", console::style("✓").green());
    Ok(())
}

/// `montrs services stop <name>`
pub async fn stop(name: &str) -> anyhow::Result<()> {
    let supervisor = create_supervisor()?;
    let id = ServiceId::from_name(name);
    println!("Stopping {id}...");
    supervisor.stop(&id).await?;
    println!("{} Service {id} stopped", console::style("✓").green());
    Ok(())
}

/// `montrs services restart <name>`
pub async fn restart(name: &str) -> anyhow::Result<()> {
    let supervisor = create_supervisor()?;
    let id = ServiceId::from_name(name);
    println!("Restarting {id}...");
    supervisor.restart(&id).await?;
    println!("{} Service {id} restarted", console::style("✓").green());
    Ok(())
}

/// `montrs services status [name]`
pub async fn status(name: Option<&str>) -> anyhow::Result<()> {
    let supervisor = create_supervisor()?;
    if let Some(name) = name {
        let id = ServiceId::from_name(name);
        match supervisor.status(&id).await {
            Some(status) => {
                println!("{}: {}", id, format_status(&status));
            }
            None => {
                anyhow::bail!("Service '{name}' not found");
            }
        }
    } else {
        list().await?;
    }
    Ok(())
}

/// `montrs services start --all`
pub async fn start_all() -> anyhow::Result<()> {
    let supervisor = create_supervisor()?;
    println!("Starting all services...");
    supervisor.start_all().await?;
    println!("{} All services started", console::style("✓").green());
    Ok(())
}

/// `montrs services stop --all`
pub async fn stop_all() -> anyhow::Result<()> {
    let supervisor = create_supervisor()?;
    println!("Stopping all services...");
    supervisor.stop_all().await?;
    println!("{} All services stopped", console::style("✓").green());
    Ok(())
}

/// `montrs services logs [name]`
pub async fn logs(name: Option<&str>) -> anyhow::Result<()> {
    let store = montrs_log::LogStore::open_default()?;
    let query = montrs_log::LogQuery {
        service: name.map(|s| s.to_string()),
        limit: 50,
        ..Default::default()
    };
    let entries = store.query(query).await?;
    if entries.is_empty() {
        println!("No log entries found");
        return Ok(());
    }
    for entry in &entries {
        let level = match entry.level.to_lowercase().as_str() {
            "error" => console::style(&entry.level).red().to_string(),
            "warn" => console::style(&entry.level).yellow().to_string(),
            "info" => console::style(&entry.level).green().to_string(),
            _ => entry.level.clone(),
        };
        println!(
            "[{}] [{}] {}: {}",
            entry.ts, level, entry.service, entry.message
        );
    }
    Ok(())
}
