use config::{Config, Environment};
use http::HeaderValue;

#[derive(serde::Deserialize)]
pub struct Server {
    // Server
    pub env: Env,
    pub allowed_origins: AllowedOrigins,
    pub metrics_authorization_token: Option<String>,
    pub max_blocking_threads: Option<usize>,
    pub instance_metrics_log_every_seconds: Option<u64>,
    // Auth
    pub jwt_secret: String,
    pub jwt_access_token_expiration_hours: i64,
    pub jwt_refresh_token_expiration_days: i64,
    pub email_verification_expiration_hours: i64,
    // Database
    pub database_url: String,
    pub connection_timeout_seconds: u64,
    pub pool_size: usize,
}

/// Used for setting different values depending on whether the app is being run in production,
/// in development, or for testing.
///
/// The app's `config.env` value is set to `Production` if the environment variable
/// `FLY_APP_NAME` is set and `Development` otherwise. `config.env` is set to `Test`
/// unconditionally in *src/test/all.rs*.
#[derive(PartialEq, Eq, Clone, Copy, Debug, serde::Deserialize)]
pub enum Env {
    Development,
    Test,
    Production,
}

impl Server {
    pub fn from_environment() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let env = match std::env::var("FLY_APP_NAME") {
            Ok(_) => "Production",
            Err(_) => "Development",
        };

        let builder = Config::builder()
            .add_source(Environment::default())
            .set_default("env", env)?;

        Ok(builder.build()?.try_deserialize()?)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct AllowedOrigins(Vec<String>);

impl TryFrom<String> for AllowedOrigins {
    type Error = std::convert::Infallible;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(
            s.split(',')
                .map(str::trim)
                .map(ToString::to_string)
                .collect(),
        ))
    }
}

impl AllowedOrigins {
    pub fn contains(&self, value: &HeaderValue) -> bool {
        self.0.iter().any(|it| {
            if let Ok(header_str) = value.to_str() {
                it == header_str
            } else {
                false
            }
        })
    }
}
