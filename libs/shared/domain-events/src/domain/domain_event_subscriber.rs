//! Domain event subscriber trait definition.

use super::domain_event::DomainEvent;
use super::event_bus_error::EventBusError;

/// Handler for a specific domain event type `E`.
///
/// Subscribers are registered in an [`EventBus`] and are invoked synchronously
/// whenever a matching event is published.
///
/// # Type parameters
///
/// * `E` - The concrete [`DomainEvent`] type this subscriber handles.
///
/// # Example
///
/// ```rust
/// # use std::time::SystemTime;
/// # use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
/// # use shared_domain_events::domain::event_bus_error::EventBusError;
/// # struct UserCreatedEvent { base: DomainEventBase }
/// # impl DomainEvent for UserCreatedEvent {
/// #     fn event_name(&self) -> &'static str { "UserCreated" }
/// #     fn aggregate_id(&self) -> &str { &self.base.aggregate_id }
/// #     fn event_id(&self) -> &str { &self.base.event_id }
/// #     fn occurred_on(&self) -> SystemTime { self.base.occurred_on }
/// # }
/// use shared_domain_events::domain::domain_event_subscriber::DomainEventSubscriber;
///
/// struct SendWelcomeEmailOnUserCreated;
///
/// impl DomainEventSubscriber<UserCreatedEvent> for SendWelcomeEmailOnUserCreated {
///     fn on(&self, _event: &UserCreatedEvent) -> Result<(), EventBusError> {
///         Ok(())
///     }
/// }
/// ```
///
/// [`EventBus`]: super::event_bus::EventBus
pub trait DomainEventSubscriber<E: DomainEvent>: Send + Sync {
    /// Called when an event of type `E` is published.
    ///
    /// # Arguments
    ///
    /// * `event` - A reference to the event instance.
    ///
    /// # Errors
    ///
    /// Returns [`EventBusError::DispatchError`] if the subscriber logic fails.
    fn on(&self, event: &E) -> Result<(), EventBusError>;
}
