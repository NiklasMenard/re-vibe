use diesel::prelude::*;
use domain::{
    models::{NewUser, NewUserRole, User},
    schema::{user_roles, users},
};
use infrastructure::database::connection::establish_connection;

use rocket::http::Status;
use rocket::serde::json::Json;
use shared::request_models::Credentials;

pub fn register_user(credentials: Json<Credentials>) -> Status {
    let connection = &mut establish_connection();

    match diesel::insert_into(users::table)
        .values(&NewUser::from_credentials(
            &credentials.email,
            &credentials.password,
        ))
        .get_result::<User>(connection)
    {
        Ok(new_user) => {
            let new_user_role = NewUserRole {
                user_id: new_user.id,
                role_id: 1,
            };

            diesel::insert_into(user_roles::table)
                .values(&new_user_role)
                .execute(connection)
                .unwrap_or_default();

            Status::Accepted
        }
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
