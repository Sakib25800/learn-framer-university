use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;

use crate::DbResult;

#[derive(Debug, Clone)]
pub struct VerificationTokenModel {
    pub identifier: String,
    pub token: String,
    pub expires: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct VerificationTokens {
    pool: PgPool,
}

impl VerificationTokens {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        identifier: String,
        expires_in_hours: i64,
    ) -> DbResult<VerificationTokenModel> {
        let expires = Utc::now() + Duration::hours(expires_in_hours);

        let token = sqlx::query_as!(
            VerificationTokenModel,
            r#"
            INSERT INTO verification_tokens (identifier, expires)
            VALUES ($1, $2)
            RETURNING
                identifier,
                token,
                expires,
                created_at,
                updated_at
            "#,
            identifier,
            expires
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn find_by_token(&self, token: &str) -> DbResult<VerificationTokenModel> {
        let token = sqlx::query_as!(
            VerificationTokenModel,
            r#"
            SELECT
                identifier,
                token,
                expires,
                created_at,
                updated_at
            FROM verification_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn delete(&self, identifier: &str, token: &str) -> DbResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM verification_tokens
            WHERE identifier = $1 AND token = $2
            "#,
            identifier,
            token
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete_all(&self, identifier: &str) -> DbResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM verification_tokens
            WHERE identifier = $1
            "#,
            identifier
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
