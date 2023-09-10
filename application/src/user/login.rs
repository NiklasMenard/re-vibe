use diesel::prelude::*;
use domain::{models::User, schema::users};
use infrastructure::database::connection::establish_connection;

pub fn check_email_password(email: String, password: String) -> Option<User> {
    let connection = &mut establish_connection();

    // First query to get user info
    let user_info = users::table
        .filter(users::email.eq(&email))
        .first::<User>(connection)
        .ok()?;

    let hash = User::hash_with_salt(password, user_info.salt);

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
