#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_db_pools;

use database::Db;
use dotenvy::dotenv;
use rocket_db_pools::Database;

mod database;
mod jwt;
mod password_manager;
mod response;
mod user;
mod word;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![])
        .attach(Db::init())
        .attach(user::handlers::stage())
        .attach(word::public_handlers::stage())
        .attach(word::user_handlers::stage())
        .attach(word::admin_handlers::stage())
}
