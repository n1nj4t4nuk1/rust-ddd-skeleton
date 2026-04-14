# Adding a Bounded Context

A bounded context is a self-contained domain model with its own ubiquitous language. In this template each bounded context lives in `libs/<context>/`.

This guide walks through adding a bounded context from scratch. A bounded context in this project is a Cargo library crate that contains a complete domain model: entities, value objects, repository traits, domain events, application services, and infrastructure implementations.

The process is methodical -- you build from the inside out, starting with the domain layer (the core types and rules), then the application layer (the use cases that orchestrate domain logic), and finally the infrastructure layer (the concrete implementations of repository traits). This order ensures that your domain model is clean and self-contained before you add any external dependencies.

The example below creates a `user` bounded context with a `User` aggregate. Replace `user` / `User` with your actual context name throughout. The patterns shown here match exactly what you will find in the existing `config` context.

---

## 1. Create the library crate

```bash
mkdir -p libs/user/src
```

**`libs/user/Cargo.toml`**

```toml
[package]
name = "user"
version = "0.1.0"
edition = "2021"

[[test]]
name = "user"
path = "../../tests/libs/user/tests.rs"

[dependencies]
shared-cqrs         = { path = "../shared/cqrs" }
shared-domain-events = { path = "../shared/domain-events" }
async-trait = "0.1"
thiserror   = "2"
tokio       = { version = "1", features = ["full"] }
tracing     = "0.1"
```

Register it in the workspace root `Cargo.toml`:

```toml
[workspace]
members = [
    ...
    "libs/user",
]
```

---

## 2. Domain layer

The domain layer is the heart of the bounded context. It defines the types, rules, and contracts that the rest of the system depends on. Nothing in this layer imports external frameworks or infrastructure -- it is pure Rust types and traits. Everything else in the bounded context depends on this layer, but this layer depends on nothing else.

### 2a. Value objects

Value objects are typed wrappers around primitives that enforce invariants at construction time. Once created, a value object is guaranteed to be valid. This eliminates an entire class of bugs -- you can never accidentally pass an empty string where a name is expected, or confuse a user ID with a config key. Every field in an aggregate should be a value object, not a raw primitive.

**`libs/user/src/user/domain/value_objects/user_id.rs`**

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(v: String) -> Self { Self(v) }
    pub fn value(&self) -> &str   { &self.0 }
}
```

Create `user_name.rs` following the same pattern.

### 2b. Aggregate

**`libs/user/src/user/domain/entities/user.rs`**

```rust
use crate::user::domain::value_objects::user_id::UserId;
use crate::user::domain::value_objects::user_name::UserName;

#[derive(Clone)]
pub struct User {
    id:   UserId,
    name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self { Self { id, name } }
    pub fn id(&self)   -> &UserId   { &self.id }
    pub fn name(&self) -> &UserName { &self.name }
}
```

### 2c. Domain error

**`libs/user/src/user/domain/errors/user_repository_error.rs`**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("user not found")]
    NotFound,
    #[error("user already exists")]
    AlreadyExists,
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
```

### 2d. Repository trait

**`libs/user/src/user/domain/repositories/user_repository.rs`**

```rust
use async_trait::async_trait;
use crate::user::domain::entities::user::User;
use crate::user::domain::value_objects::user_id::UserId;
use crate::user::domain::errors::user_repository_error::UserRepositoryError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError>;
}
```

### 2e. Domain events

**`libs/user/src/user/domain/events/user_created_event.rs`**

```rust
use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

pub struct UserCreatedEvent {
    base: DomainEventBase,
    pub id:   String,
    pub name: String,
}

impl UserCreatedEvent {
    pub const EVENT_NAME: &'static str = "user.created";
}

impl DomainEvent for UserCreatedEvent {
    fn base(&self) -> &DomainEventBase { &self.base }
    fn name(&self) -> &'static str     { Self::EVENT_NAME }
}
```

Create the corresponding factory function in `create_user_created_event.rs`.

### 2f. Module wiring

Wire modules in `mod.rs` files at each level:

```
libs/user/src/lib.rs                  -> pub mod user;
libs/user/src/user/mod.rs             -> pub mod domain; pub mod application; pub mod infrastructure;
libs/user/src/user/domain/mod.rs      -> pub mod entities; pub mod value_objects; pub mod errors; pub mod repositories; pub mod events;
```

---

## 3. Application layer

The application layer contains use cases -- the things the system actually does. Each use case is a small, focused service that coordinates between the domain (entities, repositories) and the infrastructure (event bus). Use cases are organized one-per-directory, with each directory containing the service, the command/query struct, the response type, and the handler.

For each use case (e.g. `create_user`), create a directory under `libs/user/src/user/application/create_user/` containing:

### Command use case directory

| File | Purpose |
|---|---|
| `create_user_command.rs` | Command struct with value-object fields |
| `create_user_response.rs` | Response with `error: Option<UserErrorEntry>` |
| `user_creator.rs` | Domain service (returns `Result<(), UserRepositoryError>`) |
| `create_user_command_handler.rs` | `CommandHandler` impl with `type Response` |

### Query use case directory

| File | Purpose |
|---|---|
| `find_user_query.rs` | Query struct |
| `find_user_response.rs` | Response with `user: Option<UserEntry>` and `error: Option<UserErrorEntry>` |
| `user_finder.rs` | Domain service (returns `Result<User, UserRepositoryError>`) |
| `find_user_query_handler.rs` | `QueryHandler` impl -- maps entity to DTO |

### Domain service example -- command (UserCreator)

Domain services for commands return `Result<(), DomainError>`. They contain the business logic -- calling the repository, building the event, publishing it -- but they know nothing about DTOs or response envelopes.

```rust
pub struct UserCreator {
    repository: Arc<dyn UserRepository>,
    event_bus:  Arc<dyn EventBus>,
}

impl UserCreator {
    pub fn new(repository: Arc<dyn UserRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(&self, id: UserId, name: UserName) -> Result<(), UserRepositoryError> {
        let user = User::new(id, name);
        self.repository.save(&user).await?;

        let event = create_user_created_event(&user);
        self.event_bus.publish(vec![Box::new(event)]).await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
```

### Domain service example -- query (UserFinder)

Finders return the domain entity directly. The handler is responsible for mapping it to a response DTO.

```rust
pub struct UserFinder {
    repository: Arc<dyn UserRepository>,
}

impl UserFinder {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: UserId) -> Result<User, UserRepositoryError> {
        let user = self.repository.find_by_id(&id).await?;
        user.ok_or(UserRepositoryError::NotFound)
    }
}
```

### Response types

**Command response** (`create_user_response.rs`) -- only has an error field:

```rust
use crate::user::application::find_user::find_user_response::UserErrorEntry;

pub struct CreateUserResponse {
    pub error: Option<UserErrorEntry>,
}
```

**Query response** (`find_user_response.rs`) -- has data + error:

```rust
pub struct UserEntry {
    pub id: String,
    pub name: String,
}

pub struct UserErrorEntry {
    pub message: String,
    pub concept: String,  // "NotFound", "AlreadyExists", "Unexpected"
}

pub struct FindUserResponse {
    pub user: Option<UserEntry>,
    pub error: Option<UserErrorEntry>,
}
```

### Command handler

The handler calls the domain service and maps the result into the response envelope. Domain errors become `ErrorEntry` values with a `concept` field that the HTTP layer uses for status code selection.

```rust
#[async_trait]
impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    type Response = CreateUserResponse;

    async fn handle(&self, command: CreateUserCommand) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.id, command.name).await {
            Ok(()) => Ok(CreateUserResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    UserRepositoryError::NotFound => "NotFound",
                    UserRepositoryError::AlreadyExists => "AlreadyExists",
                    UserRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateUserResponse {
                    error: Some(UserErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
```

### Query handler

Maps domain entity to response DTO:

```rust
#[async_trait]
impl QueryHandler<FindUserQuery> for FindUserQueryHandler {
    type Response = FindUserResponse;

    async fn handle(&self, query: FindUserQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(user) => Ok(FindUserResponse {
                user: Some(UserEntry {
                    id: user.id().value().to_string(),
                    name: user.name().value().to_string(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    UserRepositoryError::NotFound => "NotFound",
                    UserRepositoryError::AlreadyExists => "AlreadyExists",
                    UserRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindUserResponse {
                    user: None,
                    error: Some(UserErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
```

---

## 4. Infrastructure layer

The infrastructure layer provides concrete implementations of the repository traits defined in the domain layer. You will typically create at least an in-memory implementation (used for testing) and optionally a PostgreSQL implementation (for production). The key insight is that neither the domain nor the application layer knows which implementation is being used -- they depend only on the trait.

### In-memory implementation

**`libs/user/src/user/infrastructure/persistence/in_memory/in_memory_user_repository.rs`**

```rust
use std::collections::HashMap;
use std::sync::Mutex;
use async_trait::async_trait;

pub struct InMemoryUserRepository {
    store: Mutex<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self { Self { store: Mutex::new(HashMap::new()) } }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let key = user.id().value().to_string();
        if store.contains_key(&key) {
            return Err(UserRepositoryError::AlreadyExists);
        }
        store.insert(key, user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        Ok(self.store.lock().unwrap().get(id.value()).cloned())
    }
}
```

---

## 5. Unit tests

Once the code compiles, you will want to add tests. The project follows a strict testing pattern: each use case gets a standard set of tests that verify the happy path, error handling, event publishing, and the absence of side effects on failure. See the `config_entry` tests for the canonical example.

Create the test crate under `tests/libs/user/`:

```
tests/libs/user/
├── tests.rs
└── src/
    ├── mod.rs
    ├── mocks/
    │   ├── user_repository_mock.rs
    │   └── event_bus_mock.rs
    └── user/
        ├── domain/
        │   └── entities/mothers/user_mother.rs
        │   └── value_objects/mothers/
        │       ├── user_id_mother.rs
        │       └── user_name_mother.rs
        └── application/
            └── create_user/user_creator_tests.rs
```

Follow the patterns in `tests/libs/config/` -- configurable mocks, object mothers, one test file per use case with tests that cover:

1. **Happy path** -- the operation succeeds and produces the expected side effect (e.g., entry saved).
2. **Event publishing** -- a domain event is published after success.
3. **Return value** -- the service returns `Ok(())` or the expected entity.
4. **Error propagation** -- the service returns the correct error variant when the repository fails.
5. **No side effects on failure** -- no events are published when the operation fails.

---

## 6. Checklist

- [ ] `libs/user/Cargo.toml` created and added to workspace `members`
- [ ] Domain entities, value objects, errors, repository trait, events defined
- [ ] Application services and command/query handlers with response envelopes defined
- [ ] `InMemoryUserRepository` created
- [ ] All `mod.rs` files wired up
- [ ] `tests/libs/user/` test crate created with mocks, mothers, and tests
- [ ] `[[test]]` entry in `libs/user/Cargo.toml` pointing to `tests/libs/user/tests.rs`
- [ ] `cargo test -p user` passes
