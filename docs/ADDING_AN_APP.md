# Adding an App

An app is an Actix-Web binary that wires together one or more bounded contexts and exposes them over HTTP. Apps live in `apps/<name>/`.

An app is the outermost layer of the architecture -- it is what actually runs as a process and listens for HTTP requests. Apps are thin: they wire together bounded contexts with their infrastructure, register handlers on the CQRS buses, and define HTTP routes. The business logic lives entirely in the bounded context libraries.

Each app follows the same structure: a `lib.rs` that exposes `build_state()` (dependency wiring) and `configure_routes()` (HTTP routing), a `main.rs` that starts the server, and a set of HTTP handlers organized by resource. This consistency means that once you understand one app, you understand them all.

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

## 2. AppState and dependency wiring -- lib.rs

The `lib.rs` file is the most important file in an app -- it is where all dependencies are assembled. The `build_state()` function creates repositories, builds domain services, creates handlers, and registers them on the buses. By keeping all wiring in one function, you get a clear picture of what the app depends on and how its components connect.

The library exposes `build_state()` and `configure_routes()` so that both `main.rs` and e2e tests can use the same wiring. Each call to `build_state()` produces an isolated in-memory store, making it safe to call once per test.

**`apps/user_api/src/lib.rs`**

```rust
use std::sync::Arc;

use actix_web::web;
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_cqrs::query::domain::query_bus::QueryBus;
use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;

use user::user::application::create_user::config_entry_creator::UserCreator;
use user::user::application::create_user::create_user_command_handler::CreateUserCommandHandler;
use user::user::application::find_user::find_user_query_handler::FindUserQueryHandler;
use user::user::application::find_user::user_finder::UserFinder;
use user::user::domain::repositories::user_repository::UserRepository;
use user::user::infrastructure::persistence::in_memory::in_memory_user_repository::InMemoryUserRepository;

pub mod health;
pub mod user;     // HTTP handlers

/// Shared application state injected into every Actix-Web request handler.
pub struct AppState {
    pub command_bus: Arc<dyn CommandBus>,
    pub query_bus:   Arc<dyn QueryBus>,
}

/// Wires all repositories, services and buses together and returns the shared
/// application state.
pub fn build_state() -> web::Data<AppState> {
    let repo: Arc<dyn UserRepository> = Arc::new(InMemoryUserRepository::new());
    let event_bus: Arc<dyn EventBus> = Arc::new(InMemoryEventBus::new());

    let creator = UserCreator::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let create_handler = CreateUserCommandHandler::new(creator);

    let finder = UserFinder::new(Arc::clone(&repo));
    let find_handler = FindUserQueryHandler::new(finder);

    let mut command_bus = InMemoryCommandBus::new();
    command_bus
        .register(create_handler)
        .expect("Failed to register CreateUserCommandHandler");

    let mut query_bus = InMemoryQueryBus::new();
    query_bus
        .register(find_handler)
        .expect("Failed to register FindUserQueryHandler");

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

> **Note:** If the HTTP handler module name clashes with the lib crate name (e.g. both called `user`), rename the HTTP module to `user_entry` or suffix it -- the same issue was encountered with the `config` context in this template.

---

## 3. Binary entry point -- main.rs

**`apps/user_api/src/main.rs`**

```rust
use actix_web::{HttpServer, App};
use user_api::{build_state, configure_routes};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

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

HTTP handlers are the bridge between the HTTP world and the CQRS world. Each handler deserializes the request, constructs a command or query, dispatches it through the appropriate bus, downcasts the response to the concrete envelope type, and translates the `concept` field into an HTTP status code. Handlers are intentionally simple -- all business logic lives in the bounded context.

Create one file per HTTP verb under `apps/user_api/src/user/` (or your chosen module name).

**`apps/user_api/src/user/post.rs`** -- command handler with response envelope:

```rust
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use tracing::{debug, info, warn};

use user::user::application::create_user::create_user_command::CreateUserCommand;
use user::user::application::create_user::create_user_response::CreateUserResponse;
use user::user::domain::value_objects::user_id::UserId;
use user::user::domain::value_objects::user_name::UserName;

use crate::AppState;

use super::create_user_request::CreateUserRequest;

#[derive(Deserialize)]
pub struct CreateUserRequest { pub id: String, pub name: String }

#[post("/users")]
pub async fn handler(
    state: web::Data<AppState>,
    body:  web::Json<CreateUserRequest>,
) -> impl Responder {
    debug!(id = %body.id, "POST /users");

    let command = CreateUserCommand {
        id: UserId::new(body.id.clone()),
        name: UserName::new(body.name.clone()),
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateUserResponse>()
                .expect("Unexpected response type from CreateUserCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %body.id, "User already exists");
                        HttpResponse::Conflict().body(error.message.clone())
                    }
                    "NotFound" => HttpResponse::NotFound().body(error.message.clone()),
                    _ => {
                        warn!(id = %body.id, error = %error.message, "Failed to create user");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %body.id, "User created");
                HttpResponse::Created().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
```

**`apps/user_api/src/user/get.rs`** -- query handler with response envelope:

```rust
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};

use user::user::application::find_user::find_user_query::FindUserQuery;
use user::user::application::find_user::find_user_response::FindUserResponse;
use user::user::domain::value_objects::user_id::UserId;

use crate::AppState;

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: String,
    pub name: String,
}

#[get("/users/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path:  web::Path<String>,
) -> impl Responder {
    let raw_id = path.into_inner();
    debug!(id = %raw_id, "GET /users/:id");

    let query = FindUserQuery { id: UserId::new(raw_id.clone()) };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindUserResponse>()
                .expect("Unexpected response type from FindUserQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %raw_id, "User not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %raw_id, error = %error.message, "Failed to find user");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.user {
                info!(id = %raw_id, "User found");
                HttpResponse::Ok().json(GetUserResponse {
                    id: entry.id.clone(),
                    name: entry.name.clone(),
                })
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %raw_id, error = %e, "Failed to find user");
            HttpResponse::InternalServerError().body(e.to_string())
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

E2E tests exercise the full stack -- from HTTP request to domain logic and back -- using in-memory repositories. They use the same `build_state()` and `configure_routes()` functions as the production app, so they test real wiring without needing a database or any external service.

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

## 8. Per-app Makefile

Each app has its own Makefile with standard targets. The root Makefile delegates to these per-app Makefiles, so you can run `make user_api/build` from the workspace root and it will execute the `build` target in `apps/user_api/Makefile`.

Create **`apps/user_api/Makefile`**:

```makefile
PACKAGE = user-api
E2E_TEST = user-api-e2e

.PHONY: build dev-build run test test/e2e

build:
	cargo build --release -p $(PACKAGE)

dev-build:
	cargo build -p $(PACKAGE)

run:
	cargo run -p $(PACKAGE)

test:
	cargo test -p $(PACKAGE)

test/e2e:
	cargo test --test $(E2E_TEST) -p $(PACKAGE)
```

Then register the app in the root `Makefile`'s `APPS` list:

```makefile
APPS := config_api user_api
```

The root Makefile's `define APP_RULES` macro will auto-generate `user_api/build`, `user_api/run`, `user_api/test/e2e`, etc. from the per-app Makefile.

---

## 9. Checklist

- [ ] `apps/user_api/Cargo.toml` created and added to workspace `members`
- [ ] `src/lib.rs` with `AppState`, `build_state()`, `configure_routes()`
- [ ] `src/main.rs` binary entry point
- [ ] `src/health/get.rs` health handler
- [ ] HTTP handlers using downcast + envelope pattern for each endpoint
- [ ] `Dockerfile` in `apps/user_api/`
- [ ] `tests/apps/user_api/` e2e test crate wired via `[[test]]`
- [ ] `apps/user_api/Makefile` created
- [ ] App name added to `APPS` list in root `Makefile`
- [ ] `cargo test -p user-api` and `cargo test --test user-api-e2e` pass
