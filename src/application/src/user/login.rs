use diesel::prelude::*;
use domain::{
    models::{Role, User, UserRole},
    schema::{roles, user_roles, users},
};
use infrastructure::database::connection::establish_connection;

pub fn check_email_password(email: String, password: String) -> Option<UserRole> {
    let connection = &mut establish_connection();

    // Use manual join conditions to get user info and associated role
    match users::table
        .filter(users::email.eq(&email))
        .inner_join(
            user_roles::table
                .on(users::id.eq(user_roles::user_id))
                .inner_join(roles::table.on(user_roles::role_id.eq(roles::role_id))),
        )
        .select((users::all_columns, roles::all_columns))
        .first::<(User, Role)>(connection)
    {
        Ok((user, role)) => {
            let hash = User::hash_with_salt(&password, &user.salt);

            if user.password == hash {
                println!("{:?}, {:?}", user, role);

                let new_user_role = UserRole {
                    id: user.id,
                    role: role.name,
                };

                return Some(new_user_role);
            } else {
                return None;
            }
        }

        Err(_) => None,
    }
}
