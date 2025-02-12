use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
    pub id: i64,
    pub email: String,
    pub email_verified: Option<NaiveDateTime>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifiedEmailResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
}
