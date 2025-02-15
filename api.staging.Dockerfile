FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .

# Environment variables for optimization
ENV RUSTFLAGS="-C target-cpu=native -C opt-level=2"
ENV CARGO_PROFILE_DEV_OPT_LEVEL=2

# Build with development profile but with some optimizations
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build && mv ./target/debug/learn-framer-university ./learn-framer-university

# Runtime image
FROM debian:bookworm-slim

# Install only necessary dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends postgresql \
    && rm -rf /var/lib/apt/lists/*

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/learn-framer-university /app/learn-framer-university

# Set environment variable to indicate staging
ENV APP_ENV=staging
ENV RUST_LOG=info

# Run the app
CMD ./learn-framer-university
