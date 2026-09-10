# Repository command surface. Run `just --list` to discover supported tasks.

workspace := "--workspace"
workspace_features := workspace + " --all-features"
workspace_targets := workspace_features + " --all-targets"
xtask := "cargo run -p xtask --"

default: build

build:
    cargo build {{ workspace_targets }}

check:
    cargo check {{ workspace_targets }}

test:
    cargo nextest run {{ workspace_features }}

lint:
    cargo clippy {{ workspace_targets }} -- -D warnings

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

schema:
    {{ xtask }} check-schema

schema-generate:
    {{ xtask }} generate-schema

pre-commit: fmt schema check lint

pre-push: fmt-check schema check lint test

ci: fmt-check schema check lint test build

release:
    #!/usr/bin/env bash
    set -euo pipefail
    version=$(git cliff --bumped-version | sed 's/^v//')
    cargo release "$version" --execute

ship: ci release
    git push origin HEAD --follow-tags

hooks-install:
    lefthook install

mcp:
    cargo build
    mcp-inspector ./target/debug/rustdoc-query
