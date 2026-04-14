//! # CQRS (Command Query Responsibility Segregation)
//!
//! This crate implements the CQRS pattern, separating write operations
//! (commands) from read operations (queries).
//!
//! ## Main modules
//!
//! - [`command`]: Traits and infrastructure for command dispatching.
//! - [`query`]: Traits and infrastructure for query dispatching.
//!
//! ## Usage example
//!
//! ```rust
//! # use shared_cqrs::command::domain::command::Command;
//! # use shared_cqrs::command::domain::command_bus_error::CommandBusError;
//! # use shared_cqrs::command::domain::command_handler::CommandHandler;
//! # use async_trait::async_trait;
//! # struct MyCommand;
//! # impl Command for MyCommand {}
//! # struct MyResponse;
//! # struct MyHandler;
//! # #[async_trait]
//! # impl CommandHandler<MyCommand> for MyHandler {
//! #     type Response = MyResponse;
//! #     async fn handle(&self, _: MyCommand) -> Result<MyResponse, CommandBusError> { Ok(MyResponse) }
//! # }
//! use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
//! use shared_cqrs::command::domain::command_bus::CommandBus;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let mut bus = InMemoryCommandBus::new();
//! bus.register(MyHandler).unwrap();
//! let raw = bus.dispatch(Box::new(MyCommand)).await.unwrap();
//! let response = raw.downcast::<MyResponse>().unwrap();
//! # }
//! ```

pub mod command;
pub mod query;
