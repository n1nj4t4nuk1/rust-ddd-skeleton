# Architecture

## Overview

This template is built around three ideas that work well together in Rust microservices:

- **Domain-Driven Design (DDD)** — model your business domain explicitly; keep domain logic isolated from I/O
- **CQRS** — separate write paths (commands) from read paths (queries) so each can evolve independently
- **Domain Events** — propagate side-effects through a decoupled event bus instead of direct calls

The result is a layered architecture where each layer has a single responsibility and depends only on layers beneath it.

```
┌─────────────────────────────────────────────────────┐
│                   HTTP (actix-web)                  │  ← apps/
│        Controllers: deserialise, dispatch, respond  │
├──────────────────────┬──────────────────────────────┤
│    Command Bus       │       Query Bus               │  ← shared/cqrs
│  (write operations)  │   (read operations)           │
├──────────────────────┴──────────────────────────────┤
│              Application Services                   │  ← libs/*/application/
│  Creator · Finder · Updater · Deleter               │
│  Orchestrates: repo + event bus                     │
├─────────────────────────────────────────────────────┤
│                   Domain                            │  ← libs/*/domain/
│  Aggregates · Value Objects · Events · Repo traits  │
├─────────────────────────────────────────────────────┤
│              Infrastructure                         │  ← libs/*/infrastructure/
│  InMemory / PostgreSQL / Redis repositories         │
└─────────────────────────────────────────────────────┘
```

## Layers

### HTTP layer (`apps/`)

Actix-Web handlers are thin adapters:

1. Deserialise the HTTP request into a typed command or query struct
2. Dispatch it through the bus
3. Map the result to an HTTP response (status code + optional JSON body)

Handlers know nothing about the domain internals. All business logic lives in the application layer.

### Command Bus (`libs/shared/cqrs`)

Commands represent intent to change state. A command is a plain struct; the bus routes it to the one handler registered for that type.

```
POST /config  →  CreateConfigEntryCommand  →  CreateConfigEntryCommandHandler
                                              └─ ConfigEntryCreator.execute()
                                                  ├─ repo.save()
                                                  └─ event_bus.publish()
```

### Query Bus (`libs/shared/cqrs`)

Queries retrieve data without side effects. The handler returns a typed response that the bus boxes and the caller downcasts.

```
GET /config/{key}  →  FindConfigEntryQuery  →  FindConfigEntryQueryHandler
                                               └─ ConfigEntryFinder.execute()
                                                   └─ repo.find_by_key()
```

### Application Services (`libs/*/application/`)

One service per use case (creator, finder, updater, deleter). Each service:

- Accepts typed value objects, not raw strings
- Calls the repository trait (not a concrete type)
- Publishes domain events after a successful state change
- Returns a typed result or a domain error

Services are pure business logic — no HTTP, no SQL, no framework.

### Domain (`libs/*/domain/`)

The domain layer defines the rules of your business:

- **Aggregates** — the root entity, owner of invariants (e.g. `ConfigEntry`)
- **Value Objects** — immutable wrappers that carry meaning (e.g. `ConfigKey`, `ConfigValue`)
- **Repository traits** — async interfaces declared here, implemented in infrastructure
- **Domain Events** — facts that happened (e.g. `ConfigEntryCreatedEvent`)
- **Errors** — domain-specific error types (e.g. `ConfigEntryRepositoryError`)

Nothing in the domain imports from actix-web, sqlx, or any framework.

### Infrastructure (`libs/*/infrastructure/`)

Concrete repository implementations. The template ships with an in-memory HashMap-backed repo behind a `Mutex`. Swap it for a sqlx implementation without touching any other layer.

## Dependency direction

```
apps/config_api
    └── libs/config         (domain + application + infrastructure)
    └── libs/shared/cqrs
    └── libs/shared/domain-events

libs/config
    └── libs/shared/cqrs
    └── libs/shared/domain-events
```

Dependencies flow inward. The domain never imports application code; application never imports HTTP code.

## Bounded Contexts

Each `libs/` directory (excluding `shared/`) represents one bounded context: a cohesive area of the domain with its own language, models, and persistence. Contexts communicate via domain events, not direct function calls.

```
libs/
├── config/      ← ConfigEntry bounded context
└── shared/      ← Cross-cutting infrastructure (no domain logic)
```

When two bounded contexts need to react to each other's events, the subscriber lives in the _consuming_ context, not the _producing_ one.

## Wiring

All wiring — repositories, services, buses — is done once in `apps/*/src/lib.rs` inside `build_state()`. This function:

1. Creates a concrete repository (e.g. `InMemoryConfigEntryRepository`)
2. Creates an event bus (`InMemoryEventBus`) and registers any subscribers
3. Instantiates domain services, wraps them in command/query handlers
4. Registers handlers in the buses
5. Returns `web::Data<AppState>` shared across all Actix-Web workers

Because `build_state()` takes no arguments (for in-memory backends), calling it once per test gives each test a fully isolated state — no shared mutable state, no database teardown.
