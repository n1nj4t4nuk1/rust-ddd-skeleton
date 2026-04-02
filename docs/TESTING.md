# Testing

This template has two test suites: **unit tests** for the domain and application layers, and **end-to-end tests** for the HTTP API.

## Running tests

```bash
# All unit tests
make test/unit

# All e2e tests (no external services needed — uses in-memory repo)
make test/e2e

# Only config_api e2e tests
make config_api/test/e2e

# Everything at once
cargo test --workspace
```

## Unit tests

Unit tests live in `tests/libs/<context>/`. They test the application services (domain services + use cases) in isolation using mock collaborators.

```
tests/libs/config/
├── tests.rs              # Test binary root: mod src;
└── src/
    ├── mod.rs
    ├── mocks/
    │   ├── config_entry_repository_mock.rs
    │   └── event_bus_mock.rs
    └── config_entry/
        ├── domain/
        │   └── entities/mothers/config_entry_mother.rs
        │   └── value_objects/mothers/
        │       ├── config_key_mother.rs
        │       └── config_value_mother.rs
        └── application/
            ├── create_config_entry/config_entry_creator_tests.rs
            ├── find_config_entry/config_entry_finder_tests.rs
            ├── update_config_entry/config_entry_updater_tests.rs
            └── delete_config_entry/config_entry_deleter_tests.rs
```

The test binary is registered in `libs/config/Cargo.toml` as a `[[test]]` target so it can import `config` as an external crate — meaning it only accesses what `pub` exposes, just like real consumers would.

### Mocks

Mocks implement the same traits as production code but let tests control behaviour declaratively.

**`ConfigEntryRepositoryMock`** — configurable via behavior enums:

```rust
ConfigEntryRepositoryMock::new()                               // all ops succeed, find returns None
ConfigEntryRepositoryMock::that_returns_entry(entry)           // find returns the given entry
ConfigEntryRepositoryMock::that_finds_nothing()                // find returns NotFound
ConfigEntryRepositoryMock::that_returns_entry_but_update_fails(entry)

// Assertions
repo.save_call_count()
repo.find_by_key_call_count()
repo.update_call_count()
repo.delete_call_count()
```

**`EventBusMock`** — records every published event:

```rust
EventBusMock::new()
EventBusMock::that_fails()          // publish returns Err

// Assertions
bus.published_event_names()         // Vec<&'static str>
bus.publish_call_count()
```

### Object Mothers

Mothers are factories that produce valid, random domain objects. They remove boilerplate from tests and make intent clear:

```rust
let entry = ConfigEntryMother::random();
let key   = ConfigKeyMother::random();
let value = ConfigValueMother::with_value("hello");
```

Mothers live next to the types they build: `tests/libs/config/src/config_entry/domain/`.

### What unit tests cover

Each use case has four tests:

| Test | What it asserts |
|---|---|
| `it_calls_*_on_the_repository` | The correct repo method is called exactly once |
| `it_publishes_a_*_event` | The event bus receives the right event name |
| `it_returns_not_found_when_entry_does_not_exist` | The service propagates `NotFound` |
| `it_does_not_publish_event_when_*_fails` | No events published on failure |

## End-to-end tests

E2E tests live in `tests/apps/<api>/`. They start the real Actix-Web service (in-memory, no external dependencies) and send HTTP requests through it.

```
tests/apps/config_api/
├── tests.rs
└── src/
    ├── mod.rs
    ├── health/health_test.rs
    └── config/
        ├── create_config_entry_test.rs
        ├── find_config_entry_test.rs
        ├── update_config_entry_test.rs
        └── delete_config_entry_test.rs
```

### Test setup

Every test bootstraps the full application stack with two lines:

```rust
let state = build_state();     // fresh in-memory repo per test
let app = test::init_service(
    App::new().app_data(state).configure(configure_routes)
).await;
```

Because `build_state()` creates a new `InMemoryConfigEntryRepository` on every call, tests are fully isolated — no teardown required.

### What e2e tests cover

| Endpoint | Cases tested |
|---|---|
| `GET /health` | 200 OK |
| `POST /config` | 201 Created, 409 Conflict on duplicate key |
| `GET /config/{key}` | 200 with body, 404 on missing key |
| `PUT /config/{key}` | 200 updated, 404 on missing key |
| `DELETE /config/{key}` | 204 No Content, 404 on missing key |

## CI pipeline

Tests run in GitHub Actions on every push:

```
unit-tests (stable / beta / nightly)
    └─▶ e2e-tests
            └─▶ dev-build
                    └─▶ build (release)
                            └─▶ docker-build
```

Unit tests run in a matrix across three Rust toolchain versions to catch compatibility regressions early.

## Adding tests for a new bounded context

1. Create `tests/libs/<context>/tests.rs` and `tests/libs/<context>/src/`.
2. Add the `[[test]]` entry to `libs/<context>/Cargo.toml`:
   ```toml
   [[test]]
   name = "<context>"
   path = "../../tests/libs/<context>/tests.rs"
   ```
3. Create mocks and mothers mirroring the existing patterns.
4. Write one test file per use case under `tests/libs/<context>/src/<context>/application/`.

## Adding e2e tests for a new app

1. Create `tests/apps/<api>/tests.rs` and `tests/apps/<api>/src/`.
2. Add the `[[test]]` entry to `apps/<api>/Cargo.toml`:
   ```toml
   [[test]]
   name = "<api>-e2e"
   path = "../../tests/apps/<api>/tests.rs"
   ```
3. Import `build_state` and `configure_routes` from the app's library crate.
4. Write one test file per resource group.
