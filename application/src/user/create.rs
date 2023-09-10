use diesel::prelude::*;
use domain::{
    models::{InsertableUser, User},
    schema::users,
};
use infrastructure::database::connection::establish_connection;

use rocket::http::Status;
use rocket::serde::json::Json;
use shared::request_models::Credentials;

pub fn register_user(credentials: Json<Credentials>) -> Status {
    let connection = &mut establish_connection();

    match diesel::insert_into(users::table)
        .values(&InsertableUser::from_credentials(
            &credentials.email,
            &credentials.password,
        ))
        .get_result::<User>(connection)
    {
        Ok(_) => Status::Accepted,
        Err(err) => match err {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info,
            ) => {
                print!("{}", info.message());
                return Status::Conflict;
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
