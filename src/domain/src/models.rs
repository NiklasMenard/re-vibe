use bcrypt::{hash_with_salt, DEFAULT_COST};

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{products, user_roles, users};

// Struct for the Users table
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub password: String,
    pub email: String,
    pub salt: String,
    pub bio: Option<String>,
    pub name: Option<String>,
    pub profile_picture_url: Option<String>,
}

impl User {
    pub fn hash_with_salt(password: &str, salt: &str) -> String {
        let decoded_bytes = base64::decode(&salt).unwrap();
        let mut salt_array: [u8; 16] = [0; 16];
        salt_array.copy_from_slice(&decoded_bytes);

        let hash = hash_with_salt(password, DEFAULT_COST, salt_array)
            .unwrap()
            .to_string();

        hash
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    id: Uuid,
    password: String,
    email: String,
    salt: String,
}

impl NewUser {
    fn generate_random_salt() -> [u8; 16] {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];
        rng.fill(&mut bytes);
        bytes
    }

    pub fn from_credentials(email: &str, password: &str) -> NewUser {
        let new_salt = NewUser::generate_random_salt();

        let hash = hash_with_salt(password, DEFAULT_COST, new_salt)
            .unwrap()
            .to_string();

        NewUser {
            id: Uuid::new_v4(),
            email: email.to_string(),
            password: hash,
            salt: base64::encode(&new_salt),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
    pub user_id: Uuid,
    pub role_id: i32,
}

#[derive(Queryable, Debug)]
pub struct Role {
    pub role_id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: i32,
}

// Struct for the Categories table
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Category {
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
}

// Struct for the Products table
#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub quantity: i32,
    pub seller_id: Uuid,
    pub category_id: i32,
    pub creation_date: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub quantity: i32,
    pub seller_id: Uuid,
    pub category_id: i32,
    pub creation_date: NaiveDateTime,
}

// Struct for the Tags table
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub tag_id: i32,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct ProductTag {
    pub product_id: i32,
    pub tag_id: i32,
}
