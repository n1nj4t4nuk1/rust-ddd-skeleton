# Rust DDD Template

A production-ready Rust workspace template for building microservices following Domain-Driven Design (DDD), CQRS, and Domain Events patterns.

## What is this?

This template gives you a working skeleton -- shared infrastructure libraries plus a fully implemented example application (`config_api`) -- so you can start a new service without spending time on architectural boilerplate.

The project follows Domain-Driven Design with Hexagonal Architecture (Ports and Adapters). In practice, this means the core business logic has zero knowledge of databases, HTTP frameworks, or external services. All coupling flows inward through repository traits and bus abstractions, so you can swap an in-memory store for PostgreSQL (or Actix-Web for another framework) without touching a single line of domain code. This separation makes the codebase easier to test in isolation, reason about under pressure, and extend without fear of breaking unrelated parts.

Everything compiles, all tests pass, and the CI pipeline is already wired. Clone it, rename things, and start writing your domain logic.

## Stack

| Component | Technology |
|---|---|
| Language | Rust 2021 |
| HTTP framework | Actix-Web 4 |
| Persistence | In-memory (ready to swap for PostgreSQL, Redis, etc.) |
| Logging | tracing + tracing-subscriber |
| Architecture | DDD + Hexagonal (Ports & Adapters) |

## Architecture overview

The project is organized as a Cargo workspace with two main directories: `apps/` contains HTTP services built with Actix-Web, and `libs/` contains the business logic organized into bounded contexts. Each bounded context is a self-contained Rust crate with its own domain model, application services, and infrastructure adapters. The shared infrastructure (CQRS buses, event bus, value objects) lives in `libs/shared/` and is used by all contexts.

### Dependency rule

Domain -> Application -> Infrastructure. Domain never imports infrastructure; coupling is via repository traits only.

### Bounded context layout

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
      <noun>_<verb>er.rs              # Domain service
      <verb>_<noun>_command.rs        # Command struct (writes)
      <verb>_<noun>_query.rs          # Query struct (reads)
      <verb>_<noun>_response.rs       # Response envelope: { data?, error? }
      <verb>_<noun>_command_handler.rs / _query_handler.rs
  infrastructure/
    persistence/
      in_memory/      # HashMap-based implementations
```

## Project structure

```
rust-ddd-skeleton/
├── apps/
│   └── config_api/          # Example REST API — key/value config store (port 8080)
│
├── libs/
│   ├── config/              # Example bounded context (config_entry CRUD)
│   │   └── src/config_entry/
│   │       ├── domain/      # entities, value_objects, repositories, events, errors
│   │       ├── application/ # create, find, update, delete use cases
│   │       └── infrastructure/persistence/in_memory/
│   └── shared/
│       ├── cqrs/            # CommandBus + QueryBus (TypeId-based dispatch)
│       ├── domain-events/   # EventBus + DomainEventSubscriber
│       └── valueobject/     # StringValueObject, ValidationError, typed primitives
│
├── tests/
│   ├── apps/config_api/     # E2E tests (HTTP -> bus -> repo)
│   └── libs/config/         # Unit tests (mocks, mothers, domain services)
│
├── docs/                    # Architecture documentation
├── docker-compose.yml
├── Makefile                 # Root Makefile (delegates to per-app Makefiles)
└── Cargo.toml               # Workspace root
```

## Quick start

**Prerequisites:** Rust stable (2021 edition), Docker (optional)

### Build

```bash
make build               # Release build (all apps)
make dev/build            # Dev build
make config_api/build     # Release build for config_api only
```

### Test

```bash
make test                 # All tests (unit + e2e)
make test/unit            # Unit tests only
make test/e2e             # All e2e suites
make config_api/test/e2e  # E2E tests for config_api
```

Tests do not require a running database -- the e2e suites use the in-memory repository by default.

### Run

```bash
make config_api/run       # Start config_api on port 8080
```

Log level defaults to `info`. Override with `RUST_LOG`:

```bash
RUST_LOG=debug make config_api/run
```

## Creating a new bounded context

1. Copy `libs/config/` as `libs/my_domain/`
2. Rename every occurrence of `config_entry` -> `my_entity` and `ConfigEntry` -> `MyEntity`
3. Add `libs/my_domain` to `[workspace] members` in the root `Cargo.toml`
4. Add the lib as a dependency in your app's `Cargo.toml`

See [docs/ADDING_A_BOUNDED_CONTEXT.md](docs/ADDING_A_BOUNDED_CONTEXT.md) for a step-by-step guide.

## Creating a new app

1. Copy `apps/config_api/` as `apps/my_api/`
2. Update the `Cargo.toml` package name and dependencies
3. Add `apps/my_api` to the workspace
4. Add Makefile targets and CI jobs

See [docs/ADDING_AN_APP.md](docs/ADDING_AN_APP.md) for details.

## Documentation

The `docs/` directory contains detailed guides for understanding and extending the system. Start with **ARCHITECTURE.md** for the big picture, then explore specific topics as needed.

| Document | Description |
|---|---|
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design, layer diagram, key patterns, and conventions |
| [PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) | Full annotated file tree with the purpose of every directory |
| [CQRS.md](docs/CQRS.md) | How commands and queries flow through the system |
| [DOMAIN_EVENTS.md](docs/DOMAIN_EVENTS.md) | Event-driven communication between bounded contexts |
| [TESTING.md](docs/TESTING.md) | Test strategy, mocks, Object Mother pattern, CI pipeline |
| [ADDING_A_BOUNDED_CONTEXT.md](docs/ADDING_A_BOUNDED_CONTEXT.md) | Step-by-step guide to adding a new domain module |
| [ADDING_AN_APP.md](docs/ADDING_AN_APP.md) | Step-by-step guide to adding a new HTTP service |

## Make targets

| Target | Description |
|---|---|
| `make build` | Release build (all apps) |
| `make dev/build` | Dev build |
| `make config_api/build` | Release build for config_api |
| `make test` | Run all tests |
| `make test/unit` | Unit tests only |
| `make test/e2e` | All e2e tests |
| `make config_api/test/e2e` | E2E tests for config_api |
| `make config_api/run` | Run config_api locally |
| `make format` | Run `cargo fmt` |
| `make audit` | Security audit via cargo-audit |
| `make docker/up` | Start containers via Docker Compose |
| `make docker/down` | Stop containers |

## License

MIT
