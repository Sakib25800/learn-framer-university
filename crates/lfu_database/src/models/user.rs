use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::DbResult;

#[derive(Debug, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Users {
    pool: PgPool,
}

impl Users {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, email: &str, role: UserRole) -> DbResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            INSERT INTO users (email, role)
            VALUES ($1, $2)
            RETURNING
                id,
                email,
                email_verified,
                image,
                role AS "role: UserRole",
                created_at,
                updated_at
            "#,
            email,
            role as _
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find(&self, id: Uuid) -> DbResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            SELECT
                id,
                email,
                email_verified,
                image,
                role AS "role: UserRole",
                created_at,
                updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_email(&self, email_str: &str) -> DbResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            SELECT
                id,
                email,
                email_verified,
                image,
                role AS "role: UserRole",
                created_at,
                updated_at
            FROM users
            WHERE email = $1
            "#,
            email_str
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn verify_email(&self, id: Uuid) -> DbResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            UPDATE users
            SET email_verified = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING
                id,
                email,
                email_verified,
                image,
                role AS "role: UserRole",
                created_at,
                updated_at
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn count(&self) -> DbResult<Option<i64>> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM users
            "#
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(count)
    }
}
