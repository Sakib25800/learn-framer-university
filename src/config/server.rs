use config::{Config, Environment};
use http::HeaderValue;

use crate::Env;

#[derive(serde::Deserialize)]
pub struct Server {
    // Server
    pub env: Env,
    pub allowed_origins: AllowedOrigins,
    pub metrics_authorization_token: Option<String>,
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
    // Other
    pub app_url: String,
    pub domain_name: String,
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
            .set_default("env", env)?
            .set_default("domain_name", "https://learn.framer.university")?;

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
