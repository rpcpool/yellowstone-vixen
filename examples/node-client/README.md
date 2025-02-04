# Node client for Vixen grpc streams updates

As specified in [stream.proto](/crates/proto/proto/stream.proto) Vixen has a [Server streaming RPC](https://grpc.io/docs/what-is-grpc/core-concepts/#server-streaming-rpc) which accepts a request with the `program` Pubkey the user wants to subscribe to, in order to receive updates.

## Running this example

First you need to run the Vixen grpc stream server, to do this navigate to `/examples/stream-parser` where an example server is built, and run:

```
RUST_LOG=info cargo run -- --config "$(pwd)/../../Vixen.toml"
```

> You will need the protoc compiler installed for running it (see the [protobuf repo for detailed explanation to install it](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation))

After the server is running you can execute this client to get the streamed results by executing:

```
node ./dynamic_codegen_client.js
```
