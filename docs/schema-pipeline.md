# Schema pipeline

The v1 JSON Schema defines the public contract shared by the Rust library and
the MCP server.

## Source of truth

[`schema/rustdoc-query.v1.schema.json`](../schema/rustdoc-query.v1.schema.json)
is the canonical contract. It defines requests, outcomes, errors, defaults,
constraints, and MCP tool metadata.

```text
canonical JSON Schema
  -> xtask generator
  -> generated Rust contract types and support code
  -> public library surface
       -> RustdocQueryService
       -> generated request and outcome types
       -> JSON or TOON encoding
  -> MCP adapter
       -> generated tool registration schema
       -> RustdocQueryService
       -> JSON-RPC tool-result framing
  -> library and MCP consumers
```

The library exposes generated types through
`rustdoc_query::contract::generated`. `RustdocQueryService` accepts generated
requests and returns generated outcomes. Library consumers can retain typed
outcomes or encode them as JSON or TOON.

`RustdocMcpServer` projects the same contract into MCP tool schemas, invokes the
service, and frames its outcomes as JSON-RPC tool results. These are two
interfaces to one contract, not separate API definitions.

## Generated artifacts

Do not hand-edit generated files.

| File | Purpose |
| --- | --- |
| `schema/generated/rustdoc-query.v1.rs` | Public Rust request, outcome, result, and error types. |
| `schema/generated/rustdoc-query.v1.defaults.rs` | Service defaults derived from the schema. |
| `schema/generated/rustdoc-query.v1.strict_option.rs` | Validation for presence-sensitive optional fields. |
| `src/mcp/generated.rs` | MCP tool bindings and schema projection metadata. |

## Changing the contract

1. Edit `schema/rustdoc-query.v1.schema.json`.
2. Regenerate artifacts.
3. Verify the generated output and workspace checks.

```sh
just schema-generate
just schema
just ci
```

Commit the canonical schema and every generated artifact changed by the
generator.

## Compatibility

The v1 schema is the compatibility boundary for library consumers and MCP
clients. Treat removed or renamed fields, changed meanings, changed defaults,
and tighter constraints as breaking changes. Additive optional fields are the
normal compatible extension point.

Breaking contract changes require a new schema version rather than modifying
v1 in place.

## Scope

The contract describes queries over crate public APIs. It does not expose cache
implementation details, transport internals, private Rust items, or raw rustdoc
data structures.
