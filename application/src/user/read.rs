use diesel::prelude::*;
use diesel::QueryDsl;
use domain::{models::User, schema::users};
use infrastructure::database::connection::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub fn user_information(id: Uuid) -> Result<User, NotFound<String>> {
    let connection = &mut establish_connection();

    match users::table.find(id).first::<User>(connection) {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting user with id {} - {}",
                        id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
