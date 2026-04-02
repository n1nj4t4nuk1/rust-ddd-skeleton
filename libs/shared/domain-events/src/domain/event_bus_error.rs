//! Error types for the event bus.

use std::fmt;

/// Errors that can occur when publishing domain events.
#[derive(Debug)]
pub enum EventBusError {
    /// An error occurred while dispatching an event to a subscriber.
    DispatchError(String),
}

impl fmt::Display for EventBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventBusError::DispatchError(msg) => write!(f, "Event dispatch error: {}", msg),
        }
    }
}

impl std::error::Error for EventBusError {}
