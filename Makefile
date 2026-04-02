.DEFAULT_GOAL := build

.PHONY: build
build:
	cargo build --release -p config-api

.PHONY: test
test:
	cargo test

.PHONY: test/unit
test/unit:
	cargo test --workspace --exclude config-api

.PHONY: test/e2e
test/e2e: config_api/test/e2e

.PHONY: config_api/test/e2e
config_api/test/e2e:
	cargo test --test config-api-e2e -p config-api

.PHONY: test/summary
test/summary:
	cargo test 2>&1 | grep -Ev "^running 0 tests$$|^test result: ok\. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered|Running unittests src/lib\.rs|Doc-tests" | cat -s

.PHONY: dev/build
dev/build:
	cargo build

.PHONY: config_api/dev-build
config_api/dev-build:
	cargo build -p config-api

.PHONY: config_api/build
config_api/build:
	cargo build --release -p config-api

.PHONY: config_api/run
config_api/run:
	cargo run -p config-api

.PHONY: config_api/test
config_api/test:
	cargo test -p config-api

.PHONY: audit
audit:
	cargo install cargo-audit || echo "cargo-audit already installed"
	cargo audit

.PHONY: deps
deps:
	cargo update

.PHONY: format
format:
	cargo fmt

.PHONY: docker/up
docker/up:
	docker compose up -d

.PHONY: docker/down
docker/down:
	docker compose down

.PHONY: docker/logs
docker/logs:
	docker compose logs -f
