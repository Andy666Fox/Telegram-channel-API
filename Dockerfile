FROM rust:1.93-slim-bookworm AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y pkg-config libssl-dev

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app 

COPY --from=builder /usr/src/app/target/release/tg_parser_api /usr/local/bin/tg_parser_api

EXPOSE ${PORT}

CMD ["tg_parser_api"]