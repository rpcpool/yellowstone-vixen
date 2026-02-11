# Proc-macro integration tests

Integration tests for `yellowstone-vixen-proc-macro`. Each test module loads an IDL via `include_vixen_parser!` and exercises the generated code.

| Module | IDL | What it covers |
|---|---|---|
| `perpetuals` | `idls/perp_idl.json` | Parsing + schema snapshot |
| `pump_fun` | `idls/pump_fun.json` | Parsing + schema snapshot |

## Parser tests

Validates the generated parser against **real mainnet transactions**.

1. Generates a parser from the IDL.
2. Loads a transaction fixture (downloaded once from mainnet, then cached in `./fixtures/`).
3. Parses the instructions and asserts the output matches expected values.

## Schema snapshot tests

Snapshots the full `PROTOBUF_SCHEMA` string produced by the proc macro using [`cargo-insta`](https://insta.rs).

```sh
# Run tests (creates/updates pending snapshots)
cargo test -p proc-macro-test

# Review snapshot diffs
cargo insta review

# Install the CLI (first time only)
cargo install cargo-insta
```

Snapshot files live in `tests/snapshots/`.