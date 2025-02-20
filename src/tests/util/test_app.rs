use axum::body::Body;
use axum::extract::{ConnectInfo, Request};
use axum::middleware::Next;
use axum_test::TestServer;
use diesel_async::AsyncPgConnection;
use regex::Regex;
use std::net::SocketAddr;
use std::sync::LazyLock;
use std::{rc::Rc, sync::Arc};

use lfu_database::models::{
    refresh_token::NewRefreshToken,
    user::{NewUser, User},
};
use lfu_test_database::TestDatabase;

use super::{MockAdminUser, MockAnonymousUser, MockAuthUser};
use crate::app::App;
use crate::auth::{generate_access_token, Tokens};
use crate::email::Emails;
use crate::Env;
use crate::{
    build_handler,
    config::{self},
};

struct TestAppInner {
    app: Arc<App>,
    server: TestServer,
    db: TestDatabase,
}

/// A representation of the app and its database transaction
#[derive(Clone)]
pub struct TestApp(Rc<TestAppInner>);

impl TestApp {
    /// Initialise an application
    pub fn init() -> TestAppBuilder {
        crate::util::tracing::init_for_test();

        TestAppBuilder {
            config: simple_config(),
        }
    }

    /// Obtain an async database connection from the database pool
    pub async fn db_conn(&self) -> AsyncPgConnection {
        self.0.db.async_connect().await
    }

    /// Create a new user with a verified email address in the database
    /// (`<username>@example.com`) and return a mock user jwt.
    ///
    /// This method updates the database directly.
    pub async fn db_new_user(&self, email: &str, is_admin: bool) -> MockAuthUser {
        let mut conn = self.db_conn().await;

        let user = NewUser::new(email, is_admin)
            .insert(&mut conn)
            .await
            .unwrap();

        let user = User::verify_email(user.id, &mut conn).await.unwrap();

        let config = &self.0.app.config;
        let config::Server {
            jwt_secret,
            jwt_access_token_expiration_hours,
            jwt_refresh_token_expiration_days,
            ..
        } = config.as_ref();
        let access_token =
            generate_access_token(jwt_secret, jwt_access_token_expiration_hours, &user).unwrap();
        let refresh_token = NewRefreshToken::new(user.id, *jwt_refresh_token_expiration_days)
            .insert(&mut conn)
            .await
            .unwrap();

        MockAuthUser {
            app: self.clone(),
            user,
            tokens: Tokens {
                access_token,
                refresh_token: refresh_token.token,
            },
        }
    }

    /// Create a new admin with a verified email address in the database
    /// (`<username>@example.com`) and return a mock admin jwt.
    ///
    /// This method updates the database directly.
    pub async fn db_new_admin(&self, email: &str) -> MockAdminUser {
        let MockAuthUser { app, user, tokens } = self.db_new_user(email, true).await;

        MockAdminUser { app, user, tokens }
    }

    pub async fn emails(&self) -> Vec<String> {
        let emails = self.as_inner().emails.mails_in_memory().await.unwrap();
        emails.into_iter().map(|(_, email)| email).collect()
    }

    pub async fn emails_snapshot(&self) -> String {
        static EMAIL_HEADER_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(Message-ID|Date): [^\r\n]+\r\n").unwrap());

        static DATE_TIME_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z").unwrap());

        static EMAIL_CONTINUE_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"/ap=?\s*i/continue/[a-f0-9]+").unwrap());

        static SEPARATOR: &str = "\n----------------------------------------\n\n";

        self.emails()
            .await
            .into_iter()
            .map(|email| {
                let email = EMAIL_HEADER_REGEX.replace_all(&email, "");
                let email = DATE_TIME_REGEX.replace_all(&email, "[0000-00-00T00:00:00Z]");
                let email = EMAIL_CONTINUE_REGEX.replace_all(&email, "/api/continue/[token]");
                email.to_string()
            })
            .collect::<Vec<_>>()
            .join(SEPARATOR)
    }

    /// Obtain a reference to the inner `App` value.
    pub fn as_inner(&self) -> &App {
        &self.0.app
    }

    /// Obtain a reference to the test server.
    pub fn server(&self) -> &TestServer {
        &self.0.server
    }
}

pub struct TestAppBuilder {
    pub config: config::Server,
}

impl TestAppBuilder {
    /// Create a `TestApp` with an empty database
    pub async fn empty(mut self) -> (TestApp, MockAnonymousUser) {
        // Run each test inside a fresh database schema, deleted at the end of the test
        // the schema will be cleared up once the app is dropped.
        let test_database = TestDatabase::new();
        self.config.database_url = test_database.url().to_string();

        let (app, test_server) = build_app(self.config);

        let test_app_inner = TestAppInner {
            app,
            server: test_server,
            db: test_database,
        };
        let test_app = TestApp(Rc::new(test_app_inner));
        let anon = MockAnonymousUser {
            app: test_app.clone(),
        };

        (test_app, anon)
    }

    /// Create a `TestApp` with a database including a default user
    pub async fn with_user(self) -> (TestApp, MockAnonymousUser, MockAuthUser) {
        let (app, anon) = self.empty().await;
        let user = app.db_new_user("test@example.com", false).await;
        (app, anon, user)
    }

    /// Create a `TestApp` with a database including a default admin user
    pub async fn with_admin(self) -> (TestApp, MockAuthUser, MockAdminUser) {
        let (app, ..) = self.empty().await;
        let user = app.db_new_user("user", false).await;
        let admin = app.db_new_admin("admin").await;
        (app, user, admin)
    }
}

fn simple_config() -> config::Server {
    config::Server {
        env: Env::Test,
        allowed_origins: Default::default(),
        metrics_authorization_token: None,
        instance_metrics_log_every_seconds: None,
        jwt_secret: "test_secret".to_string(),
        jwt_access_token_expiration_hours: 1,
        jwt_refresh_token_expiration_days: 7,
        email_verification_expiration_hours: 24,
        connection_timeout_seconds: 1,
        pool_size: 5,
        domain_name: "learn.framer.university".to_string(),
        app_url: "http://localhost:3000".to_string(),
        // This value is to be overridden by the
        // `TestAppBuilder::empty()` fn.
        database_url: "empty".to_string(),
    }
}

fn build_app(config: config::Server) -> (Arc<App>, TestServer) {
    // Use in-memory email backend for all tests, allowing tests to analyse emails sent.
    let emails = Emails::new_in_memory();

    let app = Arc::new(App::new(config, emails));

    let router = build_handler(Arc::clone(&app));
    // Manually add socket address to request extensions to prevent
    // real_ip middleware from failing.
    let router = router.layer(axum::middleware::from_fn(
        |request: Request<Body>, next: Next| async {
            let mut request = request;
            let socket_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
            request.extensions_mut().insert(ConnectInfo(socket_addr));
            next.run(request).await
        },
    ));
    let test_server = TestServer::new(router).unwrap();

    (app, test_server)
}
