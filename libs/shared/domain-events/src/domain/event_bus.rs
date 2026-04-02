//! Event bus trait definition.

use super::domain_event::DomainEvent;
use super::event_bus_error::EventBusError;

/// Synchronous publisher for domain events.
///
/// Implementations are responsible for routing each [`DomainEvent`] in the
/// provided batch to all registered [`DomainEventSubscriber`]s for that event type.
///
/// # Errors
///
/// Returns [`EventBusError::DispatchError`] if any subscriber fails.
///
/// # Example
///
/// ```rust
/// # use std::time::SystemTime;
/// # use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
/// # use shared_domain_events::domain::event_bus_error::EventBusError;
/// # use shared_domain_events::domain::event_bus::EventBus;
/// # use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;
/// # struct UserCreatedEvent { base: DomainEventBase }
/// # impl DomainEvent for UserCreatedEvent {
/// #     fn event_name(&self) -> &'static str { "UserCreated" }
/// #     fn aggregate_id(&self) -> &str { &self.base.aggregate_id }
/// #     fn event_id(&self) -> &str { &self.base.event_id }
/// #     fn occurred_on(&self) -> SystemTime { self.base.occurred_on }
/// # }
/// # let bus = InMemoryEventBus::new();
/// # let user_created_event = UserCreatedEvent { base: DomainEventBase::new("agg-1") };
/// bus.publish(vec![Box::new(user_created_event)])?;
/// # Ok::<(), EventBusError>(())
/// ```
///
/// [`DomainEventSubscriber`]: super::domain_event_subscriber::DomainEventSubscriber
pub trait EventBus: Send + Sync {
    /// Publishes a batch of domain events to all matching subscribers.
    ///
    /// # Arguments
    ///
    /// * `events` - A list of boxed, type-erased domain events to publish.
    ///
    /// # Errors
    ///
    /// Returns [`EventBusError`] if dispatch to any subscriber fails.
    fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError>;
}
