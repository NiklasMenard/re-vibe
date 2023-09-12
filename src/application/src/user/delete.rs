use diesel::prelude::*;
use diesel::QueryDsl;
use domain::schema::users;
use infrastructure::database::connection::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub fn delete_user(id: Uuid) -> Result<(), NotFound<String>> {
    let connection = &mut establish_connection();

    match diesel::delete(users::table.find(id)).execute(connection) {
        Ok(_) => Ok(()),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error deleting user with id {:?} - {:?}",
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
