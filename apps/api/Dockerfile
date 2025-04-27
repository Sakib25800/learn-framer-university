FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
COPY .sqlx .sqlx

ENV SQLX_OFFLINE=true
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin api

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash app
USER app
WORKDIR /app

COPY --from=builder /app/target/release/api /usr/local/bin/api

ENTRYPOINT ["/usr/local/bin/api"]
