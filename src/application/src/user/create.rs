use diesel_async::RunQueryDsl;
use domain::{
    models::{NewUser, NewUserRole, User},
    schema::{user_roles, users},
};
use infrastructure::database::connection::DbPool;

use rocket::http::Status;
use rocket::serde::json::Json;
use shared::request_models::Credentials;

pub async fn register_user(pool: &DbPool, credentials: Json<Credentials>) -> Status {
    let mut connection = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return Status::InternalServerError,
    };

    match diesel::insert_into(users::table)
        .values(&NewUser::from_credentials(
            &credentials.email,
            &credentials.password,
        ))
        .get_result::<User>(&mut connection)
        .await
    {
        Ok(new_user) => {
            let new_user_role = NewUserRole {
                user_id: new_user.id,
                role_id: 1,
            };

            diesel::insert_into(user_roles::table)
                .values(&new_user_role)
                .execute(&mut connection)
                .await
                .unwrap_or_default();

            Status::Accepted
        }
        Err(err) => match err {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _info,
            ) => Status::Conflict,
            _ => {
                panic!("Database error - {err}");
            }
        },
    }
}
