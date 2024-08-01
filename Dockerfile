FROM rust:latest AS builder

WORKDIR /usr/src/yellowstone-vixen

COPY . .

WORKDIR /usr/src/yellowstone-vixen/crates/demo

RUN cargo build --release

FROM debian:buster-slim

CMD ["./usr/src/yellowstone-vixen/crates/demo/target/release/demo"]