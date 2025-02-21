FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/server ./server

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y postgresql \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /usr/src/app/server /app/server

CMD ./server
