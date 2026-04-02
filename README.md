# Rust DDD Template

A production-ready Rust workspace template for building microservices following Domain-Driven Design (DDD), CQRS, and Domain Events patterns.

## What is this?

This template gives you a working skeleton — shared infrastructure libraries plus a fully implemented example application (`config_api`) — so you can start a new service without spending time on architectural boilerplate.

Everything compiles, all tests pass, and the CI pipeline is already wired. Clone it, rename things, and start writing your domain logic.

## Stack

- **Rust** — systems language, async with Tokio
- **Actix-Web 4** — HTTP framework
- **CQRS** — commands and queries dispatched via in-memory bus
- **Domain Events** — synchronous event bus with subscriber registry
- **In-memory repositories** — ready to swap for PostgreSQL (sqlx), Redis, etc.
- **Docker** — multi-stage Dockerfile per app
- **GitHub Actions** — unit → e2e → build → docker pipeline

## Structure

```
.
├── apps/
│   └── config_api/         # Example REST API (key/value store)
├── libs/
│   ├── config/             # Example bounded context (domain + application + infra)
│   └── shared/
│       ├── cqrs/           # CommandBus, QueryBus, handlers
│       ├── domain-events/  # EventBus, DomainEvent, subscribers
│       └── valueobject/    # StringValueObject, ValidationError
├── tests/
│   ├── apps/config_api/    # E2E tests (HTTP → bus → repo)
│   └── libs/config/        # Unit tests (mocks, mothers, domain services)
├── docs/                   # Architecture documentation
├── migrations/             # SQL migrations (add as needed)
├── Makefile
├── docker-compose.yml
└── Cargo.toml              # Workspace root
```

## Quick start

**Prerequisites:** Rust stable, Docker (optional)

```bash
# Build everything
cargo build

# Run all tests
cargo test

# Unit tests only
make test/unit

# E2E tests only
make test/e2e

# Run the config API
make config_api/run
```

## Creating a new bounded context

1. Copy `libs/config/` as `libs/my_domain/`
2. Rename every occurrence of `config_entry` → `my_entity` and `ConfigEntry` → `MyEntity`
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

| Document | Description |
|---|---|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Overall architecture and design principles |
| [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) | Full file tree with explanations |
| [docs/CQRS.md](docs/CQRS.md) | How the command/query buses work |
| [docs/DOMAIN_EVENTS.md](docs/DOMAIN_EVENTS.md) | How domain events and subscribers work |
| [docs/ADDING_A_BOUNDED_CONTEXT.md](docs/ADDING_A_BOUNDED_CONTEXT.md) | Step-by-step: add a new lib |
| [docs/ADDING_AN_APP.md](docs/ADDING_AN_APP.md) | Step-by-step: add a new app |
| [docs/TESTING.md](docs/TESTING.md) | Testing strategy and conventions |

## Make targets

| Target | Description |
|---|---|
| `make test` | Run all tests |
| `make test/unit` | Unit tests only (no e2e) |
| `make test/e2e` | All e2e tests |
| `make config_api/test/e2e` | E2E tests for config_api |
| `make config_api/run` | Run config_api locally |
| `make config_api/build` | Release build |
| `make format` | Run `cargo fmt` |
| `make docker/up` | Start PostgreSQL via Docker Compose |
| `make docker/down` | Stop containers |

## License

MIT
