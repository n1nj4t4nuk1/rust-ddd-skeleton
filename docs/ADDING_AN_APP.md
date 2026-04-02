# Adding an App

An app is an Actix-Web binary that wires together one or more bounded contexts and exposes them over HTTP. Apps live in `apps/<name>/`.

This guide adds a `user_api` app that exposes the `user` bounded context (see [ADDING_A_BOUNDED_CONTEXT.md](ADDING_A_BOUNDED_CONTEXT.md)).

---

## 1. Create the app crate

```bash
mkdir -p apps/user_api/src
```

**`apps/user_api/Cargo.toml`**

```toml
[package]
name = "user-api"
version = "0.1.0"
edition = "2021"

[lib]
name = "user_api"
path = "src/lib.rs"

[[bin]]
name = "user-api"
path = "src/main.rs"

[[test]]
name = "user-api-e2e"
path = "../../tests/apps/user_api/tests.rs"

[dependencies]
actix-web            = "4"
user                 = { path = "../../libs/user" }
shared-cqrs          = { path = "../../libs/shared/cqrs" }
shared-domain-events = { path = "../../libs/shared/domain-events" }
serde                = { version = "1", features = ["derive"] }
tokio                = { version = "1", features = ["full"] }
tracing              = "0.1"
tracing-subscriber   = { version = "0.3", features = ["env-filter"] }
```

Register it in the workspace root `Cargo.toml`:

```toml
[workspace]
members = [
    ...
    "apps/user_api",
]
```

---

## 2. Library root — lib.rs

The library exposes `build_state()` and `configure_routes()` so that both `main.rs` and e2e tests can use the same wiring.

**`apps/user_api/src/lib.rs`**

```rust
use actix_web::web;
use std::sync::Arc;

use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;

use user::user::application::create_user::user_creator::UserCreator;
use user::user::application::create_user::create_user_command::CreateUserCommand;
use user::user::application::create_user::create_user_command_handler::CreateUserCommandHandler;
use user::user::application::find_user::find_user_query::FindUserQuery;
use user::user::application::find_user::find_user_query_handler::FindUserQueryHandler;
use user::user::application::find_user::user_finder::UserFinder;
use user::user::infrastructure::persistence::in_memory::in_memory_user_repository::InMemoryUserRepository;

pub mod health;
pub mod user;     // HTTP handlers — rename if it clashes with the lib crate name

pub struct AppState {
    pub command_bus: Arc<dyn shared_cqrs::command::domain::command_bus::CommandBus>,
    pub query_bus:   Arc<dyn shared_cqrs::query::domain::query_bus::QueryBus>,
}

pub fn build_state() -> web::Data<AppState> {
    let repo      = Arc::new(InMemoryUserRepository::new());
    let event_bus = Arc::new(InMemoryEventBus::new());

    let creator = Arc::new(UserCreator::new(repo.clone(), event_bus.clone()));
    let finder  = Arc::new(UserFinder::new(repo.clone()));

    let mut command_bus = InMemoryCommandBus::new();
    command_bus.register::<CreateUserCommand, _>(
        Arc::new(CreateUserCommandHandler::new(creator))
    );

    let mut query_bus = InMemoryQueryBus::new();
    query_bus.register::<FindUserQuery, _>(
        Arc::new(FindUserQueryHandler::new(finder))
    );

    web::Data::new(AppState {
        command_bus: Arc::new(command_bus),
        query_bus:   Arc::new(query_bus),
    })
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::get::handler)
       .service(user::post::handler)
       .service(user::get::handler);
}
```

> **Note:** If the HTTP handler module name clashes with the lib crate name (e.g. both called `user`), rename the HTTP module to `user_entry` or suffix it — the same issue was encountered with the `config` context in this template.

---

## 3. Binary entry point — main.rs

**`apps/user_api/src/main.rs`**

```rust
use actix_web::{HttpServer, App};
use user_api::{build_state, configure_routes};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let state = build_state();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

---

## 4. Health handler

**`apps/user_api/src/health/get.rs`**

```rust
use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().finish()
}
```

---

## 5. HTTP handlers

Create one file per HTTP verb under `apps/user_api/src/user/` (or your chosen module name).

**`apps/user_api/src/user/post.rs`**

```rust
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use user::user::application::create_user::create_user_command::CreateUserCommand;
use shared_cqrs::command::domain::command_bus::CommandBus;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateUserRequest { pub id: String, pub name: String }

#[post("/users")]
pub async fn handler(
    state: web::Data<AppState>,
    body:  web::Json<CreateUserRequest>,
) -> impl Responder {
    let cmd = CreateUserCommand { id: body.id.clone(), name: body.name.clone() };
    match state.command_bus.dispatch(Box::new(cmd)).await {
        Ok(_)  => HttpResponse::Created().finish(),
        Err(e) => {
            if e.to_string().contains("already exists") {
                HttpResponse::Conflict().finish()
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
```

---

## 6. Dockerfile

**`apps/user_api/Dockerfile`**

```dockerfile
FROM rust:1.88-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p user-api

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/user-api /usr/local/bin/user-api
EXPOSE 8080
CMD ["user-api"]
```

---

## 7. End-to-end tests

```bash
mkdir -p tests/apps/user_api/src/user
```

**`tests/apps/user_api/tests.rs`**

```rust
mod src;
```

**`tests/apps/user_api/src/mod.rs`**

```rust
mod health;
mod user;
```

**`tests/apps/user_api/src/health/health_test.rs`**

```rust
use actix_web::{test, App};
use user_api::{build_state, configure_routes};

#[actix_web::test]
async fn health_returns_200() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes)
    ).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let res = test::call_service(&app, req).await;
    assert_eq!(res.status(), 200);
}
```

---

## 8. Makefile targets

Add targets to the root `Makefile`:

```makefile
user_api/run:
	cargo run -p user-api

user_api/test/e2e:
	cargo test --test user-api-e2e
```

---

## 9. Checklist

- [ ] `apps/user_api/Cargo.toml` created and added to workspace `members`
- [ ] `src/lib.rs` with `build_state()` and `configure_routes()`
- [ ] `src/main.rs` binary entry point
- [ ] `src/health/get.rs` health handler
- [ ] HTTP handlers for each endpoint
- [ ] `Dockerfile` in `apps/user_api/`
- [ ] `tests/apps/user_api/` e2e test crate
- [ ] `[[test]]` entry in `apps/user_api/Cargo.toml`
- [ ] Makefile targets added
- [ ] `cargo test -p user-api` and `cargo test --test user-api-e2e` pass
