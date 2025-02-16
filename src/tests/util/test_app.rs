use super::{MockAnonymousUser, MockAuthUser};
use crate::app::App;
use crate::auth::generate_tokens;
use crate::email::Emails;
use crate::{
    build_handler,
    config::{self, Env},
    schema::users,
};
use axum::body::Body;
use axum::extract::{ConnectInfo, Request};
use axum::middleware::Next;
use axum_test::TestServer;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use framer_university_test_db::TestDatabase;
use regex::Regex;
use std::net::SocketAddr;
use std::sync::LazyLock;
use std::{rc::Rc, sync::Arc};

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
    /// This method updates the database directly
    pub async fn db_new_user(&self, username: &str) -> MockAuthUser {
        let mut conn = self.db_conn().await;
        let email = format!("{username}@example.com");

        let user = diesel::insert_into(users::table)
            .values(crate::tests::new_user(username, &email))
            .get_result(&mut conn)
            .await
            .unwrap();

        let config = &self.0.app.config;
        let tokens = generate_tokens(
            &config.jwt_secret,
            config.jwt_access_token_expiration_hours,
            config.jwt_refresh_token_expiration_days,
            &user,
        )
        .unwrap();

        println!("{tokens:?}");

        MockAuthUser {
            app: self.clone(),
            user,
            tokens,
        }
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

        static EMAIL_CONFIRM_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"/confirm/\w+").unwrap());

        static SEPARATOR: &str = "\n----------------------------------------\n\n";

        self.emails()
            .await
            .into_iter()
            .map(|email| {
                let email = EMAIL_HEADER_REGEX.replace_all(&email, "");
                let email = DATE_TIME_REGEX.replace_all(&email, "[0000-00-00T00:00:00Z]");
                let email = EMAIL_CONFIRM_REGEX.replace_all(&email, "/confirm/[confirm-token]");
                email.to_string()
            })
            .collect::<Vec<_>>()
            .join(SEPARATOR)
    }

    /// Obtain a reference to the inner `App` value
    pub fn as_inner(&self) -> &App {
        &self.0.app
    }

    /// Obtain a reference to the test server
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
        let user = app.db_new_user("foo").await;
        (app, anon, user)
    }
}

fn simple_config() -> config::Server {
    config::Server {
        env: Env::Test,
        allowed_origins: Default::default(),
        metrics_authorization_token: None,
        max_blocking_threads: None,
        instance_metrics_log_every_seconds: None,
        jwt_secret: "test_secret".to_string(),
        jwt_access_token_expiration_hours: 1,
        jwt_refresh_token_expiration_days: 7,
        email_verification_expiration_hours: 24,
        connection_timeout_seconds: 1,
        pool_size: 5,
        mailgun_smtp_login: "test_login".to_string(),
        mailgun_smtp_password: "test_password".to_string(),
        mailgun_smtp_server: "test_domain".to_string(),
        domain_name: "test_domain".to_string(),
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
