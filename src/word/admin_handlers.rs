use crate::{database::Db, jwt::Claims};

use rocket::{
    fairing::AdHoc,
    response::{
        status::{NoContent, Unauthorized},
        Debug,
    },
    serde::{json::Json, Deserialize, Serialize},
};

use rocket_db_pools::diesel::prelude::*;

use rocket_db_pools::{diesel, Connection};

use super::models::{words, Word};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/unapproved?<page_index>&<limit>")]
async fn get_unapproved(
    mut db: Connection<Db>,
    claims: Claims,
    page_index: i64,
    limit: i64,
) -> Result<Json<Vec<Word>>, Unauthorized<String>> {
    if claims.is_admin == false {
        return Err(Unauthorized("you don't have permissions".to_string()));
    }
    let words = words::table
        .select(words::all_columns)
        .filter(words::approved.eq(false))
        .limit(limit)
        .offset(page_index * 10)
        .load(&mut db)
        .await
        .unwrap();
    Ok(Json(words))
}

#[delete("/admin/<id>")]
async fn delete(
    mut db: Connection<Db>,
    id: i32,
    claims: Claims,
) -> Result<NoContent, Unauthorized<String>> {
    if claims.is_admin == false {
        return Err(Unauthorized("you don't have permissions".to_string()));
    }
    let _ = diesel::delete(
        words::table
            .filter(words::id.eq(id))
            .filter(words::created_by.eq(claims.id)),
    )
    .execute(&mut db)
    .await;

    Ok(NoContent)
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct UpdateWordStatus {
    id: i32,
    approved: bool,
}

#[put("/update_status", data = "<update_status>")]
async fn update_status(
    mut db: Connection<Db>,
    claims: Claims,
    update_status: Json<UpdateWordStatus>,
) -> Result<NoContent, Unauthorized<String>> {
    if claims.is_admin == false {
        return Err(Unauthorized("you don't have permissions".to_string()));
    }
    let _ = diesel::update(words::table)
        .filter(words::id.eq(&update_status.id))
        .set(words::approved.eq(update_status.approved))
        .execute(&mut db)
        .await;

    Ok(NoContent)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("admin words endpoint", |rocket| async {
        rocket.mount("/words", routes![get_unapproved, delete, update_status])
    })
}
