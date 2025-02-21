FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --profile staging && mv ./target/staging/server ./server

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends postgresql \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /usr/src/app/server /app/server

ENV RUST_LOG=info

CMD ./server
