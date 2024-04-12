.DEFAULT_GOAL := all

.PHONY: build-prod
build-prod:
	cargo build --release

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy --tests --

.PHONY: test
test:
	cargo test --all

.PHONY: server
server:
	cargo run --bin server
