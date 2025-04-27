#![doc = include_str!("../README.md")]

use models::{refresh_token::RefreshTokens, user::Users, verification_token::VerificationTokens};
use sqlx::PgPool;

pub mod models;

pub type DbResult<T> = Result<T, sqlx::Error>;

#[derive(Clone)]
pub struct PgDbClient {
    pool: PgPool,
    pub users: Users,
    pub refresh_tokens: RefreshTokens,
    pub verification_tokens: VerificationTokens,
}

impl PgDbClient {
    pub fn new(pool: PgPool) -> Self {
        Self {
            users: Users::new(pool.clone()),
            refresh_tokens: RefreshTokens::new(pool.clone()),
            verification_tokens: VerificationTokens::new(pool.clone()),
            pool,
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
