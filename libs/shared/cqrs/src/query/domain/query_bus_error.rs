//! Error types for the query bus.

use std::any::TypeId;
use std::fmt;

/// Errors that can occur during query dispatching.
#[derive(Debug)]
pub enum QueryBusError {
    /// No handler has been registered for the given query type.
    HandlerNotFound(TypeId),
    /// A handler for the given query type is already registered.
    HandlerAlreadyRegistered(TypeId),
    /// The handler returned an error during execution.
    HandlerError(String),
}

impl fmt::Display for QueryBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryBusError::HandlerNotFound(type_id) => {
                write!(f, "No handler registered for query {:?}", type_id)
            }
            QueryBusError::HandlerAlreadyRegistered(type_id) => {
                write!(f, "Handler already registered for query {:?}", type_id)
            }
            QueryBusError::HandlerError(msg) => {
                write!(f, "Handler error: {msg}")
            }
        }
    }
}

impl std::error::Error for QueryBusError {}
