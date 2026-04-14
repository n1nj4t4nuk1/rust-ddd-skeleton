//! Command handler trait definition.

use async_trait::async_trait;

use super::command::Command;
use super::command_bus_error::CommandBusError;

/// Asynchronous handler for a specific command type `C`.
///
/// Each command type should have exactly one handler registered in the
/// [`CommandBus`](super::command_bus::CommandBus).
///
/// # Type parameters
///
/// * `C` - The concrete [`Command`] type this handler processes.
///
/// # Associated types
///
/// * `Response` - The type returned on success. Must be `Send + Sync + 'static`
///   so it can be boxed and returned through the bus.
///
/// # Example
///
/// ```rust
/// # use shared_cqrs::command::domain::command::Command;
/// # use shared_cqrs::command::domain::command_bus_error::CommandBusError;
/// # use async_trait::async_trait;
/// # struct CreateUserCommand;
/// # impl Command for CreateUserCommand {}
/// # struct CreateUserResponse;
/// use shared_cqrs::command::domain::command_handler::CommandHandler;
///
/// struct CreateUserHandler;
///
/// #[async_trait]
/// impl CommandHandler<CreateUserCommand> for CreateUserHandler {
///     type Response = CreateUserResponse;
///
///     async fn handle(&self, _command: CreateUserCommand) -> Result<CreateUserResponse, CommandBusError> {
///         Ok(CreateUserResponse)
///     }
/// }
/// ```
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// The type returned by the handler on success.
    type Response: Send + Sync + 'static;

    /// Handles the given command.
    ///
    /// # Arguments
    ///
    /// * `command` - The concrete command instance to process.
    ///
    /// # Errors
    ///
    /// Returns [`CommandBusError::HandlerError`] if the business logic fails.
    async fn handle(&self, command: C) -> Result<Self::Response, CommandBusError>;
}
