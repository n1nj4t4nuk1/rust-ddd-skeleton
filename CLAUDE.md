# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is this?

A Rust workspace template for building microservices following DDD with Hexagonal Architecture (Ports & Adapters). Business logic is fully decoupled from infrastructure. Ships with a working example app (`config_api`) and shared infrastructure libraries (CQRS, domain events, value objects).

## Common Commands

```bash
# Build
make build                    # Release build (all apps)
make dev/build                # Dev build
make config_api/build         # Release build for config_api

# Test
make test                     # All tests (unit + e2e)
make test/unit                # Unit tests only
make test/e2e                 # All e2e suites
make config_api/test/e2e      # E2E tests for config_api
cargo test -p <crate> <test>  # Single test

# Run
make config_api/run           # Start config_api on port 8080
RUST_LOG=debug make config_api/run

# Other
make format                   # cargo fmt
make audit                    # Security audit
make docker/up                # Start containers
make docker/down              # Stop containers
```

## Architecture

### Layered Structure

```
apps/           -> HTTP services (Actix-Web), one per bounded context
  config_api/     (port 8080)

libs/           -> Bounded contexts + shared infrastructure
  config/         config_entry (CRUD key/value store)
  shared/
    cqrs/           CommandBus + QueryBus (TypeId-based dispatch)
    domain-events/  EventBus + DomainEventSubscriber
    valueobject/    Typed validated primitives

tests/          -> External test suites (mirrors libs/ and apps/)
```

### Dependency Rule

Domain -> Application -> Infrastructure. Domain never imports infrastructure; coupling is via repository traits only.

### Each Bounded Context Follows This Layout

```
<context>/
  domain/
    entities/         # Aggregate roots
    value_objects/    # Typed wrappers with validation
    repositories/     # Trait definitions only
    events/           # Domain events + factory functions
    errors/           # Error enums (NotFound, AlreadyExists, Unexpected)
  application/
    <verb>_<noun>/    # One folder per use case
      <noun>_<verb>er.rs              # Domain service (returns entities or Result<(), Error>)
      <verb>_<noun>_command.rs        # Command struct (writes)
      <verb>_<noun>_query.rs          # Query struct (reads)
      <verb>_<noun>_response.rs       # Response envelope: { data?, error? }
      <verb>_<noun>_command_handler.rs / _query_handler.rs
  infrastructure/
    persistence/
      in_memory/    # HashMap-based implementations
```

### Naming Conventions

| Concept | Pattern | Example |
|---------|---------|---------|
| Domain Service | `NounVerber` | `ConfigEntryCreator`, `ConfigEntryFinder` |
| Command | `VerbNounCommand` | `CreateConfigEntryCommand` |
| Command Handler | `VerbNounCommandHandler` | `CreateConfigEntryCommandHandler` |
| Command Response | `VerbNounResponse` | `CreateConfigEntryResponse` |
| Query | `VerbNounQuery` | `FindConfigEntryQuery` |
| Query Handler | `VerbNounQueryHandler` | `FindConfigEntryQueryHandler` |
| Query Response | `VerbNounResponse` | `FindConfigEntryResponse` |
| Data Entry DTO | `NounEntry` | `ConfigEntryEntry` |
| Error Entry DTO | `NounErrorEntry` | `ConfigEntryErrorEntry` |
| Domain Error | `NounError` | `ConfigEntryRepositoryError` |
| Domain Event | `NounPastEvent` | `ConfigEntryCreatedEvent` |
| Event Factory | `create_noun_past_event` | `create_config_entry_created_event` |
| Event Subscriber | `OnNounPastReaction` | `OnConfigEntryCreatedDoSomething` |
| Repository Trait | `NounRepository` | `ConfigEntryRepository` |
| In-Memory Impl | `InMemoryNounRepository` | `InMemoryConfigEntryRepository` |

### Key Patterns

- **CQRS**: Commands and Queries dispatched via TypeId-based buses. Both return `Result<Box<dyn Any>, BusError>` -- callers downcast to the concrete response type.
- **Response Envelope**: All handlers return a response struct with `error: Option<ErrorEntry>`. Query responses also carry an optional data field. Domain errors map to `ErrorEntry { message, concept }` where concept is PascalCase: `"NotFound"`, `"AlreadyExists"`, `"Unexpected"`.
- **Domain Services**: Finders return domain entities; command services return `Result<(), DomainError>`. The handler does the DTO mapping and error envelope construction.
- **Domain Events**: Cross-context communication through events only. Cascade deletes and counter updates handled via async background tasks.
- **Repository Pattern**: Traits in domain, implementations in infrastructure. In-memory impls available for all repos (used in tests and as default persistence).
- **Value Objects**: Every domain primitive is a typed wrapper that validates at construction.
- **Dependency Injection**: All wiring happens in each app's `build_state()` function in `lib.rs`. A `build_state_in_memory()` variant exists for e2e tests.
- **Per-App Makefiles**: Each app has its own `Makefile` in `apps/<name>/Makefile`. The root `Makefile` delegates via `$(MAKE) -C`.

### Testing

- **Unit tests** (`tests/libs/`): Test domain services in isolation using mock repos, mock event buses, and Object Mother pattern for test data.
- **E2E tests** (`tests/apps/`): Real Actix-Web service with in-memory repositories. HTTP requests via `actix_web::test`. No database needed.

### Tech Stack

Rust 2021, Actix-Web 4, Tokio, tracing, thiserror, serde, uuid v4. No database required -- all persistence is in-memory by default.

### Detailed Documentation

See `docs/` for in-depth guides: `ARCHITECTURE.md`, `CQRS.md`, `DOMAIN_EVENTS.md`, `TESTING.md`, `ADDING_A_BOUNDED_CONTEXT.md`, `ADDING_AN_APP.md`, `PROJECT_STRUCTURE.md`.
