//! Configuration value types for `montrs.toml [services]` section.
//!
//! Thin wrappers (newtypes) around primitives with custom serialization,
//! validation, or display logic.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ---------------------------------------------------------------------------
// StringOrStruct: serde "string or struct" pattern (bidirectional)
// ---------------------------------------------------------------------------

/// Trait for config types accepting either a string shorthand or a full object.
pub trait StringOrStruct: Sized {
    type Short: for<'de> Deserialize<'de> + Serialize;
    type Raw: for<'de> Deserialize<'de> + Serialize;

    fn from_short(short: Self::Short) -> Self;
    fn from_raw(raw: Self::Raw) -> Result<Self, String>;
    fn is_shorthand(&self) -> bool;
    fn to_short(&self) -> Self::Short;
    fn to_raw(&self) -> Self::Raw;

    fn string_or_struct_serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.is_shorthand() {
            self.to_short().serialize(s)
        } else {
            self.to_raw().serialize(s)
        }
    }

    fn string_or_struct_deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        struct Visitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: StringOrStruct> serde::de::Visitor<'de> for Visitor<T> {
            type Value = T;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or an object")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<T, E> {
                let short = T::Short::deserialize(serde::de::value::StrDeserializer::<E>::new(v))
                    .map_err(E::custom)?;
                Ok(T::from_short(short))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<T, A::Error> {
                let raw = T::Raw::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                T::from_raw(raw).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_any(Visitor::<Self>(std::marker::PhantomData))
    }
}

// ---------------------------------------------------------------------------
// BoolOrU32 serde helpers
// ---------------------------------------------------------------------------

/// Trait for types that serialize as `u32` (or `bool` for the sentinel value).
pub trait BoolOrU32: Sized + Copy + From<u32> + Into<u32> {
    const TRUE_VALUE: u32;

    fn bool_or_u32_serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let raw: u32 = (*self).into();
        if raw == Self::TRUE_VALUE { s.serialize_bool(true) } else { s.serialize_u32(raw) }
    }

    fn bool_or_u32_deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor<T>(std::marker::PhantomData<T>);
        impl<T: BoolOrU32> serde::de::Visitor<'_> for Visitor<T> {
            type Value = T;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a boolean or non-negative integer")
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<T, E> {
                Ok(T::from(if v { T::TRUE_VALUE } else { 0 }))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<T, E> {
                Ok(T::from(u32::try_from(v).unwrap_or(T::TRUE_VALUE)))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<T, E> {
                if v < 0 { Err(E::custom("value cannot be negative")) } else { self.visit_u64(v as u64) }
            }
        }
        d.deserialize_any(Visitor::<Self>(std::marker::PhantomData))
    }
}

// ---------------------------------------------------------------------------
// Ready check types
// ---------------------------------------------------------------------------

/// HTTP readiness check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyHttp {
    pub url: String,
    #[serde(default)]
    pub status: u16,
    #[serde(default)]
    pub timeout_ms: u64,
}

impl Default for ReadyHttp {
    fn default() -> Self { Self { url: String::new(), status: 200, timeout_ms: 5000 } }
}

/// Port readiness check.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReadyPort {
    pub port: u16,
    #[serde(default)]
    pub retries: u32,
}

impl Default for ReadyPort {
    fn default() -> Self { Self { port: 0, retries: 10 } }
}

/// Output (regex) readiness check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyOutput {
    pub pattern: String,
    #[serde(default)]
    pub timeout_ms: u64,
}

impl Default for ReadyOutput {
    fn default() -> Self { Self { pattern: String::new(), timeout_ms: 5000 } }
}

/// Command readiness check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyCmd {
    pub command: String,
    #[serde(default)]
    pub timeout_ms: u64,
}

impl Default for ReadyCmd {
    fn default() -> Self { Self { command: String::new(), timeout_ms: 5000 } }
}

// ---------------------------------------------------------------------------
// Retry policy
// ---------------------------------------------------------------------------

/// Retry policy for a service.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Retry {
    #[serde(default = "default_retry_count")]
    pub count: u32,
    #[serde(default = "default_retry_delay_ms")]
    pub delay_ms: u64,
    #[serde(default)]
    pub backoff: bool,
}

fn default_retry_count() -> u32 { 3 }
fn default_retry_delay_ms() -> u64 { 1000 }

impl Default for Retry {
    fn default() -> Self { Self { count: default_retry_count(), delay_ms: default_retry_delay_ms(), backoff: false } }
}

// ---------------------------------------------------------------------------
// Stop config
// ---------------------------------------------------------------------------

/// Signal used to stop a service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StopSignal {
    Sigterm,
    Sigint,
    Sigkill,
    Sighup,
}

impl Default for StopSignal {
    fn default() -> Self { Self::Sigterm }
}

/// Stop configuration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StopConfig {
    #[serde(default)]
    pub signal: StopSignal,
    #[serde(default = "default_stop_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_stop_timeout_ms() -> u64 { 5000 }

impl Default for StopConfig {
    fn default() -> Self { Self { signal: StopSignal::default(), timeout_ms: default_stop_timeout_ms() } }
}

// ---------------------------------------------------------------------------
// Resource limits
// ---------------------------------------------------------------------------

/// CPU limit (percent or quota).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CpuLimit {
    #[serde(default)]
    pub percent: Option<u32>,
    #[serde(default)]
    pub quota_ns: Option<u64>,
}

impl Default for CpuLimit { fn default() -> Self { Self { percent: None, quota_ns: None } } }

/// Memory limit in bytes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MemoryLimit {
    /// Bytes limit.
    pub limit: u64,
    /// Soft limit (grace) in bytes.
    #[serde(default)]
    pub soft_limit: Option<u64>,
}

impl Default for MemoryLimit { fn default() -> Self { Self { limit: 0, soft_limit: None } } }

// ---------------------------------------------------------------------------
// Cron / Watch
// ---------------------------------------------------------------------------

/// Cron retrigger mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CronRetrigger {
    Auto,
    OnFailure,
    OnChange,
    Never,
}

impl Default for CronRetrigger { fn default() -> Self { Self::Auto } }

/// Cron schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronConfig {
    pub schedule: String,
    #[serde(default)]
    pub retrigger: CronRetrigger,
}

impl Default for CronConfig {
    fn default() -> Self { Self { schedule: String::new(), retrigger: CronRetrigger::default() } }
}

/// Watch mode for auto-restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WatchMode {
    Auto,
    OnChange,
    OnRestart,
    Disabled,
}

impl Default for WatchMode { fn default() -> Self { Self::Auto } }

/// Directory to watch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dir {
    pub path: String,
    #[serde(default)]
    pub mode: WatchMode,
}

impl Default for Dir {
    fn default() -> Self { Self { path: String::new(), mode: WatchMode::default() } }
}

// ---------------------------------------------------------------------------
// Hooks
// ---------------------------------------------------------------------------

/// OnOutput hook — run a command when output matches a pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnOutputHook {
    pub pattern: String,
    pub command: String,
}

/// Lifecycle hooks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Hooks {
    #[serde(default)]
    pub on_ready: Option<String>,
    #[serde(default)]
    pub on_fail: Option<String>,
    #[serde(default)]
    pub on_retry: Option<String>,
    #[serde(default)]
    pub on_stop: Option<String>,
    #[serde(default)]
    pub on_exit: Option<String>,
}

// ---------------------------------------------------------------------------
// Port bump
// ---------------------------------------------------------------------------

/// Port bump strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PortBump {
    Auto,
    None,
    Random,
}

impl Default for PortBump { fn default() -> Self { Self::Auto } }

/// Port configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    pub port: u16,
    #[serde(default)]
    pub bump: PortBump,
}

impl Default for PortConfig {
    fn default() -> Self { Self { port: 0, bump: PortBump::default() } }
}