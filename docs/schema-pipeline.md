# Schema pipeline

The v2 JSON Schema defines the public contract shared by the Rust library and
the MCP server.

## Source of truth

[`schema/rustdoc-query.v2.schema.json`](../schema/rustdoc-query.v2.schema.json)
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
| `schema/generated/rustdoc-query.v2.rs` | Public Rust request, outcome, result, and error types. |
| `schema/generated/rustdoc-query.v2.defaults.rs` | Service defaults derived from the schema. |
| `schema/generated/rustdoc-query.v2.strict_option.rs` | Validation for presence-sensitive optional fields. |
| `src/mcp/generated.rs` | MCP tool bindings and schema projection metadata. |

## Documentation

The canonical schema is also the source of public documentation. Every named
definition, object property, and `oneOf` branch must have a consumer-facing
`description`.

Use singleton `const` branches inside `oneOf` for documented string enum
values. This preserves the accepted JSON values while allowing the generator
to emit rustdoc for each generated Rust enum variant. The generator also
propagates documented union branches that Typify does not emit on its own.

`xtask` verifies this documentation-completeness rule and generated variant
docs. Do not hand-edit generated rustdoc attributes.

## Changing the contract

1. Edit `schema/rustdoc-query.v2.schema.json`.
2. Regenerate artifacts.
3. Verify the generated output and workspace checks.

```sh
just schema-generate
just schema
just check
just lint
just test
just rustdoc
just ci
```

Commit the canonical schema and every generated artifact changed by the
generator. `just schema` verifies that generated artifacts are current.
`just rustdoc` builds the library documentation with warnings denied, and
`just ci` runs that check with formatting, schema, compile, lint, test, and
build verification.

## Compatibility

The v2 schema is the compatibility boundary for library consumers and MCP
clients. Treat removed or renamed fields, changed meanings, changed defaults,
and tighter constraints as breaking changes. Additive optional fields are the
normal compatible extension point.

Breaking contract changes require a new schema version rather than modifying
v2 in place.

## Version taxonomy

The public contract is version 2. Its canonical schema and generated Rust
types are part of the library and MCP compatibility surface.

The cache sidecar envelope, FTS sidecar, and opaque find cursor are private
application formats, each at version 2. A cache-sidecar mismatch is rejected,
an FTS mismatch rebuilds the index, and a cursor mismatch is rejected as stale.
They are not public contract versions.

The upstream rustdoc JSON `format_version` is independent of these application
format versions. Cache sidecars retain it separately so a rustdoc-format change
does not reset or redefine the application-owned format version.

## Scope

The contract describes queries over crate public APIs. It does not expose cache
implementation details, transport internals, private Rust items, or raw rustdoc
data structures.
