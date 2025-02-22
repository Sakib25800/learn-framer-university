# Architecture of learn.framer.university

## Documentation

Documentation about the codebase appears in these locations:

- `README.md` - Important information to show on the github front page.
- `docs/` - Long-form documentation.

## API - Rust

The API  is written in Rust. Most of that code lives in the _src_ directory. It
serves a JSON API over HTTP, and the HTTP server interface is provided by the [axum](https://crates.io/crates/axum) crate and
related crates. More information about the API is in
[`docs/API.md`](https://github.com/sakib25800/learn-framer-university/blob/main/docs/API.md).

These files and directories have to do with the backend:

- `Cargo.lock` - Locks dependencies to specific versions providing consistency across development
  and deployment
- `Cargo.toml` - Defines the crate and its dependencies
- `migrations/` - Diesel migrations applied to the database during development and deployment
- `src/` - The API's source code
- `target/` - Compiled output, including dependencies and final binary artifacts - (ignored in
  `.gitignore`)

The API stores information in a Postgres database.

## Frontend - Next.js

The frontend of is written in Typescript using [Next.js][https://nextjs.org]. More information about the
frontend is in [`docs/FRONTEND.md`](https://github.com/sakib25800/learn-framer-university/blob/main/docs/FRONTEND.md).

These files have to do with the frontend:

- `app/` - The frontend's source code
- `dist/` - Contains the distributable (optimized and self-contained) output of building the
  frontend; served under the root `/` url - (ignored in `.gitignore`)
- `node_modules/` - node dependencies - (ignored in `.gitignore`)
- `pnpm-lock.yaml` - Locks dependencies to specific versions providing consistency across
  development and deployment
- `public/` - Static files that are merged into `dist/` during build

## Shared

- `shared/` - Shared code between the frontend and backend

If changes are made to the API, then run the following to ensure type safety.

To generate the `shared/api/openapi.json` file, run the following command:

```
cargo run --bin gen_openapi
```

To generate the `shared/api/v1.d.ts` file for API type definitions, run the following command:

```
npx openapi-typescript ./shared/api/openapi.json -o ./shared/api/v1.d.ts
```

## Deployment - Fly.io

learn.framer.university is deployed on [Fly](https://fly.io/).

- `fly.api.toml` - Fly config for production API
- `fly.staging.api.toml` - Fly config for the staging API

- `fly.frontend.toml` - Fly config for production frontend
- `fly.staging.frontend.toml` - Fly config for staging frontend

- `api.Dockerfile` - Dockerfile config for production API
- `api.staging.Dockerfile` - Dockerfile config for the staging API

- `frontend.Dockerfile` - Dockerfile config for production frontend
- `frontend.staging.Dockerfile` - Dockerfile config for staging frontend

## Development

These files are mostly only relevant when running in development.

- `.env` - Environment variables loaded by the backend - (ignored in `.gitignore`)
- `.env.sample` - Example environment file checked into the repository
- `.git/` - The git repository; not available in all deployments (e.g. Fly)
- `.gitignore` - Configures git to ignore certain files and folders
- `.github/workflows/*` - Configuration for continuous integration at [GitHub Actions](https://github.com/rust-lang/crates.io/actions)
