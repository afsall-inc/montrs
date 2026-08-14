//! Service identifier — namespace/name with safe-path encoding.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A unique identifier for a service, with optional namespace.
///
/// Format: `name` or `namespace/name`.
/// The name is encoded into a safe filesystem path.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceId {
    /// Optional namespace (e.g., "project", "system").
    pub namespace: Option<String>,
    /// Service name.
    pub name: String,
}

impl ServiceId {
    /// Create a new ServiceId from a namespace and name.
    pub fn new(namespace: Option<impl Into<String>>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.map(|n| n.into()),
            name: name.into(),
        }
    }

    /// Create a ServiceId from just a name (no namespace).
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            namespace: None,
            name: name.into(),
        }
    }

    /// Return the safe-path-encoded form of this ID.
    /// Used for log files, state files, and socket paths.
    pub fn encoded(&self) -> String {
        match &self.namespace {
            Some(ns) => format!("{}/{}", sanitize(ns), sanitize(&self.name)),
            None => sanitize(&self.name),
        }
    }

    /// Return the display name: `namespace/name` or `name`.
    pub fn display_name(&self) -> String {
        match &self.namespace {
            Some(ns) => format!("{ns}/{}", self.name),
            None => self.name.clone(),
        }
    }

    /// Return this ID as a filesystem-safe path component.
    pub fn to_path_component(&self) -> String {
        self.encoded().replace('/', "_")
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for ServiceId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("service ID cannot be empty".to_string());
        }
        if let Some((ns, name)) = s.split_once('/') {
            if ns.is_empty() || name.is_empty() {
                return Err("namespace and name must not be empty".to_string());
            }
            Ok(Self {
                namespace: Some(ns.to_string()),
                name: name.to_string(),
            })
        } else {
            Ok(Self {
                namespace: None,
                name: s.to_string(),
            })
        }
    }
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let id: ServiceId = "redis".parse().unwrap();
        assert_eq!(id.name, "redis");
        assert!(id.namespace.is_none());

        let id: ServiceId = "project/api".parse().unwrap();
        assert_eq!(id.namespace, Some("project".into()));
        assert_eq!(id.name, "api");
    }

    #[test]
    fn test_encoded() {
        let id = ServiceId::new(Some("my-project"), "web-api");
        assert_eq!(id.encoded(), "my-project/web-api");
    }

    #[test]
    fn test_display() {
        let id = ServiceId::from_name("redis");
        assert_eq!(id.to_string(), "redis");

        let id = ServiceId::new(Some("dev"), "api");
        assert_eq!(id.to_string(), "dev/api");
    }

    #[test]
    fn test_path_component() {
        let id = ServiceId::new(Some("ns"), "svc");
        assert_eq!(id.to_path_component(), "ns_svc");
    }
}