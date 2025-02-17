use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub email: String,
    pub email_verified: Option<NaiveDateTime>,
    pub image: Option<String>,
    pub is_admin: bool,
    pub last_active_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

    pub async fn update_email_and_verify(
        id: i64,
        new_email: &str,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<User> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set((
                users::email.eq(new_email),
                users::email_verified.eq(Some(chrono::Utc::now().naive_utc())),
            ))
            .returning(User::as_returning())
            .get_result(conn)
            .await
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub email_verified: Option<NaiveDateTime>,
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a str) -> Self {
        NewUser {
            email,
            email_verified: None,
        }
    }

    pub async fn create(&self, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(conn)
            .await
    }
}
