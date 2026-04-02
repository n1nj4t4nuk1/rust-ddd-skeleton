//! Query handler trait definition.

use async_trait::async_trait;

use super::query::Query;
use super::query_bus_error::QueryBusError;

/// Asynchronous handler for a specific query type `Q`.
///
/// Each query type should have exactly one handler registered in the
/// [`QueryBus`](super::query_bus::QueryBus).
///
/// # Type parameters
///
/// * `Q` - The concrete [`Query`] type this handler processes.
///
/// # Associated types
///
/// * `Response` - The type returned on success. Must be `Send + Sync + 'static`
///   so it can be boxed and returned through the bus.
///
/// # Example
///
/// ```rust
/// # use shared_cqrs::query::domain::query::Query;
/// # use shared_cqrs::query::domain::query_bus_error::QueryBusError;
/// # use async_trait::async_trait;
/// # struct FindUserByIdQuery;
/// # impl Query for FindUserByIdQuery {}
/// # struct UserResponse;
/// use shared_cqrs::query::domain::query_handler::QueryHandler;
///
/// struct FindUserByIdHandler;
///
/// #[async_trait]
/// impl QueryHandler<FindUserByIdQuery> for FindUserByIdHandler {
///     type Response = UserResponse;
///
///     async fn handle(&self, _query: FindUserByIdQuery) -> Result<UserResponse, QueryBusError> {
///         Ok(UserResponse)
///     }
/// }
/// ```
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// The type returned by the handler on success.
    type Response: Send + Sync + 'static;

    /// Handles the given query and returns the response.
    ///
    /// # Arguments
    ///
    /// * `query` - The concrete query instance to process.
    ///
    /// # Errors
    ///
    /// Returns [`QueryBusError::HandlerError`] if the read logic fails.
    async fn handle(&self, query: Q) -> Result<Self::Response, QueryBusError>;
}
