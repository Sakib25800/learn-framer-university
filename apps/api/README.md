# API Service

The backend service for Framer University.

## Tech

- [Rust](https://www.rust-lang.org/) - Programming language
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [Tokio](https://tokio.rs/) - Async runtime
- [Tower](https://github.com/tower-rs/tower) - Middleware framework

## Infrastructure

### Database
- [Neon](https://neon.tech/) - Serverless Postgres database
- [Cloudflare D1](https://developers.cloudflare.com/d1/) - SQLite database for edge functions

### Storage & CDN
- [Cloudflare R2](https://developers.cloudflare.com/r2/) - Object storage
- [Cloudflare Workers](https://workers.cloudflare.com/) - Edge functions

### Email & Communications
- [Loops](https://loops.so/) - Email communications platform

### Monitoring
- [Sentry](https://sentry.io/) - Error tracking and monitoring
- [Prometheus](https://prometheus.io/) - Metrics collection and monitoring

### Deployment
- [Fly.io](https://fly.io/) - Application hosting

## Development

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- pnpm 8+
- Postgres (for local development)

### Getting Started

1. Install dependencies from the root of the repository:
```bash
# Install Rust tools
cargo install cargo-watch # For development
cargo install sqlx-cli # For database migrations

# Install project dependencies
pnpm install
```

2. Start the development server:
```bash
# Start all apps
turbo dev

# Start only API
turbo dev --filter api
```

3. The API will be available at `http://localhost:3001`

### Project Structure

```
api/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app.rs            # App configuration and setup
│   ├── config/            # Configuration management
│   ├── routes/           # API route definitions
│   ├── models/           # Data models
│   ├── services/         # Business logic
│   ├── metrics/          # Metrics and monitoring
│   └── utils/            # Utility functions
├── crates/               # Internal Rust crates
│   └── framer_university_database/  # Database access layer
├── migrations/           # Database migrations
└── tests/               # Integration tests
```

### Development Commands

```bash
# Run with hot reload
turbo dev --filter api

# Run tests
turbo test --filter api

# Run linter
turbo lint --filter api

# Format code
turbo format --filter api

# Generate OpenAPI documentation
turbo dev --filter api & cargo run --bin gen_openapi
```

## API Documentation

The OpenAPI specification is generated at runtime and can be found at `/api/private/openapi.json`.

## Deployment

Deployed to [Fly.io](https://fly.io/docs/). 