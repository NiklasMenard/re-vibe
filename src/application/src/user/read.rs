use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use domain::{models::User, schema::users};
use infrastructure::database::connection::DbPool;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<User, NotFound<String>> {
    let mut connection = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            let response = Response {
                body: ResponseBody::Message("Database connection error".to_string()),
            };
            return Err(NotFound(serde_json::to_string(&response).unwrap()));
        }
    };

    match users::table.find(id).first::<User>(&mut connection).await {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting user with id {id} - {err}"
                    )),
                };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => {
                panic!("Database error - {err}");
            }
        },
    }
}
