use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = words)]
pub struct Word {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub character: String,
    pub approved: bool,
    pub created_by: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewWordView {
    pub title: String,
    pub description: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = words)]
pub struct NewWord {
    pub title: String,
    pub description: String,
    pub created_by: i32,
}

diesel::table! {
    words (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        #[max_length = 1]
        character -> Char,
        approved -> Bool,
        created_by -> Int4,
    }
}

diesel::joinable!(words -> crate::user::models::users (created_by));
