# Architecture of learn.framer.university

## Documentation

Documentation about the codebase appears in these locations:

- `README.md` - Important information to show on the github front page.
- `docs/` - Long-form documentation.

## Backend - Rust

The backend  is written in Rust. Most of that code lives in the _src_ directory. It
serves a JSON API over HTTP, and the HTTP server interface is provided by the [axum](https://crates.io/crates/axum) crate and
related crates. More information about the backend is in
[`docs/BACKEND.md`](https://github.com/sakib25800/framer-university/blob/main/docs/BACKEND.md).

These files and directories have to do with the backend:

- `Cargo.lock` - Locks dependencies to specific versions providing consistency across development
  and deployment
- `Cargo.toml` - Defines the crate and its dependencies
- `migrations/` - Diesel migrations applied to the database during development and deployment
- `src/` - The backend's source code
- `target/` - Compiled output, including dependencies and final binary artifacts - (ignored in
  `.gitignore`)

The backend stores information in a Postgres database.

## Frontend - Next.js

The frontend of is written in Typescript using [Next.js][https://nextjs.org]. More information about the
frontend is in [`docs/FRONTEND.md`](https://github.com/sakib25800/framer-university/blob/main/docs/FRONTEND.md).

These files have to do with the frontend:

- `app/` - The frontend's source code
- `dist/` - Contains the distributable (optimized and self-contained) output of building the
  frontend; served under the root `/` url - (ignored in `.gitignore`)
- `node_modules/` - npm dependencies - (ignored in `.gitignore`)
- `package.json` - Defines the npm package and its dependencies
- `package-lock.json` - Locks dependencies to specific versions providing consistency across
  development and deployment
- `public/` - Static files that are merged into `dist/` during build

## Deployment - Fly.io

learn.framer.university is deployed on [Fly](https://fly.io/).

These files are Fly-specific; for deployment to Fly.

- `fly.backend.toml` - Fly config for backend
- `fly.frontend.toml` - Fly config for frontend
- `Dockerfile.backend` - Dockerfile config for backend
- `Dockerfile.frontend` - Dockerfile config for frontend

## Development

These files are mostly only relevant when running in development.

- `.env` - Environment variables loaded by the backend - (ignored in `.gitignore`)
- `.env.sample` - Example environment file checked into the repository
- `.git/` - The git repository; not available in all deployments (e.g. Fly)
- `.gitignore` - Configures git to ignore certain files and folders
- `.github/workflows/*` - Configuration for continuous integration at [GitHub Actions](https://github.com/rust-lang/crates.io/actions)
