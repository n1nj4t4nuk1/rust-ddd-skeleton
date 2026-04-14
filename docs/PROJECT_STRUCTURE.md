# Project Structure

Full file tree with the purpose of every directory and key file.

This document provides a complete map of the repository. It is useful for orientation when you first encounter the codebase, and as a reference when you need to find where something lives. The tree below is annotated with the purpose of every directory and key file.

The repository is organized into three main areas: `apps/` contains the HTTP services that users interact with, `libs/` contains the business logic organized into bounded contexts, and `tests/` contains external test suites that mirror the source structure. Shared infrastructure (CQRS buses, event bus, value objects) lives in `libs/shared/`.

```
.
├── Cargo.toml                  # Workspace root -- lists all member crates
├── Cargo.lock                  # Pinned dependency versions (commit this)
├── Makefile                    # Developer shortcuts -- delegates per-app targets
├── docker-compose.yml          # PostgreSQL (add more services as needed)
├── .env.example                # Environment variable template
├── .gitignore
│
├── apps/
│   └── config_api/             # Example REST API -- key/value config store
│       ├── Cargo.toml          # Package: config-api, declares [lib] + [[bin]] + [[test]]
│       ├── Makefile            # Per-app build/run/test targets
│       ├── Dockerfile          # Multi-stage build
│       └── src/
│           ├── main.rs         # Binary entry point: init tracing, build_state, HttpServer
│           ├── lib.rs          # Library root: AppState, build_state(), configure_routes()
│           ├── health/
│           │   ├── mod.rs
│           │   └── get.rs      # GET /health -> 200
│           └── config_entry/
│               ├── mod.rs
│               ├── post.rs     # POST /config -> 201 | 409
│               ├── get.rs      # GET  /config/{key} -> 200 | 404
│               ├── put.rs      # PUT  /config/{key} -> 200 | 404
│               ├── delete.rs   # DELETE /config/{key} -> 204 | 404
│               ├── create_config_entry_request.rs   # { key, value }
│               └── update_config_entry_request.rs   # { value }
│
├── libs/
│   ├── config/                 # ConfigEntry bounded context
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # pub mod config_entry;
│   │       └── config_entry/
│   │           ├── mod.rs      # pub mod application; domain; infrastructure;
│   │           │
│   │           ├── domain/
│   │           │   ├── mod.rs
│   │           │   ├── entities/
│   │           │   │   └── config_entry.rs         # ConfigEntry aggregate
│   │           │   ├── value_objects/
│   │           │   │   ├── config_key.rs            # ConfigKey(String)
│   │           │   │   └── config_value.rs          # ConfigValue(String)
│   │           │   ├── errors/
│   │           │   │   └── config_entry_repository_error.rs  # NotFound | AlreadyExists | Unexpected
│   │           │   ├── repositories/
│   │           │   │   └── config_entry_repository.rs        # async trait (save/find/update/delete)
│   │           │   └── events/
│   │           │       ├── config_entry_created_event.rs
│   │           │       ├── config_entry_updated_event.rs     # carries new_value + old_value
│   │           │       ├── config_entry_deleted_event.rs
│   │           │       ├── create_config_entry_created_event.rs   # factory fn from aggregate
│   │           │       ├── create_config_entry_updated_event.rs
│   │           │       └── create_config_entry_deleted_event.rs
│   │           │
│   │           ├── application/
│   │           │   ├── create_config_entry/
│   │           │   │   ├── create_config_entry_command.rs          # Command struct
│   │           │   │   ├── create_config_entry_response.rs         # { error: Option<ErrorEntry> }
│   │           │   │   ├── config_entry_creator.rs                 # Domain service
│   │           │   │   └── create_config_entry_command_handler.rs  # CommandHandler impl
│   │           │   ├── find_config_entry/
│   │           │   │   ├── find_config_entry_query.rs
│   │           │   │   ├── find_config_entry_response.rs           # { config_entry, error }
│   │           │   │   ├── config_entry_finder.rs
│   │           │   │   └── find_config_entry_query_handler.rs      # QueryHandler impl
│   │           │   ├── update_config_entry/
│   │           │   │   ├── update_config_entry_command.rs
│   │           │   │   ├── update_config_entry_response.rs
│   │           │   │   ├── config_entry_updater.rs
│   │           │   │   └── update_config_entry_command_handler.rs
│   │           │   └── delete_config_entry/
│   │           │       ├── delete_config_entry_command.rs
│   │           │       ├── delete_config_entry_response.rs
│   │           │       ├── config_entry_deleter.rs
│   │           │       └── delete_config_entry_command_handler.rs
│   │           │
│   │           └── infrastructure/
│   │               └── persistence/
│   │                   └── in_memory/
│   │                       └── in_memory_config_entry_repository.rs  # HashMap + Mutex
│   │
│   └── shared/
│       ├── cqrs/               # CQRS building blocks
│       │   ├── Cargo.toml      # dep: async-trait
│       │   └── src/
│       │       ├── lib.rs
│       │       ├── command/
│       │       │   ├── domain/
│       │       │   │   ├── command.rs              # AnyCommand + Command marker trait
│       │       │   │   ├── command_bus.rs          # async trait CommandBus
│       │       │   │   ├── command_bus_error.rs    # HandlerNotFound | HandlerAlreadyRegistered | HandlerError
│       │       │   │   └── command_handler.rs      # async trait CommandHandler<C>
│       │       │   └── infrastructure/in_memory/
│       │       │       └── in_memory_command_bus.rs  # HashMap<TypeId, HandlerFn>
│       │       └── query/
│       │           ├── domain/
│       │           │   ├── query.rs
│       │           │   ├── query_bus.rs            # async fn ask() -> Box<dyn Any>
│       │           │   ├── query_bus_error.rs
│       │           │   └── query_handler.rs        # type Response; async fn handle()
│       │           └── infrastructure/in_memory/
│       │               └── in_memory_query_bus.rs
│       │
│       ├── domain-events/      # Event bus building blocks
│       │   ├── Cargo.toml      # dep: uuid
│       │   └── src/
│       │       ├── lib.rs
│       │       ├── domain/
│       │       │   ├── domain_event.rs             # DomainEvent trait + DomainEventBase
│       │       │   ├── domain_event_subscriber.rs  # fn on(&self, event: &E) -> Result
│       │       │   ├── event_bus.rs                # fn publish(Vec<Box<dyn DomainEvent>>) -> Result
│       │       │   └── event_bus_error.rs          # DispatchError(String)
│       │       └── infrastructure/in_memory/
│       │           └── in_memory_event_bus.rs      # HashMap<TypeId, Vec<HandlerFn>>
│       │
│       └── valueobject/        # Reusable value object primitives
│           ├── Cargo.toml
│           └── src/
│               ├── domain/
│               │   ├── strings/string_value_object.rs   # StringValueObject(String)
│               │   └── errors/value_object_validation_error.rs
│               ├── application/mod.rs   # (reserved)
│               └── infrastructure/mod.rs  # (reserved)
│
├── tests/
│   ├── apps/
│   │   └── config_api/         # E2E tests for config_api (registered via [[test]] in Cargo.toml)
│   │       ├── tests.rs        # mod src;
│   │       └── src/
│   │           ├── mod.rs
│   │           ├── health/health_test.rs
│   │           └── config/
│   │               ├── create_config_entry_test.rs
│   │               ├── find_config_entry_test.rs
│   │               ├── update_config_entry_test.rs
│   │               └── delete_config_entry_test.rs
│   │
│   └── libs/
│       └── config/             # Unit tests for libs/config
│           ├── tests.rs
│           └── src/
│               ├── mocks/
│               │   ├── config_entry_repository_mock.rs  # Configurable test double
│               │   └── event_bus_mock.rs                # Records published events
│               └── config_entry/
│                   ├── domain/
│                   │   ├── entities/mothers/config_entry_mother.rs
│                   │   └── value_objects/mothers/
│                   │       ├── config_key_mother.rs
│                   │       └── config_value_mother.rs
│                   └── application/
│                       ├── create_config_entry/config_entry_creator_tests.rs
│                       ├── find_config_entry/config_entry_finder_tests.rs
│                       ├── update_config_entry/config_entry_updater_tests.rs
│                       └── delete_config_entry/config_entry_deleter_tests.rs
│
├── migrations/                 # SQL migration files (empty, add when using PostgreSQL)
│
└── docs/
    ├── ARCHITECTURE.md
    ├── PROJECT_STRUCTURE.md    # <-- this file
    ├── CQRS.md
    ├── DOMAIN_EVENTS.md
    ├── TESTING.md
    ├── ADDING_A_BOUNDED_CONTEXT.md
    ├── ADDING_AN_APP.md
    └── GIT_FLOW.md
```

---

## Use case directory structure

Within each bounded context, use cases follow a consistent directory layout. Understanding this layout is key to navigating the codebase -- once you know the pattern, you can predict exactly where to find any command handler, query response, or domain service.

Each use case (command or query) lives in its own directory under `application/`:

### Command use case (write)

```
create_config_entry/
├── create_config_entry_command.rs            # Command struct
├── create_config_entry_response.rs           # { error: Option<ConfigEntryErrorEntry> }
├── config_entry_creator.rs                   # Domain service -> Result<(), DomainError>
└── create_config_entry_command_handler.rs    # CommandHandler impl with type Response
```

### Query use case (read)

```
find_config_entry/
├── find_config_entry_query.rs                # Query struct
├── find_config_entry_response.rs             # { config_entry: Option<ConfigEntryEntry>, error: Option<ConfigEntryErrorEntry> }
├── config_entry_finder.rs                    # Domain service -> Result<ConfigEntry, DomainError>
└── find_config_entry_query_handler.rs        # QueryHandler impl -- maps entity to DTO
```

---

## Naming conventions

The naming conventions below are strictly followed throughout the codebase. They serve two purposes: making it easy to find any type by name (just convert the type name to snake_case and you have the file name), and making the role of each type immediately clear from its name.

| Concept | Rust name | Example |
|---|---|---|
| Aggregate | `PascalCase` struct | `ConfigEntry` |
| Value Object | `PascalCase` struct | `ConfigKey`, `ConfigValue` |
| Command | `VerbNounCommand` | `CreateConfigEntryCommand` |
| Query | `VerbNounQuery` | `FindConfigEntryQuery` |
| Domain Service | `NounVerber` | `ConfigEntryCreator`, `ConfigEntryFinder` |
| Command Handler | `VerbNounCommandHandler` | `CreateConfigEntryCommandHandler` |
| Query Handler | `VerbNounQueryHandler` | `FindConfigEntryQueryHandler` |
| Command Response | `VerbNounResponse` | `CreateConfigEntryResponse` |
| Query Response | `VerbNounResponse` | `FindConfigEntryResponse` |
| Data Entry DTO | `NounEntry` | `ConfigEntryEntry` |
| Error Entry DTO | `NounErrorEntry` | `ConfigEntryErrorEntry` |
| Domain Event | `NounPastEvent` | `ConfigEntryCreatedEvent` |
| Event factory fn | `create_noun_past_event` | `create_config_entry_created_event` |
| Domain Error | `NounRepositoryError` | `ConfigEntryRepositoryError` |
| Repository trait | `NounRepository` | `ConfigEntryRepository` |
| Repo impl -- in-memory | `InMemoryNounRepository` | `InMemoryConfigEntryRepository` |
| Repo impl -- PostgreSQL | `SqlxPostgresNounRepository` | `SqlxPostgresConfigEntryRepository` |

---

## File naming conventions

Files follow a one-type-per-file rule. The file name is always the snake_case version of the primary type it contains. This makes it trivial to locate any type -- if you are looking for `CreateConfigEntryCommandHandler`, it is in `create_config_entry_command_handler.rs`.

Files are named in `snake_case` and mirror the primary type they contain:

- `config_entry.rs` -> `struct ConfigEntry`
- `config_entry_repository.rs` -> `trait ConfigEntryRepository`
- `create_config_entry_command.rs` -> `struct CreateConfigEntryCommand`
- `create_config_entry_response.rs` -> `struct CreateConfigEntryResponse`
- `find_config_entry_response.rs` -> `struct FindConfigEntryResponse` + `struct ConfigEntryEntry` + `struct ConfigEntryErrorEntry`

Each file contains exactly one primary type (response files may contain the entry DTOs too). This makes grepping for a type trivial.
