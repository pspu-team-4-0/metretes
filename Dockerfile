FROM rust:slim as builder
LABEL authors="bulbaman"

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y build-essential musl-tools musl-dev  && rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY .sqlx .sqlx
COPY migrations migrations
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/metretes /usr/local/bin/metretes
CMD ["metretes"]