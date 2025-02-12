use crate::schema::accounts;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i64,
    pub user_id: i64,
    pub auth_method: AuthMethod,
    pub provider: String,
    pub provider_account_id: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::AuthMethodEnum"]
pub enum AuthMethod {
    #[db_rename = "oauth"]
    OAuth,
    #[db_rename = "email"]
    Email,
    #[db_rename = "credentials"]
    Credentials,
    #[db_rename = "oidc"]
    OIDC,
    #[db_rename = "sms"]
    SMS,
}

impl Default for AuthMethod {
    fn default() -> Self {
        AuthMethod::Email
    }
}

impl Account {
    pub async fn find(id: i64, conn: &mut AsyncPgConnection) -> QueryResult<Account> {
        accounts::table.find(id).first(conn).await
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccount<'a> {
    pub user_id: i64,
    pub auth_method: AuthMethod,
    pub provider: &'a str,
    pub provider_account_id: &'a str,
    pub refresh_token: Option<&'a str>,
    pub access_token: Option<&'a str>,
    pub expires_at: Option<i64>,
    pub token_type: Option<&'a str>,
    pub scope: Option<&'a str>,
    pub id_token: Option<&'a str>,
}

impl<'a> NewAccount<'a> {
    pub fn new(
        user_id: i64,
        auth_method: AuthMethod,
        provider: &'a str,
        provider_account_id: &'a str,
        refresh_token: Option<&'a str>,
        access_token: Option<&'a str>,
        expires_at: Option<i64>,
        token_type: Option<&'a str>,
        scope: Option<&'a str>,
        id_token: Option<&'a str>,
    ) -> Self {
        NewAccount {
            user_id,
            auth_method,
            provider,
            provider_account_id,
            refresh_token,
            access_token,
            expires_at,
            token_type,
            scope,
            id_token,
        }
    }

    pub async fn create(&self, conn: &mut AsyncPgConnection) -> QueryResult<Account> {
        diesel::insert_into(accounts::table)
            .values(self)
            .returning(Account::as_returning())
            .get_result(conn)
            .await
    }
}
