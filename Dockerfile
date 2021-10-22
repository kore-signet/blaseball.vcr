FROM rust:1.56-slim-buster as builder

RUN apt-get update; apt-get install -y --no-install-recommends llvm-dev libclang-dev clang libssl-dev pkg-config

WORKDIR /usr/src/vcr

COPY vcr_lib ./vcr_lib
COPY player ./player
COPY encoder ./encoder
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock

RUN cargo build --release --features bundle_before

FROM debian:buster-slim

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install wget -y

COPY --from=builder /usr/src/vcr/target/release/player .
USER 1000
CMD ["./player"]
