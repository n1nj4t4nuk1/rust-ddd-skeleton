//! Base traits for defining commands in the CQRS pattern.

use std::any::{Any, TypeId};

/// Helper trait that enables type erasure for commands, allowing them to be
/// handled as `dyn Any` in a thread-safe manner.
///
/// This trait is implemented automatically for any type that is `Any + Send + Sync`,
/// so manual implementation is not required.
pub trait AnyCommand: Any + Send + Sync {
    /// Consumes the command and returns it as a `Box<dyn Any + Send + Sync>`.
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync>;

    /// Returns the [`TypeId`] of the concrete command type.
    ///
    /// Used internally by the command bus to route to the correct handler.
    fn command_type_id(&self) -> TypeId;
}

impl<T: Any + Send + Sync> AnyCommand for T {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync> {
        self
    }

    fn command_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

/// Marker trait for types that represent a command.
///
/// Implementing this trait signals that a type can be dispatched through
/// a [`CommandBus`](super::command_bus::CommandBus).
///
/// # Example
///
/// ```rust
/// use shared_cqrs::command::domain::command::Command;
///
/// struct CreateUserCommand {
///     pub username: String,
/// }
///
/// impl Command for CreateUserCommand {}
/// ```
pub trait Command: AnyCommand {}
