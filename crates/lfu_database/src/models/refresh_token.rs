use crate::schema::refresh_tokens;
use bon::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// The model representing a row in the `refresh_tokens` database table.
#[derive(Clone, Debug, Queryable, Identifiable, Selectable)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expires: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub async fn find(id: i64, conn: &mut AsyncPgConnection) -> QueryResult<RefreshToken> {
        refresh_tokens::table.find(id).first(conn).await
    }
}

/// Represents a new refresh token to be inserted into the database.
#[derive(Insertable, Debug, Builder)]
#[diesel(table_name = refresh_tokens, check_for_backend(diesel::pg::Pg))]
pub struct NewRefreshToken {
    pub user_id: i64,
    pub expires: DateTime<Utc>,
}

impl NewRefreshToken {
    pub fn new(user_id: i64, expires_in_days: i64) -> Self {
        let expires = Utc::now() + chrono::Duration::days(expires_in_days);

        Self { user_id, expires }
    }

    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<RefreshToken> {
        diesel::insert_into(refresh_tokens::table)
            .values(self)
            .returning(RefreshToken::as_returning())
            .get_result(conn)
            .await
    }
}
