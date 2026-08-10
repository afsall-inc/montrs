//! Lifecycle hooks — run commands on service state transitions.

use crate::config::LifecycleHooks;
use tracing::error;

/// The set of lifecycle events that can trigger hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    Ready,
    Fail,
    Retry,
    Stop,
    Exit,
}

impl LifecycleEvent {
    fn command_key(&self) -> &'static str {
        match self {
            LifecycleEvent::Ready => "on_ready",
            LifecycleEvent::Fail => "on_fail",
            LifecycleEvent::Retry => "on_retry",
            LifecycleEvent::Stop => "on_stop",
            LifecycleEvent::Exit => "on_exit",
        }
    }
}

/// Runs lifecycle hooks as shell commands.
pub struct HookRunner;

impl HookRunner {
    /// Get the command for a given event, if configured.
    pub fn command_for(hooks: &LifecycleHooks, event: LifecycleEvent) -> Option<String> {
        match event {
            LifecycleEvent::Ready => hooks.on_ready.clone(),
            LifecycleEvent::Fail => hooks.on_fail.clone(),
            LifecycleEvent::Retry => hooks.on_retry.clone(),
            LifecycleEvent::Stop => hooks.on_stop.clone(),
            LifecycleEvent::Exit => hooks.on_exit.clone(),
        }
    }

    /// Run the hook for an event, if configured. Non-blocking.
    pub async fn run_if_present(
        service: &str,
        hooks: &LifecycleHooks,
        event: LifecycleEvent,
    ) {
        if let Some(cmd) = Self::command_for(hooks, event) {
            let result = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .spawn();
            match result {
                Ok(mut child) => {
                    if let Err(e) = child.wait().await {
                        error!(
                            "service {}: hook {} failed to run: {e}",
                            service,
                            event.command_key()
                        );
                    }
                }
                Err(e) => {
                    error!(
                        "service {}: failed to spawn hook {}: {e}",
                        service,
                        event.command_key()
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_for() {
        let hooks = LifecycleHooks {
            on_ready: Some("echo ready".into()),
            ..Default::default()
        };
        assert_eq!(
            HookRunner::command_for(&hooks, LifecycleEvent::Ready),
            Some("echo ready".to_string())
        );
        assert_eq!(HookRunner::command_for(&hooks, LifecycleEvent::Fail), None);
    }
}