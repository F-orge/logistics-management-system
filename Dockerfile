# syntax=docker/dockerfile:1

FROM clux/muslrust:stable AS chef

USER root

RUN cargo install cargo-chef
WORKDIR /app

# ----------------------------

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ----------------------------

FROM chef AS rust-builder

WORKDIR /app

COPY --from=planner /app/recipe.json /app/recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

RUN apt update && \
  apt install protobuf-compiler -y

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# ----------------------------

FROM oven/bun:alpine AS bun-frontend-builder

WORKDIR /app

COPY . .

RUN bun install && \
  bun run build

# ----------------------------

FROM alpine:latest AS runtime

RUN addgroup -S docker_user && adduser -S docker_user -G docker_user

WORKDIR /app

COPY --from=bun-frontend-builder /app/target/release/frontend-build /app/frontend-build
COPY --from=rust-builder /app/target/x86_64-unknown-linux-musl/release/etmar-logistics /app/etmar-logistics

USER docker_user

EXPOSE 3000

ENTRYPOINT [ "./etmar-logistics","serve" ]