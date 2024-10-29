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

To list all available services, execute the following command

```bash
grpcurl -plaintext 127.0.0.1:3030 list
```

To introspect the program stream service, execute the following command

```bash
grpcurl -plaintext 127.0.0.1:3030 describe vixen.stream.ProgramStreams.Subscribe
```

To subcribe to the stream and receive parsed accounts and ixs, execute the following command

```bash
# Subscribing to Token extension program stream (replace this pubkey with the desired program pubkey that is supported by vixen)
grpcurl -plaintext -d '{"program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"}' 127.0.0.1:3030 vixen.stream.ProgramStreams/Subscribe
```

This example is using Token extensions program to parse Account updates.
replace this with other program pubkeys that are supported by vixen.
