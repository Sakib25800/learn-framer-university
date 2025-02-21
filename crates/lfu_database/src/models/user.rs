use crate::schema::users;
use bon::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// The model representing a row in the `users` database table.
#[derive(Clone, Debug, Queryable, Identifiable, Selectable)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub is_admin: bool,
    pub last_active_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find(id: i64, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        users::table.find(id).first(conn).await
    }

    pub async fn find_by_email(email_str: &str, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(email_str))
            .first(conn)
            .await
    }

    pub async fn verify_email(id: i64, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(users::email_verified.eq(Some(chrono::Utc::now().naive_utc())))
            .returning(User::as_returning())
            .get_result(conn)
            .await
    }
}

/// Represents a new user to be inserted into the database.
#[derive(Insertable, Debug, Builder)]
#[diesel(table_name = users, check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub email: &'a str,
    #[builder(default = false)]
    pub is_admin: bool,
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a str, is_admin: bool) -> Self {
        NewUser { email, is_admin }
    }

    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(conn)
            .await
    }
}
