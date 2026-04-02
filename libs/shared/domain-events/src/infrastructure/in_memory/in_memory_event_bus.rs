//! In-memory implementation of the [`EventBus`] trait.

use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::domain::domain_event::DomainEvent;
use crate::domain::domain_event_subscriber::DomainEventSubscriber;
use crate::domain::event_bus::EventBus;
use crate::domain::event_bus_error::EventBusError;

/// Type alias for a type-erased, heap-allocated synchronous subscriber function.
type HandlerFn = Box<dyn Fn(&dyn Any) -> Result<(), EventBusError> + Send + Sync>;

/// An in-memory [`EventBus`] that stores subscriber functions in a [`HashMap`]
/// keyed by event [`TypeId`].
///
/// Multiple subscribers can be registered for the same event type. They are
/// invoked in registration order when the event is published.
///
/// # Example
///
/// ```rust
/// # use std::time::SystemTime;
/// # use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
/// # use shared_domain_events::domain::domain_event_subscriber::DomainEventSubscriber;
/// # use shared_domain_events::domain::event_bus::EventBus;
/// # use shared_domain_events::domain::event_bus_error::EventBusError;
/// # struct UserCreatedEvent { base: DomainEventBase }
/// # impl DomainEvent for UserCreatedEvent {
/// #     fn event_name(&self) -> &'static str { "UserCreated" }
/// #     fn aggregate_id(&self) -> &str { &self.base.aggregate_id }
/// #     fn event_id(&self) -> &str { &self.base.event_id }
/// #     fn occurred_on(&self) -> SystemTime { self.base.occurred_on }
/// # }
/// # struct SendWelcomeEmailOnUserCreated;
/// # impl DomainEventSubscriber<UserCreatedEvent> for SendWelcomeEmailOnUserCreated {
/// #     fn on(&self, _: &UserCreatedEvent) -> Result<(), EventBusError> { Ok(()) }
/// # }
/// # struct AuditLogOnUserCreated;
/// # impl DomainEventSubscriber<UserCreatedEvent> for AuditLogOnUserCreated {
/// #     fn on(&self, _: &UserCreatedEvent) -> Result<(), EventBusError> { Ok(()) }
/// # }
/// use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;
///
/// let mut bus = InMemoryEventBus::new();
/// bus.add_subscriber(SendWelcomeEmailOnUserCreated);
/// bus.add_subscriber(AuditLogOnUserCreated);
///
/// # let user_created_event = UserCreatedEvent { base: DomainEventBase::new("agg-1") };
/// bus.publish(vec![Box::new(user_created_event)])?;
/// # Ok::<(), EventBusError>(())
/// ```
pub struct InMemoryEventBus {
    handlers: HashMap<TypeId, Vec<HandlerFn>>,
}

impl InMemoryEventBus {
    /// Creates a new, empty `InMemoryEventBus`.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a subscriber for a specific domain event type `E`.
    ///
    /// Multiple subscribers can be added for the same event type.
    ///
    /// # Type parameters
    ///
    /// * `E` - The concrete [`DomainEvent`] type the subscriber handles.
    /// * `S` - A [`DomainEventSubscriber<E>`] implementation.
    pub fn add_subscriber<E, S>(&mut self, subscriber: S)
    where
        E: DomainEvent + 'static,
        S: DomainEventSubscriber<E> + 'static,
    {
        let type_id = TypeId::of::<E>();
        let handler: HandlerFn = Box::new(move |event: &dyn Any| {
            let event = event
                .downcast_ref::<E>()
                .ok_or_else(|| EventBusError::DispatchError("type mismatch".to_string()))?;
            subscriber.on(event)
        });
        self.handlers.entry(type_id).or_default().push(handler);
    }
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus for InMemoryEventBus {
    /// Publishes a batch of events, invoking all matching subscribers for each one.
    ///
    /// Events with no registered subscribers are silently ignored.
    ///
    /// # Errors
    ///
    /// Returns [`EventBusError`] immediately if any subscriber fails,
    /// without processing subsequent events or subscribers.
    fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError> {
        for event in &events {
            let type_id = event.domain_event_type_id();
            if let Some(handlers) = self.handlers.get(&type_id) {
                for handler in handlers {
                    handler(event.as_any())?;
                }
            }
        }
        Ok(())
    }
}
