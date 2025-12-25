
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use domain::schema::users;
use infrastructure::database::connection::DbPool;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub async fn delete_user(pool: &DbPool, id: Uuid) -> Result<(), NotFound<String>> {
    let mut connection = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            let response = Response {
                body: ResponseBody::Message("Database connection error".to_string()),
            };
            return Err(NotFound(serde_json::to_string(&response).unwrap()));
        }
    };

    match diesel::delete(users::table.find(id)).execute(&mut connection).await {
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
