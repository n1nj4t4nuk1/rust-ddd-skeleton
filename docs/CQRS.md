# CQRS

Command Query Responsibility Segregation separates write operations (commands) from read operations (queries). The `shared-cqrs` crate provides in-memory bus implementations used by every app in the workspace.

## Why CQRS?

In practical terms, every HTTP endpoint either changes data (POST, PUT, DELETE) or reads it (GET). CQRS formalizes this split -- write operations go through the `CommandBus`, read operations go through the `QueryBus`. Each side has its own handler, its own response type, and its own optimization path. This is not an abstract architectural preference; it mirrors the reality of how web services work.

The benefit is that handlers stay small and focused. A create handler only creates. A find handler only finds. There is no temptation to mix read and write logic in the same place, no risk of a query accidentally triggering a side effect, and no reason for a command handler to assemble a complex read model. Each handler does one thing, which makes it straightforward to test, review, and maintain.

At runtime the mechanism is simple. The bus receives a boxed command or query, looks up the matching handler by `TypeId` (Rust's built-in type identity system), calls the handler, and returns the response boxed as `dyn Any`. The caller then downcasts the response to the expected concrete type. This is the same dispatch pattern used in actor frameworks and event systems -- it trades a small amount of type safety at the boundary (the downcast) for complete decoupling between the bus infrastructure and the concrete handler types.

## Core concepts

| Concept | Role |
|---|---|
| `Command` | Intent to change state -- dispatched on the command bus, returns a typed response |
| `Query` | Request for data -- dispatched on the query bus, returns a typed response |
| `CommandBus` | Routes a command to its registered handler, returns `Box<dyn Any + Send + Sync>` |
| `QueryBus` | Routes a query to its registered handler, returns `Box<dyn Any + Send + Sync>` |
| `CommandHandler<C>` | Processes one command type; declares `type Response` |
| `QueryHandler<Q>` | Processes one query type; declares `type Response` |

The following diagrams show how a command and a query travel through the system, from HTTP handler to domain service and back.

## Command flow

```
HTTP handler
  │
  └── CommandBus::dispatch(Box<dyn Command>)
        │
        └── InMemoryCommandBus
              │  TypeId lookup -> HandlerFn
              └── CommandHandler<CreateConfigEntryCommand>::handle(cmd)
                    │                        -> Result<CreateConfigEntryResponse, CommandBusError>
                    └── ConfigEntryCreator::execute(...)
                          ├── ConfigEntryRepository::save(&entry)
                          └── EventBus::publish([ConfigEntryCreatedEvent])

HTTP handler receives Box<dyn Any>, downcasts to CreateConfigEntryResponse,
checks response.error to determine HTTP status code.
```

## Query flow

```
HTTP handler
  │
  └── QueryBus::ask(Box<dyn Query>)
        │
        └── InMemoryQueryBus
              │  TypeId lookup -> HandlerFn
              └── QueryHandler<FindConfigEntryQuery>::handle(query)
                    │                        -> Result<FindConfigEntryResponse, QueryBusError>
                    └── ConfigEntryFinder::execute(key)
                          └── ConfigEntryRepository::find_by_key(&key) -> ConfigEntry (domain entity)

QueryHandler maps the domain entity to a FindConfigEntryResponse { config_entry: Some(ConfigEntryEntry { ... }), error: None }.
HTTP handler downcasts Box<dyn Any> to FindConfigEntryResponse and checks response.error.
```

## Traits

### Command marker trait

```rust
// libs/shared/cqrs/src/command/domain/command.rs
pub trait Command: AnyCommand {}

pub trait AnyCommand: Any + Send + Sync {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync>;
    fn command_type_id(&self) -> TypeId;
}
```

### CommandHandler

```rust
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    type Response: Send + Sync + 'static;

    async fn handle(&self, command: C) -> Result<Self::Response, CommandBusError>;
}
```

### CommandBus

```rust
#[async_trait]
pub trait CommandBus: Send + Sync {
    async fn dispatch(
        &self,
        command: Box<dyn Command>,
    ) -> Result<Box<dyn Any + Send + Sync>, CommandBusError>;
}
```

### QueryHandler

```rust
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    type Response: Send + Sync + 'static;

    async fn handle(&self, query: Q) -> Result<Self::Response, QueryBusError>;
}
```

### QueryBus

```rust
#[async_trait]
pub trait QueryBus: Send + Sync {
    async fn ask(
        &self,
        query: Box<dyn Query>,
    ) -> Result<Box<dyn Any + Send + Sync>, QueryBusError>;
}
```

A key design decision is how handlers communicate success and failure back to the HTTP layer. Rather than using Rust's `Result` to carry domain errors (which would couple the bus infrastructure to domain error types), handlers always return `Ok(Response)` -- even when the domain operation fails. The response struct carries an optional error field that the controller inspects to determine the HTTP status code.

## Response envelope pattern

Both command and query handlers return a **response envelope** -- a struct that carries either the data or a structured error, never both.

### Command responses (no data, only error)

```rust
pub struct CreateConfigEntryResponse {
    pub error: Option<ConfigEntryErrorEntry>,
}
```

### Query responses (data + error)

```rust
pub struct FindConfigEntryResponse {
    pub config_entry: Option<ConfigEntryEntry>,  // data DTO
    pub error: Option<ConfigEntryErrorEntry>,     // structured error
}
```

### ErrorEntry structure

Every context defines an `ErrorEntry` with two fields:

```rust
pub struct ConfigEntryErrorEntry {
    pub message: String,    // human-readable error message
    pub concept: String,    // PascalCase: "NotFound", "AlreadyExists", "Unexpected"
}
```

### Handler pattern -- command example

```rust
#[async_trait]
impl CommandHandler<CreateConfigEntryCommand> for CreateConfigEntryCommandHandler {
    type Response = CreateConfigEntryResponse;

    async fn handle(&self, command: CreateConfigEntryCommand) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.key, command.value).await {
            Ok(()) => Ok(CreateConfigEntryResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    ConfigEntryRepositoryError::NotFound => "NotFound",
                    ConfigEntryRepositoryError::AlreadyExists => "AlreadyExists",
                    ConfigEntryRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateConfigEntryResponse {
                    error: Some(ConfigEntryErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
```

### Handler pattern -- query example

The **finder** (domain service) returns domain entities. The **handler** maps them to response DTOs:

```rust
#[async_trait]
impl QueryHandler<FindConfigEntryQuery> for FindConfigEntryQueryHandler {
    type Response = FindConfigEntryResponse;

    async fn handle(&self, query: FindConfigEntryQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.key).await {
            Ok(entry) => Ok(FindConfigEntryResponse {
                config_entry: Some(ConfigEntryEntry {
                    key: entry.key().value().to_string(),
                    value: entry.value().value().to_string(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    ConfigEntryRepositoryError::NotFound => "NotFound",
                    ConfigEntryRepositoryError::AlreadyExists => "AlreadyExists",
                    ConfigEntryRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindConfigEntryResponse {
                    config_entry: None,
                    error: Some(ConfigEntryErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
```

### Controller pattern -- using the envelope

Controllers downcast the `Box<dyn Any>` and check the error field:

```rust
match state.command_bus.dispatch(Box::new(command)).await {
    Ok(boxed) => {
        let response = boxed
            .downcast::<CreateConfigEntryResponse>()
            .expect("Unexpected response type from CreateConfigEntryCommandHandler");

        if let Some(ref error) = response.error {
            match error.concept.as_str() {
                "AlreadyExists" => HttpResponse::Conflict().body(error.message.clone()),
                "NotFound" => HttpResponse::NotFound().body(error.message.clone()),
                _ => HttpResponse::InternalServerError().body(error.message.clone()),
            }
        } else {
            HttpResponse::Created().finish()
        }
    }
    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
}
```

The bus implementations are intentionally simple. There is no async message queue, no serialization, no network hop. Everything happens in-process and in-memory. This keeps latency minimal and makes the system easy to debug -- you can step through a command's entire lifecycle in a single stack trace.

## In-memory implementations

Both `InMemoryCommandBus` and `InMemoryQueryBus` store handlers in a `HashMap<TypeId, HandlerFn>`. The `TypeId` is derived from the concrete command/query type at registration time and used again at dispatch time for O(1) lookup. No configuration file or string keys are needed.

The response is boxed as `Box<dyn Any + Send + Sync>` inside the bus and must be downcast by the caller.

```rust
// Registration (in build_state)
command_bus.register(CreateConfigEntryCommandHandler::new(creator))
    .expect("Failed to register CreateConfigEntryCommandHandler");

// Dispatch (in HTTP handler)
let boxed = state.command_bus
    .dispatch(Box::new(CreateConfigEntryCommand { key, value }))
    .await?;
let response = boxed.downcast::<CreateConfigEntryResponse>().unwrap();
```

## Wiring in build_state()

All buses are built and handlers registered inside `build_state()` in each app's `lib.rs`. The `AppState` struct holds the buses behind `Arc<dyn CommandBus>` / `Arc<dyn QueryBus>` so HTTP handlers can extract them via Actix's `web::Data<AppState>` extractor.

```rust
pub fn build_state() -> web::Data<AppState> {
    let repo: Arc<dyn ConfigEntryRepository> = Arc::new(InMemoryConfigEntryRepository::new());
    let event_bus: Arc<dyn EventBus> = Arc::new(InMemoryEventBus::new());

    // Domain services
    let creator = ConfigEntryCreator::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let finder  = ConfigEntryFinder::new(Arc::clone(&repo));
    let updater = ConfigEntryUpdater::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let deleter = ConfigEntryDeleter::new(Arc::clone(&repo), Arc::clone(&event_bus));

    // Command handlers
    let mut command_bus = InMemoryCommandBus::new();
    command_bus.register(CreateConfigEntryCommandHandler::new(creator))
        .expect("Failed to register CreateConfigEntryCommandHandler");
    command_bus.register(UpdateConfigEntryCommandHandler::new(updater))
        .expect("Failed to register UpdateConfigEntryCommandHandler");
    command_bus.register(DeleteConfigEntryCommandHandler::new(deleter))
        .expect("Failed to register DeleteConfigEntryCommandHandler");

    // Query handlers
    let mut query_bus = InMemoryQueryBus::new();
    query_bus.register(FindConfigEntryQueryHandler::new(finder))
        .expect("Failed to register FindConfigEntryQueryHandler");

    web::Data::new(AppState {
        command_bus: Arc::new(command_bus),
        query_bus:   Arc::new(query_bus),
    })
}
```

Adding a new use case follows a predictable recipe. Once you have done it once, subsequent use cases are mostly mechanical -- create the structs, implement the traits, register the handler, wire the HTTP endpoint.

## Adding a new use case

1. Create a command or query struct in `libs/<context>/src/<context>/application/<use_case>/`.
2. Create a response struct with `error: Option<ConfigEntryErrorEntry>` (commands) or both data and error fields (queries).
3. Create a domain service that calls the repository and/or event bus. Finders return domain entities.
4. Create a handler that delegates to the service. The handler maps domain entities to DTOs and domain errors to `ConfigEntryErrorEntry`.
5. Register the handler in `build_state()` in the relevant app's `lib.rs`.
6. Add an HTTP handler that downcasts the bus response and checks `response.error`.
