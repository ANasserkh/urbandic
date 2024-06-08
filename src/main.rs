#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_db_pools;

use dotenvy::dotenv;

mod jwt;
mod database;
mod user;
mod response;
#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![])
        .attach(user::handlers::stage())
}
