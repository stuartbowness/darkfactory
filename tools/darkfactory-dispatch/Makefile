.PHONY: build release install test lint fmt check

build:
	cargo build

release:
	cargo build --release

install:
	cargo build --release && cp target/release/darkfactory-dispatch ~/.local/bin/

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --check

check: fmt lint test
