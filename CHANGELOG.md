# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [0.1.0] - 2026-09-10

### Added
- **rustdoc:** Load and cache versioned rustdoc artifacts: Query crate documentation from docs.rs and cache immutable resolved versions in memory and on disk. Build a public API surface index while loading each crate.
- **overview:** Add crate overview queries: Summarize a crate or public module with documentation, headline items, and kind counts. Return canonical public paths and resolved crate versions.
- **find:** Add public item search and pagination: Search public APIs by name, kind, module, or documentation with stable ranked results. Return opaque cursors for complete retrieval across result pages.
- **detail:** Add public item detail queries: Expose signatures, fields, variants, methods, and implementation context for public items. Distinguish concrete inherent implementations and filter blanket implementations by default.
- **search:** Add documentation search: Search public API documentation through a persistent full-text index. Bound retrieval work to preserve predictable latency on large crates.
- **contract:** Add a schema-derived public library contract: Define requests, outcomes, defaults, and errors in one generated contract. Enforce schema constraints at deserialization and keep generated artifacts current in CI.
- **service:** Expose generated rustdoc query operations: Provide overview, search, and item-detail operations through generated request and outcome types. Apply documented defaults and map private failures to typed API errors.
- **mcp:** Add generated MCP registration and response formats: Register schema-derived tools and support JSON or TOON responses. Keep transport framing separate from service execution and serialization.

