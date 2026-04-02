//! # Domain Events
//!
//! This crate provides the foundational building blocks for implementing the
//! Domain Events pattern in a Domain-Driven Design (DDD) architecture.
//!
//! It allows publishing events produced by aggregates and subscribing to them
//! in a decoupled manner via an event bus.
//!
//! ## Main modules
//!
//! - [`domain`]: Core traits and structs: [`DomainEvent`], [`EventBus`],
//!   [`DomainEventSubscriber`], and [`EventBusError`].
//! - [`infrastructure`]: Concrete implementations, such as the in-memory event bus.
//!
//! ## Usage example
//!
//! ```rust
//! # use std::time::SystemTime;
//! # use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
//! # use shared_domain_events::domain::domain_event_subscriber::DomainEventSubscriber;
//! # use shared_domain_events::domain::event_bus_error::EventBusError;
//! # struct MyEvent { base: DomainEventBase }
//! # impl DomainEvent for MyEvent {
//! #     fn event_name(&self) -> &'static str { "MyEvent" }
//! #     fn aggregate_id(&self) -> &str { &self.base.aggregate_id }
//! #     fn event_id(&self) -> &str { &self.base.event_id }
//! #     fn occurred_on(&self) -> SystemTime { self.base.occurred_on }
//! # }
//! # struct MySubscriber;
//! # impl DomainEventSubscriber<MyEvent> for MySubscriber {
//! #     fn on(&self, _: &MyEvent) -> Result<(), EventBusError> { Ok(()) }
//! # }
//! use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;
//! use shared_domain_events::domain::event_bus::EventBus;
//!
//! let mut bus = InMemoryEventBus::new();
//! bus.add_subscriber(MySubscriber);
//! # let my_event = MyEvent { base: DomainEventBase::new("agg-1") };
//! bus.publish(vec![Box::new(my_event)]).unwrap();
//! ```
//!
//! [`DomainEvent`]: domain::domain_event::DomainEvent
//! [`EventBus`]: domain::event_bus::EventBus
//! [`DomainEventSubscriber`]: domain::domain_event_subscriber::DomainEventSubscriber
//! [`EventBusError`]: domain::event_bus_error::EventBusError

pub mod domain;
pub mod infrastructure;
