# Static grpc client generator for Vixen streams

This small package is intended to facilitate the grpc clients construction for consuming Vixen streams. It has dependencies on [@grpc/grpc-js](https://github.com/grpc/grpc-node/tree/master/packages/grpc-js) for the actual network service implementation, and on [protobufjs-cli](https://github.com/protobufjs/protobuf.js/tree/master/cli) for the pre-generated static classes and types for the grpc Messages.

## Updating static generated code

For updating/re-generating the static classes one could run:

Generate updated proto files using grpcurl reflection against the Vixen streams server (replace `127.0.0.1:3030` with the appropiate server url)

```bash
grpcurl -proto-out-dir "./protos" -plaintext 127.0.0.1:3030 describe vixen.stream.ProgramStreams.Subscribe
```

```bash
# Initial JS classes generation (with JSDoc comments, needed for the types generation next step)
npx pbjs -t static-module -w commonjs -o compiled.js "$(pwd)/proto/stream.proto" "$(pwd)/proto/parser.proto"

# TS types generation out of compiled JS on previous step
npx pbts --no-comments -o compiled.d.ts compiled.js

# Override JS with `--no-comments` for optimized file size
npx pbjs -t static-module --no-comments -w commonjs -o compiled.js "$(pwd)/proto/stream.proto" "$(pwd)/proto/parser.proto"
```

> \*After the proto files are used for the `pbjs` code generation, they are not needed any more.
