FROM rust:1.78.0-buster as builder

WORKDIR /usr/src/yellowstone-vixen

COPY . .

WORKDIR /usr/src/yellowstone-vixen/crates/demo

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/yellowstone-vixen/crates/demo/target/release/demo .

EXPOSE 3030

CMD ["./demo"]
