use framer_university_database::models::user::UserModel;
use framer_university_database::PgDbClient;
use http::request::Parts;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::middleware::log_request::RequestLogExt;
use crate::util::errors::{internal, unauthorized, AppResult};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub exp: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_access_token(
    jwt_secret: &str,
    jwt_access_token_expiration_hours: &i64,
    user_id: Uuid,
    user_email: String,
) -> AppResult<String> {
    let expiration =
        OffsetDateTime::now_utc().unix_timestamp() + (jwt_access_token_expiration_hours * 60 * 60);
    let claims = Claims {
        sub: user_id,
        email: user_email,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| internal("Failed to create token"))
}

pub fn validate_token(jwt_secret: &str, token: &str) -> AppResult<TokenData<Claims>> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| unauthorized("Invalid token"))?;

    Ok(token_data)
}

#[derive(Debug)]
pub struct AuthCheck;

impl AuthCheck {
    #[instrument(name = "auth.check", skip_all)]
    pub async fn check(jwt_secret: &str, parts: &Parts, db: &PgDbClient) -> AppResult<UserModel> {
        let auth_header = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or(unauthorized("Invalid or missing authentication"))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(unauthorized("Invalid authorization header format"));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();
        let token_data = validate_token(jwt_secret, token)?;

        let user = db.users.find(token_data.claims.sub).await.map_err(|_| {
            parts
                .request_log()
                .add("cause", "User not found in database");
            internal("User not found")
        })?;

        Ok(user)
    }
}
