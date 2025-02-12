//! Application-wide components in a struct accessible from each request

use crate::config::{self, Env};
use crate::metrics::{InstanceMetrics, ServiceMetrics};
use axum::extract::{FromRequestParts, State};
use deadpool_diesel::Runtime;
use derive_more::Deref;
use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_async::pooled_connection::deadpool::Pool as DeadpoolPool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;
use std::time::Duration;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct App {
    /// Database connection pool
    pub database: DeadpoolPool<AsyncPgConnection>,

    /// Server configuration
    pub config: Arc<config::Server>,

    /// Metrics related the service as a whole
    pub service_metrics: ServiceMetrics,

    /// Metrics related to this specific instance of the service
    pub instance_metrics: InstanceMetrics,
}

impl App {
    /// Creates a new `App` with a given `Config`
    ///
    /// Configures and sets up:
    ///
    /// - TODO: Google OAuth
    /// - Database migrations
    /// - Database connection pool
    pub fn new(config: config::Server) -> App {
        let instance_metrics =
            InstanceMetrics::new().expect("could not initialise instance metrics");

        if config.env != Env::Test {
            let mut conn = PgConnection::establish(&config.database_url)
                .expect("Failed to connect to database");
            conn.run_pending_migrations(MIGRATIONS)
                .expect("Failed to run migrations");
        }

        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);
        let pool = DeadpoolPool::builder(manager)
            .runtime(Runtime::Tokio1)
            .max_size(config.pool_size)
            .wait_timeout(Some(Duration::from_secs(config.connection_timeout_seconds)))
            .build()
            .unwrap();

        App {
            instance_metrics,
            database: pool,
            config: Arc::new(config),
            service_metrics: ServiceMetrics::new().expect("could not intialise service metrics"),
        }
    }
}

#[derive(Clone, FromRequestParts, Deref)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);
