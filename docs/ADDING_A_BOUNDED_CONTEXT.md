# Adding a Bounded Context

A bounded context is a self-contained domain model with its own ubiquitous language. In this template each bounded context lives in `libs/<context>/`.

This guide walks through adding a `user` bounded context with a `User` aggregate.

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

### Aggregate

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

### Value objects

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

### Repository error

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

### Repository trait

**`libs/user/src/user/domain/repositories/user_repository.rs`**

```rust
use async_trait::async_trait;
use crate::user::domain::entities::user::User;
use crate::user::domain::value_objects::user_id::UserId;
use crate::user::domain::errors::user_repository_error::UserRepositoryError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<(), UserRepositoryError>;
    async fn find_by_id(&self, id: &UserId) -> Result<User, UserRepositoryError>;
}
```

### Domain events

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

### Module wiring

Wire modules in `mod.rs` files at each level:

```
libs/user/src/lib.rs                  → pub mod user;
libs/user/src/user/mod.rs             → pub mod domain; pub mod application; pub mod infrastructure;
libs/user/src/user/domain/mod.rs      → pub mod entities; pub mod value_objects; pub mod errors; pub mod repositories; pub mod events;
```

---

## 3. Application layer

For each use case (e.g. `create_user`), create a directory under `libs/user/src/user/application/create_user/` containing:

| File | Purpose |
|---|---|
| `create_user_command.rs` | `struct CreateUserCommand { id: String, name: String }` |
| `create_user_response.rs` | `struct CreateUserResponse` (often `()`) |
| `user_creator.rs` | Domain service — calls repo + event bus |
| `create_user_command_handler.rs` | `CommandHandler<CreateUserCommand>` impl |

**Domain service example:**

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
        self.repository.save(user.clone()).await?;

        let event = create_user_created_event(&user);
        self.event_bus.publish(vec![Box::new(event)]).await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
```

---

## 4. Infrastructure layer

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
    async fn save(&self, user: User) -> Result<(), UserRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let key = user.id().value().to_string();
        if store.contains_key(&key) {
            return Err(UserRepositoryError::AlreadyExists);
        }
        store.insert(key, user);
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<User, UserRepositoryError> {
        self.store.lock().unwrap()
            .get(id.value())
            .cloned()
            .ok_or(UserRepositoryError::NotFound)
    }
}
```

---

## 5. Unit tests

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

Follow the patterns in `tests/libs/config/` — configurable mocks, object mothers, one test file per use case with four tests each.

---

## 6. Checklist

- [ ] `libs/user/Cargo.toml` created and added to workspace `members`
- [ ] Domain entities, value objects, errors, repository trait, events defined
- [ ] Application services and command/query handlers defined
- [ ] `InMemoryUserRepository` created
- [ ] All `mod.rs` files wired up
- [ ] `tests/libs/user/` test crate created with mocks, mothers, and tests
- [ ] `[[test]]` entry in `libs/user/Cargo.toml` pointing to `tests/libs/user/tests.rs`
- [ ] `cargo test -p user` passes
