# Domain Events

Domain events record facts that happened inside the domain. They decouple the code that causes a state change from the code that reacts to it.

## Why domain events?

Consider what happens when you save a new config entry: the system might also need to write an audit log, invalidate a cache, or notify another bounded context. Without events, the `ConfigEntryCreator` service would have to import and call each of those services directly. That means the creator -- which belongs to the `config_entry` module -- suddenly depends on every module that cares about new entries. Two unrelated concerns become tightly coupled, and the problem compounds every time you add another side effect.

Domain events break that coupling. After saving the new config entry, the creator simply publishes a `ConfigEntryCreatedEvent`. It does not know or care who is listening. Separately, any number of subscribers can listen for that specific event and perform their own work. Neither side imports the other. You can add, remove, or modify subscribers without touching the code that publishes events, and the publisher can be tested in complete isolation from any subscriber.

The practical benefit is extensibility. Need to send a notification when a config entry is created? Add a subscriber. Need to log every deletion? Add a subscriber. Need to synchronize data to an external system? Add a subscriber. The publishing code never changes, and existing subscribers are unaffected by new ones. This keeps each piece of the system small, focused, and independently testable.

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

The current implementation is synchronous and in-process -- when an event is published, all subscribers run before the `publish` call returns. This is simple and sufficient for a single-process application. If the system grows to need asynchronous processing or cross-service events, the `EventBus` trait can be implemented with a message broker (RabbitMQ, Kafka) without changing any domain code, because the domain only depends on the trait, never on the concrete implementation.

`InMemoryEventBus` holds a `HashMap<TypeId, Vec<HandlerFn>>`. When `publish` is called it iterates the events, looks up each `TypeId`, and calls every registered handler for that type.

```
EventBus::publish([ConfigEntryCreatedEvent, ...])
  |
  ├─▶ TypeId::of::<ConfigEntryCreatedEvent>()
  |     └─▶ subscriber_1.on(&event)
  |     └─▶ subscriber_2.on(&event)
  |
  └─▶ (other event types)
```

## Event factory functions

Events are created through dedicated factory functions rather than calling constructors directly. This keeps event construction logic in one place, isolates it from the application service, and makes it easy to ensure all required fields are populated from the aggregate. If the event structure changes, only the factory function needs updating -- the application service that publishes the event remains untouched.

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

The order of operations matters: events are always published **after** the repository write succeeds. This ensures that subscribers never react to changes that were not actually persisted. If the save fails, no event is published and no side effects happen. This "persist first, publish second" rule is a simple but important guarantee that keeps the system consistent.

```rust
// Inside ConfigEntryCreator::execute()
self.repository.save(entry.clone()).await?;

let event = create_config_entry_created_event(&entry);
self.event_bus
    .publish(vec![Box::new(event)])
    .await?;
```

The event is published **after** the repository operation succeeds. If the save fails, no event is published -- ensuring the bus only receives facts that actually happened.

## Adding a new event

The following checklist walks you through adding a new event to the system. Each step is small and self-contained, so you can verify correctness incrementally.

1. Create the event struct in `libs/<context>/src/<context>/domain/events/<noun>_<past>_event.rs`.
2. Implement `DomainEvent` and declare `EVENT_NAME`.
3. Create a factory function `create_<noun>_<past>_event.rs`.
4. Expose it from the domain's `events/mod.rs`.
5. Publish it from the relevant application service.

## Adding a subscriber

The following checklist walks you through adding a subscriber that reacts to an existing event. Subscribers are the extension point of the event system -- you can add as many as you need without modifying the publishing code.

1. Create a struct that implements `DomainEventSubscriber<YourEvent>`.
2. Register it on the `EventBus` inside `build_state()`:

```rust
event_bus.subscribe::<ConfigEntryCreatedEvent, _>(
    Arc::new(MySubscriber::new(...))
);
```

The current template does not ship any subscribers -- the event bus infrastructure is wired and ready but the subscriber list is empty. Add your first subscriber when you have a cross-cutting concern (notifications, audit log, cache invalidation, etc.).
