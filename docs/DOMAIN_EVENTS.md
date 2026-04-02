# Domain Events

Domain events record facts that happened inside the domain. They decouple the code that causes a state change from the code that reacts to it.

## Core concepts

| Concept | Role |
|---|---|
| `DomainEvent` | Trait that all events implement |
| `DomainEventBase` | Common metadata: `id`, `name`, `occurred_on` |
| `EventBus` | Delivers a batch of events to all registered subscribers |
| `DomainEventSubscriber<E>` | Reacts to one specific event type |

## DomainEvent trait

```rust
// libs/shared/domain-events/src/domain/domain_event.rs
pub trait DomainEvent: Any + Send + Sync {
    fn base(&self) -> &DomainEventBase;
    fn name(&self) -> &'static str;
}

pub struct DomainEventBase {
    pub id: Uuid,
    pub name: String,
    pub occurred_on: String,   // ISO-8601
}
```

Every concrete event also declares an `EVENT_NAME` constant used by subscribers and tests:

```rust
impl ConfigEntryCreatedEvent {
    pub const EVENT_NAME: &'static str = "config_entry.created";
}
```

## EventBus trait

```rust
#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError>;
    fn subscribe<E, S>(&mut self, subscriber: Arc<S>)
    where
        E: DomainEvent + 'static,
        S: DomainEventSubscriber<E> + 'static;
}
```

## DomainEventSubscriber trait

```rust
#[async_trait]
pub trait DomainEventSubscriber<E: DomainEvent>: Send + Sync {
    async fn on(&self, event: &E) -> Result<(), Box<dyn Error + Send + Sync>>;
}
```

## In-memory implementation

`InMemoryEventBus` holds a `HashMap<TypeId, Vec<HandlerFn>>`. When `publish` is called it iterates the events, looks up each `TypeId`, and calls every registered handler for that type.

```
EventBus::publish([ConfigEntryCreatedEvent, ...])
  │
  ├─▶ TypeId::of::<ConfigEntryCreatedEvent>()
  │     └─▶ subscriber_1.on(&event)
  │     └─▶ subscriber_2.on(&event)
  │
  └─▶ (other event types)
```

## Event factory functions

Creating an event from an aggregate is done by dedicated factory functions, keeping the event construction logic isolated and testable:

```rust
// libs/config/src/config_entry/domain/events/create_config_entry_created_event.rs
pub fn create_config_entry_created_event(entry: &ConfigEntry) -> ConfigEntryCreatedEvent {
    ConfigEntryCreatedEvent {
        base: DomainEventBase::new(ConfigEntryCreatedEvent::EVENT_NAME),
        key: entry.key().to_string(),
        value: entry.value().to_string(),
    }
}
```

## Lifecycle in a use case

```rust
// Inside ConfigEntryCreator::execute()
self.repository.save(entry.clone()).await?;

let event = create_config_entry_created_event(&entry);
self.event_bus
    .publish(vec![Box::new(event)])
    .await?;
```

The event is published **after** the repository operation succeeds. If the save fails, no event is published — ensuring the bus only receives facts that actually happened.

## Adding a new event

1. Create the event struct in `libs/<context>/src/<context>/domain/events/<noun>_<past>_event.rs`.
2. Implement `DomainEvent` and declare `EVENT_NAME`.
3. Create a factory function `create_<noun>_<past>_event.rs`.
4. Expose it from the domain's `events/mod.rs`.
5. Publish it from the relevant application service.

## Adding a subscriber

1. Create a struct that implements `DomainEventSubscriber<YourEvent>`.
2. Register it on the `EventBus` inside `build_state()`:

```rust
event_bus.subscribe::<ConfigEntryCreatedEvent, _>(
    Arc::new(MySubscriber::new(...))
);
```

The current template does not ship any subscribers — the event bus infrastructure is wired and ready but the subscriber list is empty. Add your first subscriber when you have a cross-cutting concern (notifications, audit log, cache invalidation, etc.).
