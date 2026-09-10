# rustdoc-query

Make AI slightly less stupid about Rust APIs.

`rustdoc-query` queries crate public APIs from docs.rs rustdoc JSON. Use it as
an embeddable Rust library or an MCP server.

## Public surface

The supported Rust API is:

- `rustdoc_query::contract::generated::*` for versioned request and outcome types
- `rustdoc_query::service::RustdocQueryService` for crate queries
- `rustdoc_query::mcp::RustdocMcpServer` for an embeddable MCP adapter

Other modules are implementation details.

The service operates on public API only. It provides crate and module overviews,
public item detail, name search, and documentation search. The generated v2
contract is the source of truth for request fields, defaults, and outcomes.

## Output formats

JSON is the default representation. TOON is available for compact text output.
The request and outcome types are unchanged.

Library users can call `RustdocQueryService::json_value` or
`RustdocQueryService::toon`. MCP users can set
`RUSTDOC_QUERY_OUTPUT_FORMAT=toon` or pass `--output-format toon`.

## Library

Add the package as a dependency:

```toml
[dependencies]
rustdoc-query = "0.1"
```

Construct `RustdocQueryService`, then call its async `overview`, `find`, or
`get_item` methods with the corresponding generated request type. Results are
the matching generated outcome type and include typed API errors.

API documentation is available at [docs.rs](https://docs.rs/rustdoc-query).

## MCP server

Install from crates.io:

```sh
cargo install rustdoc-query
```

Or install a release build:

```sh
curl -LsSf https://releases.gnu.foo/rustdoc-query/latest/install.sh | sh
```

The server uses stdio JSON-RPC. A client configuration needs this command:

```json
{
  "command": "rustdoc-query"
}
```

It registers these tools:

- `crate_overview`
- `find_items`, including documentation search
- `get_item`

Logs are written to standard error. Set `RUSTDOC_QUERY_CACHE_DIR` to override
the local rustdoc cache directory.

## Build from source

```sh
git clone https://github.com/gnufood/rustdoc-query.git
cd rustdoc-query
mise install --locked
just build
./target/debug/rustdoc-query
```

## Development

```sh
just hooks-install
```

| Command | Purpose |
| --- | --- |
| `just build` | Build the workspace. |
| `just check` | Type-check all workspace targets. |
| `just test` | Run the test suite with nextest. |
| `just lint` | Run Clippy. |
| `just schema` | Verify generated schema artifacts. |
| `just ci` | Run the full local CI sequence. |

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.
