use anyhow::Context;
use env_var::env_var;
use sentry::types::Dsn;
use sentry::IntoDsn;

pub struct SentryConfig {
    pub dsn: Option<Dsn>,
    pub environment: Option<String>,
    pub release: Option<String>,
    pub traces_sample_rate: f32,
}

impl SentryConfig {
    pub fn from_environment() -> anyhow::Result<Self> {
        let dsn = env_var!(optional "SENTRY_DSN_API")
            .into_dsn()
            .context("SENTRY_DSN_API is not a valid Sentry DSN value")?;

        let environment = dsn.as_ref().map(|_| env_var!(required "SENTRY_ENV_API"));

        Ok(Self {
            dsn: dsn.clone(),
            environment,
            release: Some(env_var!(optional "FLY_MACHINE_VERSION", default: "0")),
            traces_sample_rate: env_var!(optional "SENTRY_TRACES_SAMPLE_RATE", default: "0.0")
                .parse()
                .unwrap_or(0.0),
        })
    }
}
