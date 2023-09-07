use bcrypt::{hash_with_salt, DEFAULT_COST};
use diesel::prelude::*;
use domain::{
    models::{InsertableUser, RegisterableUser, User},
    schema::users,
};
use infrastructure::database::connection::establish_connection;
use rocket::{http::Status, serde::json::Json};
use shared::request_models::Credentials;

pub fn register_user(credentials: Json<Credentials>) -> Status {
    let connection = &mut establish_connection();

    let new_credentials = credentials.into_inner();

    let new_user = RegisterableUser {
        password: new_credentials.password,
        email: new_credentials.email,
    };

    match diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(new_user))
        .get_result::<User>(connection)
    {
        Ok(_) => Status::Accepted,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn register_user_by_email_and_password(email: String, password_: String) -> Option<User> {
    let connection = &mut establish_connection();

    // First query to get user info
    let user_info_result = users::table
        .filter(users::email.eq(&email))
        .first::<User>(connection);

    let user_info = match user_info_result {
        Ok(user) => user,
        Err(_) => return None,
    };

    let decoded_bytes = base64::decode(&user_info.salt).unwrap();
    let mut salt_array: [u8; 16] = [0; 16];

    salt_array.copy_from_slice(&decoded_bytes);

    let hash = hash_with_salt(password_, DEFAULT_COST, salt_array)
        .unwrap()
        .to_string();

    let res = users::table
        .filter(users::email.eq(&email))
        .filter(users::password.eq(&hash))
        .order(users::id)
        .first(connection);
    match res {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}
