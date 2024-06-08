use crate::{
    database::Db,
    jwt::{self, Claims},
};
use rocket::{
    fairing::AdHoc,
    response::{
        status::{Created, NotFound},
        Debug,
    },
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::{Connection, Database};

use rocket_db_pools::diesel::prelude::*;

use super::models::{users, NewUser, User};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct UserLoginView {
    email: String,
    password: String,
    token: Option<String>,
}

#[post("/signup", data = "<new_user>")]
async fn singup(mut db: Connection<Db>, new_user: Json<NewUser>) -> Result<Created<Json<User>>> {
    let user = diesel::insert_into(users::table)
        .values(&*new_user)
        .returning(User::as_returning())
        .get_result(&mut db)
        .await?;

    Ok(Created::new("/").body(Json(user)))
}

#[get("/login", data = "<user_view>")]
async fn login<'a>(
    mut db: Connection<Db>,
    mut user_view: Json<UserLoginView>,
) -> Result<Created<Json<UserLoginView>>, NotFound<&'a str>> {
    let user = users::table
        .select(users::all_columns)
        .filter(users::email.eq(&user_view.email))
        .filter(users::password.eq(&user_view.password))
        .first::<User>(&mut db)
        .await;

    match user {
        Ok(user) => match jwt::create_jwt(Claims::new(user.id, user.is_admin)) {
            Ok(token) => {
                user_view.token = Some(token);
                Ok(Created::new("/").body(user_view))
            }
            Err(_) => Err(NotFound("User not Found")),
        },
        Err(_) => Err(NotFound("User not Found")),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("users endpoint", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/users", routes![singup, login])
    })
}
