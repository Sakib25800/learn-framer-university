// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "auth_method_enum"))]
    pub struct AuthMethodEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuthMethodEnum;

    accounts (id) {
        id -> Int8,
        user_id -> Int8,
        auth_method -> AuthMethodEnum,
        provider -> Text,
        provider_account_id -> Text,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        expires_at -> Nullable<Int8>,
        token_type -> Nullable<Text>,
        scope -> Nullable<Text>,
        id_token -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Int8,
        user_id -> Int8,
        token -> Text,
        expires -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        email -> Text,
        email_verified -> Nullable<Timestamptz>,
        name -> Text,
        image -> Nullable<Text>,
        is_admin -> Bool,
        last_active_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    verification_tokens (identifier, token) {
        identifier -> Text,
        token -> Text,
        expires -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(accounts, refresh_tokens, users, verification_tokens,);
