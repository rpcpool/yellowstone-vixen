FROM rust:latest AS builder

WORKDIR /usr/src/yellowstone-vixen

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/yellowstone-vixen/crates/demo/target/release/demo /usr/local/bin/demo

# Define the command to run the binary
CMD ["demo"]