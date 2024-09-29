use rocket::{http::Status, request::FromParam};
use uuid::Uuid;

use crate::auth::UserApiKey;

#[derive(Debug)]
pub struct ValidUuid {
    pub value: Uuid,
}

#[derive(Debug)]
pub struct Validi32 {
    pub value: i32,
}

impl<'r> FromParam<'r> for ValidUuid {
    type Error = Status;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param)
            .map(|uuid| ValidUuid { value: uuid })
            .map_err(|_| Status::BadRequest)
    }
}

impl<'r> FromParam<'r> for Validi32 {
    type Error = Status;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        param
            .parse::<i32>()
            .map(|id| Validi32 { value: id })
            .map_err(|_| Status::BadRequest)
    }
}

pub fn verify_user_id(user_key: &UserApiKey, param_id: &ValidUuid) -> Result<(), Status> {
    if user_key.0.key != param_id.value.to_string() {
        return Err(Status::Forbidden);
    }
    Ok(())
}
