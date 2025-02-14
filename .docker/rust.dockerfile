# syntax=docker.io/docker/dockerfile:1.7-labs

FROM lukemathwalker/cargo-chef:latest-rust-1 as chef

WORKDIR /app

RUN apt update && \
  apt install protobuf-compiler -y

# ----------------------------

FROM chef AS planner

COPY --parents cmd/ crates/ services/ .sqlx/ Cargo.toml Cargo.lock ./

RUN cargo chef prepare --recipe-path recipe.json

# ----------------------------

FROM chef AS rust-builder

WORKDIR /app

COPY --from=planner /app/recipe.json /app/recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY --parents cmd/ crates/ services/ .sqlx/ migrations/ Cargo.toml Cargo.lock ./

RUN cargo build --release --bin backend

# ----------------------------

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt install -y openssl

COPY --from=rust-builder /app/target/release/backend ./

ENV RUST_PORT=8000
ENV RUST_ADDRESS=0.0.0.0
ENV RUST_STORAGE_DIRECTORY_URL=./storage
ENV RUST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
ENV RUST_JWT_SECRET=lPPZmlMsRRkCxwWE7jusVfHiaFfGEL6iSU6/h+TD4D8=

CMD [ "./backend" ]