# Stream Parser Example

This example demonstrates how vixen-streams works,
It spins up a gRPC server which serves a stream of parsed accounts and transaction updates

## Running the example

To run the example, navigate to [`examples/stream-parser`](/examples/stream-parser/) and execute the following command:

### Server

```bash
RUST_LOG=info cargo run -- --config "$(pwd)/../../Vixen.toml"
```

### Client

To subcribe to the stream, navigate to [`crates/proto/proto`](/crates/proto/proto/) and run

```bash
grpcurl -plaintext -import-path ./ -proto stream.proto -proto parser.proto -proto solana.proto -d '{"program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"}' 127.0.0.1:3030 vixen.stream.ProgramStreams/Subscribe
```

This example is using Token extensions program to parse Account updates (ixs coming soon).
replace TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA in request data to parse Token Program
