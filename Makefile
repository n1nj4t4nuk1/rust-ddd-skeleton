.DEFAULT_GOAL := build

APPS := config_api

# Per-app targets — delegated to apps/<name>/Makefile
define APP_RULES
.PHONY: $(1)/build $(1)/dev-build $(1)/run $(1)/test $(1)/test/e2e
$(1)/build:
	$$(MAKE) -C apps/$(1) build
$(1)/dev-build:
	$$(MAKE) -C apps/$(1) dev-build
$(1)/run:
	$$(MAKE) -C apps/$(1) run
$(1)/test:
	$$(MAKE) -C apps/$(1) test
$(1)/test/e2e:
	$$(MAKE) -C apps/$(1) test/e2e
endef
$(foreach app,$(APPS),$(eval $(call APP_RULES,$(app))))

# Global targets

.PHONY: build
build:
	cargo build --release

.PHONY: dev/build
dev/build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: test/unit
test/unit:
	cargo test --workspace --exclude config-api

.PHONY: test/e2e
test/e2e: $(foreach app,$(APPS),$(app)/test/e2e)

.PHONY: test/summary
test/summary:
	cargo test 2>&1 | grep -Ev "^running 0 tests$$|^test result: ok\. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered|Running unittests src/lib\.rs|Doc-tests" | cat -s

.PHONY: format
format:
	cargo fmt

.PHONY: audit
audit:
	cargo install cargo-audit || echo "cargo-audit already installed"
	cargo audit

.PHONY: deps
deps:
	cargo update

.PHONY: docker/up
docker/up:
	docker compose up -d

.PHONY: docker/down
docker/down:
	docker compose down

.PHONY: docker/logs
docker/logs:
	docker compose logs -f
