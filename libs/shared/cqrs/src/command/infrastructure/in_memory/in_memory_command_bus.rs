//! In-memory implementation of the [`CommandBus`] trait.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;

use crate::command::domain::command::Command;
use crate::command::domain::command_bus::CommandBus;
use crate::command::domain::command_bus_error::CommandBusError;
use crate::command::domain::command_handler::CommandHandler;

/// Type alias for a type-erased, heap-allocated async handler function
/// that returns a boxed response.
type HandlerFn = Box<
    dyn Fn(
            Box<dyn Any + Send + Sync>,
        ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send + Sync>, CommandBusError>> + Send>>
        + Send
        + Sync,
>;

/// An in-memory [`CommandBus`] that stores handlers in a [`HashMap`] keyed by command [`TypeId`].
///
/// Handlers are registered at startup and looked up at dispatch time using
/// Rust's type system. Each command type may have at most one handler.
///
/// The response is returned as `Box<dyn Any + Send + Sync>` and must be
/// downcast by the caller to the expected concrete type.
///
/// # Example
///
/// ```rust
/// # use shared_cqrs::command::domain::command::Command;
/// # use shared_cqrs::command::domain::command_bus::CommandBus;
/// # use shared_cqrs::command::domain::command_bus_error::CommandBusError;
/// # use shared_cqrs::command::domain::command_handler::CommandHandler;
/// # use async_trait::async_trait;
/// # struct CreateUserCommand { username: String }
/// # impl Command for CreateUserCommand {}
/// # struct CreateUserResponse;
/// # struct CreateUserHandler;
/// # #[async_trait]
/// # impl CommandHandler<CreateUserCommand> for CreateUserHandler {
/// #     type Response = CreateUserResponse;
/// #     async fn handle(&self, _: CreateUserCommand) -> Result<CreateUserResponse, CommandBusError> { Ok(CreateUserResponse) }
/// # }
/// use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
///
/// # #[tokio::main]
/// # async fn main() {
/// let mut bus = InMemoryCommandBus::new();
/// bus.register(CreateUserHandler).unwrap();
///
/// let raw = bus.dispatch(Box::new(CreateUserCommand { username: "alice".into() }))
///     .await
///     .unwrap();
/// let response = raw.downcast::<CreateUserResponse>().unwrap();
/// # }
/// ```
pub struct InMemoryCommandBus {
    handlers: HashMap<TypeId, HandlerFn>,
}

impl InMemoryCommandBus {
    /// Creates a new, empty `InMemoryCommandBus`.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a handler for a specific command type `C`.
    ///
    /// # Type parameters
    ///
    /// * `C` - The concrete [`Command`] type the handler processes.
    /// * `H` - A [`CommandHandler<C>`] implementation.
    ///
    /// # Errors
    ///
    /// Returns [`CommandBusError::HandlerAlreadyRegistered`] if a handler for
    /// `C` has already been registered.
    pub fn register<C, H>(&mut self, handler: H) -> Result<(), CommandBusError>
    where
        C: Command + 'static,
        H: CommandHandler<C> + 'static,
    {
        let type_id = TypeId::of::<C>();

        if self.handlers.contains_key(&type_id) {
            return Err(CommandBusError::HandlerAlreadyRegistered(type_id));
        }

        let handler = Arc::new(handler);
        self.handlers.insert(
            type_id,
            Box::new(move |cmd: Box<dyn Any + Send + Sync>| {
                let handler = Arc::clone(&handler);
                let cmd = match cmd.downcast::<C>() {
                    Ok(c) => *c,
                    Err(_) => {
                        return Box::pin(async move {
                            Err(CommandBusError::HandlerNotFound(TypeId::of::<C>()))
                        })
                            as Pin<
                                Box<
                                    dyn Future<
                                            Output = Result<
                                                Box<dyn Any + Send + Sync>,
                                                CommandBusError,
                                            >,
                                        > + Send,
                                >,
                            >;
                    }
                };
                Box::pin(async move {
                    let response = handler.handle(cmd).await?;
                    Ok(Box::new(response) as Box<dyn Any + Send + Sync>)
                })
            }),
        );

        Ok(())
    }
}

impl Default for InMemoryCommandBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CommandBus for InMemoryCommandBus {
    /// Dispatches the command to its registered handler and returns the response.
    ///
    /// # Errors
    ///
    /// Returns [`CommandBusError::HandlerNotFound`] if no handler is registered
    /// for the command's type.
    async fn dispatch(
        &self,
        command: Box<dyn Command>,
    ) -> Result<Box<dyn Any + Send + Sync>, CommandBusError> {
        let type_id = command.command_type_id();

        match self.handlers.get(&type_id) {
            Some(handler) => handler(command.into_any()).await,
            None => Err(CommandBusError::HandlerNotFound(type_id)),
        }
    }
}
