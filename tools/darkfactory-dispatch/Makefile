SHELL := /bin/bash
export PATH := $(HOME)/.cargo/bin:$(PATH)

.PHONY: build release install test lint fmt check smoke

build:
	cargo build

release:
	cargo build --release

install:
	cargo build --release && mkdir -p ~/.local/bin && cp target/release/darkfactory-dispatch ~/.local/bin/

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --check

smoke:
	./tests/smoketest.sh

check: fmt lint test
