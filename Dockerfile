FROM rust:1.78.0-buster as builder

WORKDIR /usr/src/yellowstone-vixen

COPY . .

WORKDIR /usr/src/yellowstone-vixen/crates/demo

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/yellowstone-vixen/crates/demo/target/release/demo .

CMD ["./demo"]
