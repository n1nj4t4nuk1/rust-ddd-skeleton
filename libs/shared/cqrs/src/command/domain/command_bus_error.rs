//! Error types for the command bus.

use std::any::TypeId;
use std::fmt;

/// Errors that can occur during command dispatching.
#[derive(Debug)]
pub enum CommandBusError {
    /// No handler has been registered for the given command type.
    HandlerNotFound(TypeId),
    /// A handler for the given command type is already registered.
    HandlerAlreadyRegistered(TypeId),
    /// The handler returned an error during execution.
    HandlerError(String),
}

impl fmt::Display for CommandBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandBusError::HandlerNotFound(type_id) => {
                write!(f, "No handler registered for command {:?}", type_id)
            }
            CommandBusError::HandlerAlreadyRegistered(type_id) => {
                write!(f, "Handler already registered for command {:?}", type_id)
            }
            CommandBusError::HandlerError(msg) => {
                write!(f, "Handler error: {msg}")
            }
        }
    }
}

impl std::error::Error for CommandBusError {}
