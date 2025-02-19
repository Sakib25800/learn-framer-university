use crate::schema::verification_tokens;
use bon::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// The model representing a row in the `verification_tokens` database table.
#[derive(Clone, Debug, Queryable, Identifiable, Selectable)]
#[diesel(primary_key(identifier, token))]
pub struct VerificationToken {
    pub identifier: String,
    pub token: String,
    pub expires: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl VerificationToken {
    pub async fn find_by_token(
        token: &str,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<VerificationToken> {
        verification_tokens::table
            .filter(verification_tokens::token.eq(token))
            .first(conn)
            .await
    }

    pub async fn delete(
        identifier: &str,
        token: &str,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<usize> {
        diesel::delete(verification_tokens::table)
            .filter(verification_tokens::identifier.eq(identifier))
            .filter(verification_tokens::token.eq(token))
            .execute(conn)
            .await
    }

    pub async fn delete_all(identifier: &str, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::delete(verification_tokens::table)
            .filter(verification_tokens::identifier.eq(identifier))
            .execute(conn)
            .await
    }
}

/// Represents a new verification token to be inserted into the database.
#[derive(Insertable, Debug, Builder)]
#[diesel(table_name = verification_tokens, check_for_backend(diesel::pg::Pg))]
pub struct NewVerificationToken {
    pub identifier: String,
    pub expires: chrono::NaiveDateTime,
}

impl NewVerificationToken {
    pub fn new(identifier: String, expires_in_hours: i64) -> Self {
        let expires = Utc::now().naive_utc() + chrono::Duration::hours(expires_in_hours);

        Self {
            identifier,
            expires,
        }
    }

    pub async fn create(&self, conn: &mut AsyncPgConnection) -> QueryResult<VerificationToken> {
        diesel::insert_into(verification_tokens::table)
            .values(self)
            .returning(VerificationToken::as_returning())
            .get_result(conn)
            .await
    }
}
