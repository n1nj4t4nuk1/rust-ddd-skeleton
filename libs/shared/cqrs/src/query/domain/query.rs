//! Base traits for defining queries in the CQRS pattern.

use std::any::{Any, TypeId};

/// Helper trait that enables type erasure for queries, allowing them to be
/// handled as `dyn Any` in a thread-safe manner.
///
/// This trait is implemented automatically for any type that is `Any + Send + Sync`,
/// so manual implementation is not required.
pub trait AnyQuery: Any + Send + Sync {
    /// Consumes the query and returns it as a `Box<dyn Any + Send + Sync>`.
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync>;

    /// Returns the [`TypeId`] of the concrete query type.
    ///
    /// Used internally by the query bus to route to the correct handler.
    fn query_type_id(&self) -> TypeId;
}

impl<T: Any + Send + Sync> AnyQuery for T {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync> {
        self
    }

    fn query_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

/// Marker trait for types that represent a query.
///
/// Implementing this trait signals that a type can be dispatched through
/// a [`QueryBus`](super::query_bus::QueryBus).
///
/// # Example
///
/// ```rust
/// use shared_cqrs::query::domain::query::Query;
///
/// struct FindUserByIdQuery {
///     pub id: String,
/// }
///
/// impl Query for FindUserByIdQuery {}
/// ```
pub trait Query: AnyQuery {}
