# Stream Parser Example

This example demonstrates how vixen-streams (but this applies to any Vixen parser service) can be integrated with OpenTelemetry
 and tracing using the parsers "tracing" feature flag

## Running the example

To run the example, navigate to [`examples/streams-tracing`](/examples/streams-tracing/) and execute the following command:

**Note:** Additionally to the example code you need to setup your OTEL infrastructure

### Server

```bash
RUST_LOG=info cargo run -- --config "$(pwd)/../../Vixen.toml"
```
