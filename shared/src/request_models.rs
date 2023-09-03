use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}
