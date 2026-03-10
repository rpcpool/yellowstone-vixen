# gRPC Idle Client

This example is a minimal raw Yellowstone gRPC client for reproducing stream idleness.

It subscribes to:

- `entries`
- `blockMeta`
- `slotStatus` with `interslot_updates = true`
- `transactions`

It does not run the Vixen runtime or the block coordinator. It only connects, subscribes, and
logs:

- the first update received for each subscribed event type
- the first update received for each slot status
- a running summary every `--log-every` updates
- idle gaps after `--idle-warn-secs`

It supports two modes:

- `continuous`: keep one subscription open and log idle/resume events
- `subscription-idle`: keep opening fresh subscriptions until one goes idle during a startup window, then stay on that stream

## Config

Use a standard Vixen config with a `[source]` section, for example:

```toml
[source]
endpoint = "https://index.rpcpool.com"
x-token = "<X-TOKEN>"
timeout = 60
```

Optional `commitment-level`, `from-slot`, `max-decoding-message-size`, and
`accept-compression` fields are also honored.

## Run

```bash
RUST_LOG=info cargo run -p yellowstone-vixen-example-grpc-idle-client -- --config ./Vixen.example.toml
```

Example with faster idle reporting:

```bash
RUST_LOG=info cargo run -p yellowstone-vixen-example-grpc-idle-client -- --config ./Vixen.example.toml --idle-warn-secs 10 --log-every 100
```

Subscription-idle mode:

```bash
RUST_LOG=info cargo run -p yellowstone-vixen-example-grpc-idle-client -- --config ./Vixen.example.toml --mode subscription-idle --idle-warn-secs 5 --subscription-idle-window-secs 10 --resubscribe-delay-ms 250
```

If `--subscription-idle-window-secs` is omitted, it defaults to `2 * --idle-warn-secs`.

Stop with `Ctrl-C`.
