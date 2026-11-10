SHELL := /bin/bash
export PATH := $(HOME)/.cargo/bin:$(PATH)

.PHONY: build release install test lint fmt check smoke

build:
	cargo build --locked

release:
	cargo build --locked --release

install:
	cargo build --locked --release && mkdir -p ~/.local/bin && cp target/release/darkfactory-dispatch ~/.local/bin/

test:
	cargo test --locked

lint:
	cargo clippy --locked --all-targets -- -D warnings

fmt:
	cargo fmt --check

smoke:
	./tests/smoketest.sh

check: fmt lint test
