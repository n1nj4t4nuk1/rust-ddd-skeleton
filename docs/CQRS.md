# CQRS

Command Query Responsibility Segregation separates write operations (commands) from read operations (queries). This template provides in-memory bus implementations for both.

## Core concepts

| Concept | Role |
|---|---|
| `Command` | Intent to change state — no return value beyond `()` |
| `Query` | Request for data — returns a typed response |
| `CommandBus` | Routes a command to its registered handler |
| `QueryBus` | Routes a query to its registered handler, returns the response |
| `CommandHandler<C>` | Processes one specific command type |
| `QueryHandler` | Processes one specific query type, declares its `Response` type |

## Command flow

```
HTTP handler
  │
  └─▶ CommandBus::dispatch(Box<dyn Command>)
        │
        └─▶ InMemoryCommandBus
              │  looks up handler by TypeId
              └─▶ CommandHandler<CreateConfigEntryCommand>::handle(cmd)
                    │
                    ├─▶ Domain service (ConfigEntryCreator)
                    │     ├─▶ Repository::save(entry)
                    │     └─▶ EventBus::publish(events)
                    └─▶ Ok(())
```

## Query flow

```
HTTP handler
  │
  └─▶ QueryBus::ask(Box<dyn Query>)
        │
        └─▶ InMemoryQueryBus
              │  looks up handler by TypeId
              └─▶ QueryHandler::handle(query) → Box<dyn Any + Send + Sync>
                    │
                    └─▶ Domain service (ConfigEntryFinder)
                          └─▶ Repository::find_by_key(key) → FindConfigEntryResponse
```

The HTTP handler then downcasts the `Box<dyn Any>` to the concrete response type.

## Traits

### Command marker trait

```rust
// libs/shared/cqrs/src/command/domain/command.rs
pub trait Command: Any + Send + Sync {}
pub type AnyCommand = Box<dyn Command>;
```

### CommandHandler

```rust
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    async fn handle(&self, command: C) -> Result<(), Box<dyn Error + Send + Sync>>;
}
```

### CommandBus

```rust
#[async_trait]
pub trait CommandBus: Send + Sync {
    async fn dispatch(&self, command: AnyCommand) -> Result<(), CommandBusError>;

    fn register<C, H>(&mut self, handler: Arc<H>)
    where
        C: Command + 'static,
        H: CommandHandler<C> + 'static;
}
```

### QueryHandler

```rust
#[async_trait]
pub trait QueryHandler: Send + Sync {
    type Response: Any + Send + Sync;
    async fn handle(&self, query: AnyQuery) -> Result<Self::Response, Box<dyn Error + Send + Sync>>;
}
```

### QueryBus

```rust
#[async_trait]
pub trait QueryBus: Send + Sync {
    async fn ask(&self, query: AnyQuery) -> Result<Box<dyn Any + Send + Sync>, QueryBusError>;

    fn register<Q, H>(&mut self, handler: Arc<H>)
    where
        Q: Query + 'static,
        H: QueryHandler + 'static;
}
```

## In-memory implementations

Both `InMemoryCommandBus` and `InMemoryQueryBus` store handlers in a `HashMap<TypeId, HandlerFn>`. The `TypeId` is derived from the concrete command/query type at registration time and used again at dispatch time for O(1) lookup.

```rust
// Registration (from build_state)
command_bus.register::<CreateConfigEntryCommand, _>(creator_handler.clone());

// Dispatch (from HTTP handler)
command_bus.dispatch(Box::new(CreateConfigEntryCommand { key, value })).await?;
```

## Wiring in build_state()

All buses are built and handlers registered inside `build_state()` in `apps/config_api/src/lib.rs`. The result is stored in `web::Data<AppState>` so every request handler gets shared access through Actix's extractor system.

```rust
pub fn build_state() -> web::Data<AppState> {
    let repo = Arc::new(InMemoryConfigEntryRepository::new());
    let event_bus = Arc::new(InMemoryEventBus::new());

    // Domain services
    let creator = Arc::new(ConfigEntryCreator::new(repo.clone(), event_bus.clone()));
    let finder  = Arc::new(ConfigEntryFinder::new(repo.clone()));
    let updater = Arc::new(ConfigEntryUpdater::new(repo.clone(), event_bus.clone()));
    let deleter = Arc::new(ConfigEntryDeleter::new(repo.clone(), event_bus.clone()));

    // Command handlers
    let mut command_bus = InMemoryCommandBus::new();
    command_bus.register::<CreateConfigEntryCommand, _>(
        Arc::new(CreateConfigEntryCommandHandler::new(creator))
    );
    // ... updater, deleter handlers

    // Query handlers
    let mut query_bus = InMemoryQueryBus::new();
    query_bus.register::<FindConfigEntryQuery, _>(
        Arc::new(FindConfigEntryQueryHandler::new(finder))
    );

    web::Data::new(AppState {
        command_bus: Arc::new(command_bus),
        query_bus:   Arc::new(query_bus),
    })
}
```

## Adding a new use case

1. Create a command or query struct in `libs/<context>/src/<context>/application/<use_case>/`.
2. Create a domain service that calls the repository and/or event bus.
3. Create a command handler or query handler that delegates to the service.
4. Register the handler in `build_state()`.
5. Add an HTTP handler in `apps/<api>/src/<context>/` that dispatches to the bus.
