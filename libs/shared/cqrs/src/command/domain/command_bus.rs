//! Command bus trait definition.

use async_trait::async_trait;

use super::command::Command;
use super::command_bus_error::CommandBusError;

/// Asynchronous dispatcher for commands.
///
/// Implementations are responsible for routing a boxed [`Command`] to its
/// registered [`CommandHandler`](super::command_handler::CommandHandler).
///
/// # Errors
///
/// Returns [`CommandBusError::HandlerNotFound`] if no handler has been
/// registered for the given command type.
///
/// # Example
///
/// ```rust
/// # use shared_cqrs::command::domain::command::Command;
/// # use shared_cqrs::command::domain::command_bus::CommandBus;
/// # use shared_cqrs::command::domain::command_bus_error::CommandBusError;
/// # use shared_cqrs::command::domain::command_handler::CommandHandler;
/// # use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
/// # use async_trait::async_trait;
/// # struct MyCommand;
/// # impl Command for MyCommand {}
/// # struct MyHandler;
/// # #[async_trait]
/// # impl CommandHandler<MyCommand> for MyHandler {
/// #     async fn handle(&self, _: MyCommand) -> Result<(), CommandBusError> { Ok(()) }
/// # }
/// # #[tokio::main]
/// # async fn main() {
/// #     let mut bus = InMemoryCommandBus::new();
/// #     bus.register(MyHandler).unwrap();
/// #     let my_command = MyCommand;
/// let result = bus.dispatch(Box::new(my_command)).await;
/// assert!(result.is_ok());
/// # }
/// ```
#[async_trait]
pub trait CommandBus: Send + Sync {
    /// Dispatches a command to its registered handler.
    ///
    /// # Arguments
    ///
    /// * `command` - A boxed, type-erased command to dispatch.
    ///
    /// # Errors
    ///
    /// Returns [`CommandBusError`] if no handler is found or the handler fails.
    async fn dispatch(&self, command: Box<dyn Command>) -> Result<(), CommandBusError>;
}
