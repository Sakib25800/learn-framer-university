use axum::extract::{FromRequestParts, State};
use derive_more::Deref;
use diesel::pg::PgConnection;
use diesel::{Connection, ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use rustls::ClientConfig;
use rustls_platform_verifier::ConfigVerifierExt;
use std::sync::Arc;
use std::time::Duration;

use crate::config::{self};
use crate::email::Emails;
use crate::metrics::{InstanceMetrics, ServiceMetrics};
use crate::Env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct App {
    /// Database connection pool
    pub database: Pool<AsyncPgConnection>,

    /// Server configuration
    pub config: Arc<config::Server>,

    /// Metrics related the service as a whole
    pub service_metrics: ServiceMetrics,

    /// Backend to send emails
    pub emails: Emails,

    /// Metrics related to this specific instance of the service
    pub instance_metrics: InstanceMetrics,
}

impl App {
    pub async fn new(config: config::Server, emails: Emails) -> App {
        let instance_metrics =
            InstanceMetrics::new().expect("could not initialise instance metrics");

        if config.env != Env::Test {
            let mut conn = PgConnection::establish(&config.database_url)
                .expect("failed to connect to database");
            conn.run_pending_migrations(MIGRATIONS)
                .expect("failed to run migrations");
        }

        let mut manager_config = ManagerConfig::default();
        manager_config.custom_setup = Box::new(establish_connection);

        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
            &config.database_url,
            manager_config,
        );

        let pool = Pool::builder()
            .max_size(config.pool_size)
            .connection_timeout(Duration::from_secs(config.connection_timeout_seconds))
            .build(manager)
            .await
            .unwrap();

        App {
            instance_metrics,
            emails,
            database: pool,
            config: Arc::new(config),
            service_metrics: ServiceMetrics::new().expect("could not intialise service metrics"),
        }
    }
}

fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        let rustls_config = ClientConfig::with_platform_verifier();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;

        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    };
    fut.boxed()
}

#[derive(Clone, FromRequestParts, Deref)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);
