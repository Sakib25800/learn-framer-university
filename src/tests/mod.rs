pub mod routes;
pub mod util;

use chrono::Utc;

use crate::models::user::NewUser;

fn new_user<'a>(name: &'a str, email: &'a str) -> NewUser<'a> {
    NewUser {
        name,
        email,
        email_verified: Some(Utc::now().naive_utc()),
    }
}
