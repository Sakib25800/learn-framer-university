<div align="center">
  <img src="assets/readme-logo-light.png" alt="Framer University Logo" width="150"/>
</div>

# Framer University

<div align="center">
  <p>Learn everything there is to know about Framer</p>
</div>

<div align="center">
  <a href="#overview">Overview</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="#deployment">Deployment</a>
</div>

## Project Structure

The project is organized as a monorepo using Turborepo:

```
.
├── apps/
│   ├── web/                # Main Next.js web platform
│   ├── admin/              # Vite-based admin dashboard
│   ├── plugin/             # Framer plugin
│   └── api/                # Rust/Axum backend service
├── packages/               # Shared packages
│   ├── api/                # Shared API types
│   ├── ui/                 # Shared UI components
│   ├── config-typescript/   # TypeScript config
│   ├── config-eslint/       # ESLint config
│   └── jest-presets/       # Jest config
├── docs/                   # Documentation
└── turbo.json              # Turborepo configuration
```

## Documentation

- Apps:

  - [Web](./apps/web/README.md)
  - [Admin](./apps/admin/README.md)
  - [Plugin](./apps/plugin/README.md)
  - [API](./apps/api/README.md)

- Packages:
  - [API](./packages/api/README.md)
  - [UI](./packages/ui/README.md)
  - [TypeScript Config](./packages/config-typescript/README.md)
  - [ESLint Config](./packages/config-eslint/README.md)
  - [Jest Presets](./packages/jest-presets/README.md)

## Getting Started

### Prerequisites

- [Node.js 18+](https://nodejs.org/en/download/)
- [pnpm 8+](https://pnpm.io/installation)
- [Turborepo](https://turbo.build/repo/docs/installing)
- [Rust 1.70+](https://www.rust-lang.org/tools/install) (for API development)

### Quick Start

```bash
# Install dependencies
pnpm install

# Start all apps
turbo dev

# Start specific app
turbo dev --filter web
turbo dev --filter admin
turbo dev --filter plugin
turbo dev --filter api

# Run tests
turbo test

turbo e2e

# Lint code
turbo lint
```

## Infrastructure

- [Neon](https://neon.tech) - Serverless Postgres
- [Loops](https://loops.so) - Email service provider
- [Cloudflare](https://cloudflare.com) - Pages + Workers + R2 + D1
- [Fly.io](https://fly.io) - Backend hosting
