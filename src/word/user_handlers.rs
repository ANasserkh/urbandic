use crate::{database::Db, jwt::Claims};

use rocket::{
    fairing::AdHoc,
    response::{
        status::{Created, NoContent},
        Debug,
    },
    serde::json::Json,
};

use rocket_db_pools::Connection;

use rocket_db_pools::diesel::prelude::*;

use super::models::{words, NewWord, NewWordView, Word};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/mywords?<page_index>&<limit>")]
async fn get_my_words(
    mut db: Connection<Db>,
    claims: Claims,
    page_index: i64,
    limit: i64,
) -> Result<Json<Vec<Word>>> {
    let words = words::table
        .select(words::all_columns)
        .filter(words::created_by.eq(claims.id))
        .limit(limit)
        .offset(page_index * 10)
        .load(&mut db)
        .await?;
    Ok(Json(words))
}

#[delete("/<id>")]
async fn delete(mut db: Connection<Db>, id: i32, claims: Claims) -> Result<NoContent> {
    diesel::delete(
        words::table
            .filter(words::id.eq(id))
            .filter(words::created_by.eq(claims.id)),
    )
    .execute(&mut db)
    .await?;

    Ok(NoContent)
}

#[post("/", data = "<new_word>")]
async fn create_word(
    mut db: Connection<Db>,
    new_word: Json<NewWordView>,
    claims: Claims,
) -> Result<Created<Json<Word>>> {
    let new_word = NewWord {
        title: new_word.title.clone(),
        description: new_word.description.clone(),
        created_by: claims.id,
    };
    let word = diesel::insert_into(words::table)
        .values(&new_word)
        .returning(Word::as_returning())
        .get_result(&mut db)
        .await?;

    Ok(Created::new("/").body(Json(word)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("user words endpoint", |rocket| async {
        rocket.mount("/words", routes![get_my_words, create_word, delete])
    })
}
