//! Runtime state persistence for services.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Persistent state for a single service.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceState {
    /// Whether the service is enabled.
    pub enabled: bool,
    /// PID of the last known running process.
    pub pid: Option<u32>,
    /// Number of times started.
    pub start_count: u64,
    /// Number of times failed.
    pub fail_count: u64,
    /// Timestamp of last start.
    pub last_start: Option<String>,
    /// Timestamp of last stop.
    pub last_stop: Option<String>,
    /// Whether it was running when the supervisor last exited.
    pub was_running: bool,
    /// Extra metadata.
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl ServiceState {
    pub fn new() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }
}

/// File-backed state store for the supervisor.
#[derive(Debug)]
pub struct StateFile {
    path: PathBuf,
    services: HashMap<String, ServiceState>,
}

impl StateFile {
    /// Open (or create) the state file at `path`.
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let services = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            toml::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Ok(Self { path, services })
    }

    /// Load the state file from the default location.
    pub fn default() -> anyhow::Result<Self> {
        let root = state_root();
        std::fs::create_dir_all(&root)?;
        Self::open(root.join("montrs-services.toml"))
    }

    /// Get the state for a service.
    pub fn get(&self, id: &str) -> Option<&ServiceState> {
        self.services.get(id)
    }

    /// Get mutable state for a service, creating if absent.
    pub fn get_mut(&mut self, id: &str) -> &mut ServiceState {
        self.services.entry(id.to_string()).or_default()
    }

    /// Set the state for a service and persist.
    pub fn set(&mut self, id: &str, state: ServiceState) -> anyhow::Result<()> {
        self.services.insert(id.to_string(), state);
        self.save()
    }

    /// Remove a service's state.
    pub fn remove(&mut self, id: &str) -> anyhow::Result<()> {
        self.services.remove(id);
        self.save()
    }

    /// List all tracked service IDs.
    pub fn list(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    /// Persist to disk.
    pub fn save(&self) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(&self.services)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    /// The path of the state file.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// The MontRS state root directory.
pub fn state_root() -> PathBuf {
    std::env::var_os("MONTRS_STATE")
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".local/state/montrs/services")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_roundtrip() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("state.toml");

        let mut state = StateFile::open(&path)?;
        state.get_mut("api").pid = Some(1234);
        state.get_mut("api").was_running = true;
        state.save()?;

        let state2 = StateFile::open(&path)?;
        let api = state2.get("api").unwrap();
        assert_eq!(api.pid, Some(1234));
        assert!(api.was_running);
        Ok(())
    }
}