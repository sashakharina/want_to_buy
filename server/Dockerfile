FROM rust:1.67-slim-bullseye as builder

RUN mkdir -p /server
WORKDIR /server

# Setup environment
RUN rustup component add rustfmt

# Copy dependencies
COPY ./server/migrations ./migrations
COPY ./server/src ./src
COPY ./server/config.toml ./config.toml
COPY ./server/Cargo.toml ./Cargo.toml
COPY ./server/Cargo.lock ./Cargo.lock

# Build
RUN cargo build

# Copy binary to a clean image
FROM debian:bullseye-slim as dev

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /server/target/debug/server /usr/local/bin/server
COPY ./server/config.toml /etc/wtb/config.toml

CMD server