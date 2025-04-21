use axum::extract::{FromRequestParts, State};
use derive_more::Deref;
use lfu_database::PgDbClient;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;

use crate::config::{self};
use crate::email::Emails;
use crate::metrics::{InstanceMetrics, ServiceMetrics};

pub struct App {
    /// Database client
    pub db: PgDbClient,

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
    pub async fn build(config: config::Server, emails: Emails) -> App {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(config.connection_timeout_seconds))
            .connect(&config.database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!().run(&pool).await.unwrap();

        let instance_metrics = InstanceMetrics::new().expect("Failed to initialise metrics");
        let service_metrics = ServiceMetrics::new().expect("Failed to intialise service metrics");

        App {
            instance_metrics,
            service_metrics,
            emails,
            db: PgDbClient::new(pool),
            config: Arc::new(config),
        }
    }

    pub fn db(&self) -> &PgDbClient {
        &self.db
    }
}

#[derive(Clone, FromRequestParts, Deref)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);
