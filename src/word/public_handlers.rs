use crate::database::Db;

use rocket::{
    fairing::AdHoc,
    response::{status::NotFound, Debug},
    serde::json::Json,
};

use rocket_db_pools::Connection;

use rocket_db_pools::diesel::prelude::*;

use super::models::{words, Word};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/?<page_index>&<limit>")]
async fn get_all_rows(
    mut db: Connection<Db>,
    page_index: i64,
    limit: i64,
) -> Result<Json<Vec<Word>>> {
    let words = words::table
        .select(words::all_columns)
        .filter(words::approved.eq(true))
        .limit(limit)
        .offset(10 * page_index)
        .load(&mut db)
        .await?;
    Ok(Json(words))
}

#[get("/<id>")]
async fn get_by_id(mut db: Connection<Db>, id: i32) -> Result<Json<Word>, NotFound<String>> {
    let word = words::table
        .find(id)
        .filter(words::approved.eq(true))
        .get_result::<Word>(&mut db)
        .await;
    match word {
        Ok(word) => Ok(Json(word)),
        Err(_) => Err(NotFound("Word not found".to_string())),
    }
}

#[get("/search/<search_word>?<page_index>&<limit>")]
async fn search(
    mut db: Connection<Db>,
    search_word: String,
    page_index: i64,
    limit: i64,
) -> Result<Json<Vec<Word>>> {
    let search = format!("%{}%", search_word);
    let words = words::table
        .select(words::all_columns)
        .filter(words::title.like(search))
        .filter(words::approved.eq(true))
        .limit(limit)
        .offset(page_index * 10)
        .load(&mut db)
        .await?;
    Ok(Json(words))
}

#[get("/get-sorted?<page_index>&<limit>")]
async fn get_sorted(
    mut db: Connection<Db>,
    page_index: i64,
    limit: i64,
) -> Result<Json<Vec<Word>>> {
    let words = words::table
        .select(words::all_columns)
        .limit(limit)
        .offset(page_index * 10)
        .order_by(words::character)
        .load(&mut db)
        .await?;
    Ok(Json(words))
}

#[get("/characters")]
async fn get_characters(mut db: Connection<Db>) -> Result<Json<Vec<String>>> {
    let chars = words::table
        .select(words::character)
        .distinct_on(words::character)
        .load(&mut db)
        .await?;
    Ok(Json(chars))
}

#[get("/characters/<character>?<page_index>&<limit>")]
async fn get_by_characters(
    mut db: Connection<Db>,
    character: String,
    page_index: i64,
    limit: i64,
) -> Result<Json<Vec<Word>>> {
    let chars = words::table
        .select(words::all_columns)
        .filter(words::character.eq(&character))
        .limit(limit)
        .offset(page_index * 10)
        .load(&mut db)
        .await?;
    Ok(Json(chars))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("words endpoint", |rocket| async {
        rocket.mount(
            "/words",
            routes![
                get_all_rows,
                search,
                get_sorted,
                get_by_id,
                get_characters,
                get_by_characters
            ],
        )
    })
}
