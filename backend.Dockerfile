FROM rust:1-bookworm as builder

ARG BUILD_PROFILE=release
ENV BUILD_PROFILE=${BUILD_PROFILE}

WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --profile ${BUILD_PROFILE} && mv ./target/${BUILD_PROFILE}/server ./server

FROM debian:bookworm-slim

ARG INSTALL_EXTRA_CERTS=false
RUN apt-get update \
    && apt-get install -y postgresql $([ "$INSTALL_EXTRA_CERTS" = "true" ] && echo "ca-certificates") \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /usr/src/app/server /app/server

ARG RUST_LOG_LEVEL
ENV RUST_LOG=${RUST_LOG_LEVEL}

CMD ./server