//! Query bus trait definition.

use std::any::Any;

use async_trait::async_trait;

use super::query::Query;
use super::query_bus_error::QueryBusError;

/// Asynchronous dispatcher for queries.
///
/// Implementations route a boxed [`Query`] to its registered
/// [`QueryHandler`](super::query_handler::QueryHandler) and return the
/// response as a type-erased `Box<dyn Any + Send + Sync>`.
///
/// Callers are responsible for downcasting the response to the expected type.
///
/// # Errors
///
/// Returns [`QueryBusError::HandlerNotFound`] if no handler has been
/// registered for the given query type.
///
/// # Example
///
/// ```rust
/// # use shared_cqrs::query::domain::query::Query;
/// # use shared_cqrs::query::domain::query_bus::QueryBus;
/// # use shared_cqrs::query::domain::query_bus_error::QueryBusError;
/// # use shared_cqrs::query::domain::query_handler::QueryHandler;
/// # use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
/// # use async_trait::async_trait;
/// # struct FindUserByIdQuery { id: String }
/// # impl Query for FindUserByIdQuery {}
/// # struct UserResponse;
/// # struct FindUserByIdHandler;
/// # #[async_trait]
/// # impl QueryHandler<FindUserByIdQuery> for FindUserByIdHandler {
/// #     type Response = UserResponse;
/// #     async fn handle(&self, _: FindUserByIdQuery) -> Result<UserResponse, QueryBusError> { Ok(UserResponse) }
/// # }
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #     let mut bus = InMemoryQueryBus::new();
/// #     bus.register(FindUserByIdHandler).unwrap();
/// let raw = bus.ask(Box::new(FindUserByIdQuery { id: "123".into() })).await?;
/// let user = raw.downcast::<UserResponse>().unwrap();
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait QueryBus: Send + Sync {
    /// Dispatches a query to its registered handler and returns the response.
    ///
    /// # Arguments
    ///
    /// * `query` - A boxed, type-erased query to dispatch.
    ///
    /// # Errors
    ///
    /// Returns [`QueryBusError`] if no handler is found or the handler fails.
    async fn ask(&self, query: Box<dyn Query>) -> Result<Box<dyn Any + Send + Sync>, QueryBusError>;
}
