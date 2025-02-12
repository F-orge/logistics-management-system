# syntax=docker/dockerfile:1

FROM clux/muslrust:stable AS chef

USER root

WORKDIR /app

RUN apt update && \
  apt install protobuf-compiler -y

RUN cargo install cargo-chef

# ----------------------------

FROM chef AS planner

COPY cmd ./cmd

COPY crates ./crates

COPY services ./services

COPY .sqlx ./sqlx

COPY Cargo.toml Cargo.lock ./

RUN cargo chef prepare --recipe-path recipe.json

# ----------------------------

FROM chef AS rust-builder

WORKDIR /app

COPY --from=planner /app/recipe.json /app/recipe.json

COPY cmd crates services .sqlx Cargo.toml Cargo.lock ./

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

RUN cargo build --release --target x86_64-unknown-linux-musl --bin backend

# ----------------------------

FROM rust:alpine AS runtime

RUN addgroup -S tonic-axum && adduser -S tonic-axum -G tonic-axum

WORKDIR /app

COPY --from=rust-builder /app/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin/

USER tonic-axum

ENV RUST_ADDRESS=0.0.0.0
ENV RUST_STORAGE_DIRECTORY_URL=./storage
ENV RUST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
ENV RUST_JWT_SECRET=lPPZmlMsRRkCxwWE7jusVfHiaFfGEL6iSU6/h+TD4D8=

CMD [ "/usr/local/bin/backend" ]