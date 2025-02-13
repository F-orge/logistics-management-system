# syntax=docker/dockerfile:1

FROM lukemathwalker/cargo-chef:latest-rust-1 as chef

WORKDIR /app

RUN apt update && \
  apt install protobuf-compiler -y

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

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin backend

# ----------------------------

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=rust-builder /app/target/release/backend ./

RUN apt-get update && apt install -y openssl

ENV RUST_PORT=8000
ENV RUST_ADDRESS=0.0.0.0
ENV RUST_STORAGE_DIRECTORY_URL=./storage
ENV RUST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
ENV RUST_JWT_SECRET=lPPZmlMsRRkCxwWE7jusVfHiaFfGEL6iSU6/h+TD4D8=

CMD [ "./backend" ]