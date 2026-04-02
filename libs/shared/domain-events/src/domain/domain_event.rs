//! Core traits and base struct for domain events.

use std::any::{Any, TypeId};
use std::time::SystemTime;
use uuid::Uuid;

/// Helper trait that enables type erasure for domain events, allowing them to
/// be stored and dispatched as `dyn Any` in a thread-safe manner.
///
/// Implemented automatically for any type that is `Any + Send + Sync`.
pub trait AnyDomainEvent: Any + Send + Sync {
    /// Returns a reference to `self` as `&dyn Any` for downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Returns the [`TypeId`] of the concrete event type.
    ///
    /// Used internally by the event bus to route to the correct subscribers.
    fn domain_event_type_id(&self) -> TypeId;
}

impl<T: Any + Send + Sync> AnyDomainEvent for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn domain_event_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

/// Trait that all domain events must implement.
///
/// Provides the metadata required to identify, trace, and timestamp an event.
/// Concrete event types should embed a [`DomainEventBase`] to satisfy the
/// standard fields.
///
/// # Example
///
/// ```rust
/// use std::time::SystemTime;
/// use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
///
/// struct UserCreatedEvent {
///     base: DomainEventBase,
///     username: String,
/// }
///
/// impl DomainEvent for UserCreatedEvent {
///     fn event_name(&self) -> &'static str { "UserCreated" }
///     fn aggregate_id(&self) -> &str { &self.base.aggregate_id }
///     fn event_id(&self) -> &str { &self.base.event_id }
///     fn occurred_on(&self) -> SystemTime { self.base.occurred_on }
/// }
/// ```
pub trait DomainEvent: AnyDomainEvent {
    /// Returns the unique name of this event type (e.g. `"UserCreated"`).
    fn event_name(&self) -> &'static str;

    /// Returns the ID of the aggregate that produced the event.
    fn aggregate_id(&self) -> &str;

    /// Returns the unique identifier for this specific event instance.
    fn event_id(&self) -> &str;

    /// Returns the timestamp at which the event occurred.
    fn occurred_on(&self) -> SystemTime;
}

/// Shared metadata fields for all domain events.
///
/// Embed this struct inside a concrete event type and delegate the
/// [`DomainEvent`] methods to its fields to avoid repetition.
pub struct DomainEventBase {
    /// Identifier of the aggregate that produced the event.
    pub aggregate_id: String,
    /// Unique identifier for this event instance (UUID v4).
    pub event_id: String,
    /// Timestamp at which the event was created.
    pub occurred_on: SystemTime,
}

impl DomainEventBase {
    /// Creates a new `DomainEventBase` with a generated UUID and the current time.
    ///
    /// # Arguments
    ///
    /// * `aggregate_id` - The identifier of the aggregate that raised the event.
    pub fn new(aggregate_id: impl Into<String>) -> Self {
        Self {
            aggregate_id: aggregate_id.into(),
            event_id: Uuid::new_v4().to_string(),
            occurred_on: SystemTime::now(),
        }
    }

    /// Creates a `DomainEventBase` from explicit primitive values.
    ///
    /// Useful when reconstituting events from a persistent store.
    ///
    /// # Arguments
    ///
    /// * `aggregate_id` - The identifier of the aggregate.
    /// * `event_id` - A pre-existing event identifier.
    /// * `occurred_on` - The original timestamp of the event.
    pub fn from_primitives(
        aggregate_id: impl Into<String>,
        event_id: impl Into<String>,
        occurred_on: SystemTime,
    ) -> Self {
        Self {
            aggregate_id: aggregate_id.into(),
            event_id: event_id.into(),
            occurred_on,
        }
    }
}
