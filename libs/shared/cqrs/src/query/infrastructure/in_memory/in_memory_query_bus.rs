//! In-memory implementation of the [`QueryBus`] trait.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;

use crate::query::domain::query::Query;
use crate::query::domain::query_bus::QueryBus;
use crate::query::domain::query_bus_error::QueryBusError;
use crate::query::domain::query_handler::QueryHandler;

/// Type alias for a type-erased, heap-allocated async handler function
/// that returns a boxed response.
type HandlerFn = Box<
    dyn Fn(
            Box<dyn Any + Send + Sync>,
        ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send + Sync>, QueryBusError>> + Send>>
        + Send
        + Sync,
>;

/// An in-memory [`QueryBus`] that stores handlers in a [`HashMap`] keyed by query [`TypeId`].
///
/// Handlers are registered at startup and looked up at dispatch time using
/// Rust's type system. Each query type may have at most one handler.
///
/// The response is returned as `Box<dyn Any + Send + Sync>` and must be
/// downcast by the caller to the expected concrete type.
///
/// # Example
///
/// ```rust
/// # use shared_cqrs::query::domain::query::Query;
/// # use shared_cqrs::query::domain::query_bus::QueryBus;
/// # use shared_cqrs::query::domain::query_bus_error::QueryBusError;
/// # use shared_cqrs::query::domain::query_handler::QueryHandler;
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
/// use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
///
/// # #[tokio::main]
/// # async fn main() {
/// let mut bus = InMemoryQueryBus::new();
/// bus.register(FindUserByIdHandler).unwrap();
///
/// let raw = bus.ask(Box::new(FindUserByIdQuery { id: "123".into() })).await.unwrap();
/// let user = raw.downcast::<UserResponse>().unwrap();
/// # }
/// ```
pub struct InMemoryQueryBus {
    handlers: HashMap<TypeId, HandlerFn>,
}

impl InMemoryQueryBus {
    /// Creates a new, empty `InMemoryQueryBus`.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a handler for a specific query type `Q`.
    ///
    /// # Type parameters
    ///
    /// * `Q` - The concrete [`Query`] type the handler processes.
    /// * `H` - A [`QueryHandler<Q>`] implementation.
    ///
    /// # Errors
    ///
    /// Returns [`QueryBusError::HandlerAlreadyRegistered`] if a handler for
    /// `Q` has already been registered.
    pub fn register<Q, H>(&mut self, handler: H) -> Result<(), QueryBusError>
    where
        Q: Query + 'static,
        H: QueryHandler<Q> + 'static,
    {
        let type_id = TypeId::of::<Q>();

        if self.handlers.contains_key(&type_id) {
            return Err(QueryBusError::HandlerAlreadyRegistered(type_id));
        }

        let handler = Arc::new(handler);
        self.handlers.insert(
            type_id,
            Box::new(move |query: Box<dyn Any + Send + Sync>| {
                let handler = Arc::clone(&handler);
                let query = match query.downcast::<Q>() {
                    Ok(q) => *q,
                    Err(_) => {
                        return Box::pin(async move {
                            Err(QueryBusError::HandlerNotFound(TypeId::of::<Q>()))
                        })
                            as Pin<
                                Box<
                                    dyn Future<
                                            Output = Result<
                                                Box<dyn Any + Send + Sync>,
                                                QueryBusError,
                                            >,
                                        > + Send,
                                >,
                            >;
                    }
                };
                Box::pin(async move {
                    let response = handler.handle(query).await?;
                    Ok(Box::new(response) as Box<dyn Any + Send + Sync>)
                })
            }),
        );

        Ok(())
    }
}

impl Default for InMemoryQueryBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl QueryBus for InMemoryQueryBus {
    /// Dispatches the query to its registered handler and returns the response.
    ///
    /// # Errors
    ///
    /// Returns [`QueryBusError::HandlerNotFound`] if no handler is registered
    /// for the query's type.
    async fn ask(
        &self,
        query: Box<dyn Query>,
    ) -> Result<Box<dyn Any + Send + Sync>, QueryBusError> {
        let type_id = query.query_type_id();

        match self.handlers.get(&type_id) {
            Some(handler) => handler(query.into_any()).await,
            None => Err(QueryBusError::HandlerNotFound(type_id)),
        }
    }
}
