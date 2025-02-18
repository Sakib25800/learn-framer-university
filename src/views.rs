use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Me {
    pub id: i64,
    pub email: String,
    pub email_verified: Option<NaiveDateTime>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct VerifiedEmailResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DataResponse<T> {
    pub data: T,
}
