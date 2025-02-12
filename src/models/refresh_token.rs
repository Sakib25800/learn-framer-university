use crate::schema::refresh_tokens;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset,
)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expires: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl RefreshToken {
    pub async fn find(id: i64, conn: &mut AsyncPgConnection) -> QueryResult<RefreshToken> {
        refresh_tokens::table.find(id).first(conn).await
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRefreshToken {
    pub user_id: i64,
    pub token: String,
    pub expires: NaiveDateTime,
}

impl NewRefreshToken {
    pub fn new(user_id: i64, expires_in_days: i64) -> Self {
        let token = uuid::Uuid::new_v4().to_string();
        let expires = (Utc::now() + chrono::Duration::days(expires_in_days)).naive_utc();

        Self {
            user_id,
            token,
            expires,
        }
    }

    pub async fn create(&self, conn: &mut AsyncPgConnection) -> QueryResult<RefreshToken> {
        diesel::insert_into(refresh_tokens::table)
            .values(self)
            .returning(RefreshToken::as_returning())
            .get_result(conn)
            .await
    }
}
