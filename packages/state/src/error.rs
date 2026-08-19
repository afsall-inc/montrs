use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum StateError {
    #[error("state lock is poisoned")]
    LockPoisoned,
    #[error("invalid state transition from {from} on event {event}")]
    InvalidTransition { from: String, event: String },
    #[error("state not found: {0}")]
    StateNotFound(String),
    #[error("event not handled: {0}")]
    EventNotHandled(String),
    #[error("guard rejected transition")]
    GuardRejected,
    #[error("action error: {0}")]
    ActionError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("{0}")]
    General(String),
}

pub type StateResult<T> = Result<T, StateError>;