# Proc-macro integration tests

Integration tests for `yellowstone-vixen-proc-macro`. Each test module loads an IDL via `include_vixen_parser!` and exercises the generated code.

| Module | IDL | What it covers |
|---|---|---|
| `perpetuals` | `idls/perp_idl.json` | Instruction parsing + schema snapshot |
| `pump_fun` | `idls/pump_fun.json` | Instruction parsing + schema snapshot |
| `dynamic_bonding_curve` | `idls/dynamic_bonding_curve.json` | Account parsing + schema snapshot |
| `order_engine` | `idls/order_engine.json` | IDL with no accounts + schema snapshot |

## Parser tests

Validates the generated parser against **real mainnet transactions**.

1. Generates a parser from the IDL.
2. Loads a transaction fixture (downloaded once from mainnet, then cached in `./fixtures/`).
3. Parses the instructions and asserts the output matches expected values.

## Schema snapshot tests

Snapshots the full `PROTOBUF_SCHEMA` string produced by the proc macro using [`cargo-insta`](https://insta.rs), and validates it with `protoc`.

```sh
# Run tests (creates/updates pending snapshots)
cargo test -p proc-macro-test

# Review snapshot diffs
cargo insta review

# Install the CLI (first time only)
cargo install cargo-insta
```

Snapshot files live in `tests/snapshots/`.

## Protoc validation

Each `check_protobuf_schema` test also runs `protoc` to verify the generated `.proto` schema is syntactically valid. This is skipped automatically when `protoc` is not installed, or can be disabled explicitly:

```sh
SKIP_PROTOC=1 cargo test -p proc-macro-test
```