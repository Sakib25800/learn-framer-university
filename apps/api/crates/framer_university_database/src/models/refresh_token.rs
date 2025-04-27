use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::DbResult;

#[derive(Debug, Clone)]
pub struct RefreshTokenModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct RefreshTokens {
    pool: PgPool,
}

impl RefreshTokens {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_id: Uuid, expires_in_days: i64) -> DbResult<RefreshTokenModel> {
        let expires = Utc::now() + Duration::days(expires_in_days);

        let token = sqlx::query_as!(
            RefreshTokenModel,
            r#"
            INSERT INTO refresh_tokens (user_id, expires)
            VALUES ($1, $2)
            RETURNING
                id,
                user_id,
                token,
                expires,
                created_at,
                updated_at
            "#,
            user_id,
            expires
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }
}
