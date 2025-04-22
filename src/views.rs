use chrono::{DateTime, Utc};
use lfu_database::models::user::UserRole;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AuthenticatedUser {
    /// Unique identifier for the user.
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,

    /// Email address of the user.
    #[schema(example = "user@example.com")]
    pub email: String,

    /// Whether the user's email address has been verified.
    #[schema(example = "2019-12-13T13:46:41Z")]
    pub email_verified: Option<DateTime<Utc>>,

    /// URL of the user's profile image.
    #[schema(example = "https://example.com/image.jpg")]
    pub image: Option<String>,

    /// Role of the user.
    #[schema(example = "admin")]
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct VerifiedEmailResponse {
    /// Access token for the user.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,

    /// Refresh token for the user.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MessageResponse {
    /// Message content.
    #[schema(example = "We've sent you an email!")]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DataResponse<T> {
    pub data: T,
}
