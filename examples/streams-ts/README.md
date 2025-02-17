# Node.js client for Vixen grpc streams updates

As specified in [stream.proto](/crates/proto/proto/stream.proto) Vixen has a [Server streaming RPC](https://grpc.io/docs/what-is-grpc/core-concepts/#server-streaming-rpc) which accepts a request with the `program` Pubkey the user wants to subscribe to, in order to receive updates.

## Running `client.ts` example

`client.ts` contains an example with dynamic code generation for the grpc client.

First you need to run the Vixen grpc stream server, to do this navigate to `/examples/stream-parser` where an example server is built, and run:

```
RUST_LOG=info cargo run -- --config "$(pwd)/../../Vixen.toml"
```

> 💡 You will need the protoc compiler installed for running it (see the [protobuf repo for detailed explanation to install it](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation))

Then in order to generate the compiled files and types out of the proto files provided for the stream server reflection service you can run:

```bash
# Install dependencies
npm install

# Build the proto files and compiled json package definition out of them.
make build
```

> 💡 `compiled.json` was generated using [protobufjs-cli](https://github.com/protobufjs/protobuf.js/tree/master/cli)

After the server is running and the compiled files are generated you can execute this client to get the streamed results by executing `client.ts`:

```bash
npm start
```
