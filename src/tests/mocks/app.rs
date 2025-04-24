use std::net::SocketAddr;
use std::sync::LazyLock;
use std::{rc::Rc, sync::Arc};

use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    middleware::Next,
};
use axum_test::TestServer;
use lfu_database::{models::user::UserRole, PgDbClient};
use regex::Regex;
use sqlx::PgPool;

use crate::{
    auth::{generate_access_token, Tokens},
    App, Emails, Env, Server,
};

use super::{MockAdmin, MockAnonymous, MockUser};
struct TestAppInner {
    app: Arc<App>,
    server: TestServer,
    db: Arc<PgDbClient>,
}

#[derive(Clone)]
pub struct TestApp(Rc<TestAppInner>);

impl TestApp {
    pub fn init() -> TestAppBuilder {
        crate::util::tracing::init_for_test();

        TestAppBuilder {
            config: simple_config(),
        }
    }

    /// Create a new user with a verified email address in the database
    /// and return mock user tokens.
    async fn db_new_user(&self, email: &str, role: UserRole) -> MockUser {
        let user = self.db().users.create(email, role).await.unwrap();
        let user = self.db().users.verify_email(user.id).await.unwrap();

        let config = &self.0.app.config;
        let Server {
            jwt_secret,
            jwt_access_token_expiration_hours,
            jwt_refresh_token_expiration_days,
            ..
        } = config.as_ref();

        let access_token = generate_access_token(
            jwt_secret,
            jwt_access_token_expiration_hours,
            user.id,
            email.to_string(),
        )
        .unwrap();

        let refresh_token = self
            .db()
            .refresh_tokens
            .create(user.id, *jwt_refresh_token_expiration_days)
            .await
            .unwrap();

        MockUser {
            app: self.clone(),
            user,
            tokens: Tokens {
                access_token,
                refresh_token: refresh_token.token,
            },
        }
    }

    async fn new_admin(&self, email: &str) -> MockAdmin {
        let mock_auth_user = self.db_new_user(email, UserRole::Admin).await;

        MockAdmin {
            app: self.clone(),
            user: mock_auth_user.user,
            tokens: mock_auth_user.tokens,
        }
    }

    async fn new_user(&self, email: &str) -> MockUser {
        let mock_auth_user = self.db_new_user(email, UserRole::User).await;

        MockUser {
            app: self.clone(),
            user: mock_auth_user.user,
            tokens: mock_auth_user.tokens,
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

        static EMAIL_CONTINUE_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"/api=\s*/continue/[a-f0-9]+").unwrap());

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

    pub fn as_inner(&self) -> &App {
        &self.0.app
    }

    pub fn db(&self) -> &PgDbClient {
        &self.0.db
    }

    pub fn server(&self) -> &TestServer {
        &self.0.server
    }
}

pub struct TestAppBuilder {
    config: Server,
}

impl TestAppBuilder {
    /// Create a `TestApp` with an anonymous user.
    pub async fn empty(self, pool: PgPool) -> (TestApp, MockAnonymous) {
        let (app, server) = build_app(self.config, pool.clone()).await;

        let test_app_inner = TestAppInner {
            app,
            server,
            db: Arc::new(PgDbClient::new(pool)),
        };
        let test_app = TestApp(Rc::new(test_app_inner));
        let anon = MockAnonymous {
            app: test_app.clone(),
        };

        (test_app, anon)
    }

    pub async fn with_user(self, pool: PgPool) -> (TestApp, MockAnonymous, MockUser) {
        let (app, anon) = self.empty(pool).await;
        let user = app.new_user("foo").await;
        (app, anon, user)
    }

    pub async fn with_admin(self, pool: PgPool) -> (TestApp, MockAnonymous, MockUser, MockAdmin) {
        let (app, anon) = self.empty(pool).await;
        let user = app.new_user("foo").await;
        let admin = app.new_admin("admin").await;
        (app, anon, user, admin)
    }
}

async fn build_app(config: Server, pool: PgPool) -> (Arc<App>, TestServer) {
    let emails = Emails::new_in_memory();
    let app = App::build(config, emails, Some(pool)).await;

    let app = Arc::new(app);
    let router = crate::build_handler(Arc::clone(&app));
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

fn simple_config() -> Server {
    Server {
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
        app_url: "https://learn.framer.university".to_string(),
        database_url: "postgres://postgres:password@localhost:5432/learn_framer_university"
            .to_string(),
    }
}
