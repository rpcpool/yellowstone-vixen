# Node.js client for Vixen grpc streams updates

As specified in [stream.proto](/crates/proto/proto/stream.proto) Vixen has a [Server streaming RPC](https://grpc.io/docs/what-is-grpc/core-concepts/#server-streaming-rpc) which accepts a request with the `program` Pubkey the user wants to subscribe to, in order to receive updates.

## Running `client.ts` example

`client.ts` contains an example with dynamic code generation for the grpc client.

First you need to run the Vixen grpc stream server, to do this navigate to `/examples/stream-parser` where an example server is built, and run:

```
RUST_LOG=info cargo run -- --config "$(pwd)/../../Vixen.toml"
```

> You will need the protoc compiler installed for running it (see the [protobuf repo for detailed explanation to install it](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation))

After the server is running you can execute this client to get the streamed results by executing `client.ts`:

```
npx ts-node ./client.ts
```

> \*The files in [`/proto`](/examples/ts-client/proto/) has been pre-generated using the [@grpc/proto-loader cli tool](https://github.com/grpc/grpc-node/tree/master/packages/proto-loader#generating-typescript-types) and [`compiled.json`](/examples/ts-client//compiled.json) was pre-generated using [protobufjs-cli](https://github.com/protobufjs/protobuf.js/tree/master/cli)

## Running `customStaticClientLib.ts` example

This example uses [our custom mini-library client generator](/examples/ts-client/client_lib/), which is based on static code generation for the classes and types for the grpc Services and Messages respectively

To run the example you can follow the same steps than the ones for the `client.ts` example, first run the grpc server and after that execute `customStaticClientLib.ts`
