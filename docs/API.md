# API Overview

## Server

The code to run the server is located in _src/main.rs_. This is where the system is pieced together and instantiated, and can be seen as the "entry point" to api.learn.framer.university.

The server does the following:

1. Initialize logging
2. Run pending database migrations using `diesel_migrations`
3. Read values from environment variables to configure a new instance of `framer_university::App`
4. Adds middleware to the app by calling `framer_university::middleware`
5. Starts a [hyper](https://crates.io/crates/hyper) server

## Routes

The API routes are defined in _src/router.rs_.

All of the `api_router` routes are mounted under the `/v1` path.

Each API route definition looks like this:

```rust
let (..) = BaseOpenApi::router()
    // Other routes...
    .routes(routes!(user::me::get_authenticated_user))
    .split_for_parts();
```

## Modules

### `app`

This contains the `App` struct, which holds a `Config` instance plugin other application components, such as:
- Database connection pool
- The `Config` instance
- Service metrics
- Instance metrics

### `config`

This module contains the `Config` struct, which holds values read from environment variables e.g. `allowed_origins`.

See `.env.sample` for an example of what should be in the env file.

### `db`

This module is responsible for managing database connections and migrations.

- **Database Connection Pool**: The connection pool is managed using `deadpool-diesel` and `diesel_async`
- **Database Migrations**: The database migrations are managed using `diesel_migrations`
- **Database Configuration**: The database configuration is managed using `diesel_config` and is read from the environment. The settings include:

  - `database_url`: The URL of the database.
  - `connection_timeout_seconds`: The timeout duration for database connections in seconds.
  - `pool_size`: The size of the database connection pool.

### `model`

This module contains the data models used in the application. These models represent the structure of the data stored in the database and are defined using Diesel ORM.

## Tests

### Integration tests

Integration tests are located in `src/tests` and contain tests from exercising routes and controllers to middlewares and other application components.

The [axum_test](https://docs.rs/axum-test/latest/axum_test/) crate is used to run a mock server.

#### Insta

Insta is a snapshot testing crate that allows you to create and update snapshots of your test outputs.

```
# Review and accept snapshots
cargo insta review

# Update all snapshots without review
cargo insta accept

# Reject pending snapshots
cargo insta reject

# Show snapshot status
cargo insta status
```

Example snapshot test:
```rust
//src/tests/routes/auth/signin.rs
#[tokio::test(flavor = "multi_thread")]
async fn signin_existing_user() {
    let (app, anon, user) = TestApp::init().with_user().await;

    println!("Email: {}", &user.as_model().email);
    let response = anon
        .post("/v1/auth/signin")
        .json(&json!({
            "email": user.as_model().email
        }))
        .await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "message": "We've sent you an email"
    }));

    let emails = app.emails().await;

    assert_eq!(emails.len(), 1);
    assert_snapshot!(app.emails_snapshot().await);
}
```

Example route test:

```rust
// src/tests/routes/me/get.rs

#[tokio::test(flavor = "multi_thread")]
async fn me_failure() {
    let (_, anon, user) = TestApp::init().with_user().await;

    let response = anon.get("/v1/me").expect_failure().await;

    response.assert_status(StatusCode::FORBIDDEN);
    response.assert_json(&json!({
        "errors": [
            {
                "detail": "Missing authorization header"
            }
        ]
    }));
}
```

Example CORS test:
```rust
// src/tests/cors.rs

#[tokio::test(flavor = "multi_thread")]
async fn test_with_matching_origin() {
    let (_, _, cookie) = TestApp::init()
        .with_config(|server| {
            server.allowed_origins = "https://learn.framer.university".parse().unwrap();
        })
        .with_user()
        .await;

    let mut request = cookie.get_request("/v1/me");
    request.header("Origin", "https://learn.framer.university");

    let response = cookie.run::<()>(request).await;
    assert_eq!(response.status(), StatusCode::OK);
}
```

### Running Tests

To run all tests, use the following command:

```sh
cargo test
```
