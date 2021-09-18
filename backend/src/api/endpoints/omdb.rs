use anyhow::Error;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{get, Build, Rocket};

use crate::omdb_helpers;

#[get("/id/<id>")]
async fn find(id: String) -> Result<Json<omdb::Movie>, Debug<Error>> {
    let movie = omdb_helpers::find(id).await?;
    Ok(Json(movie))
}

#[get("/title/<title>")]
async fn find_by_title(title: String) -> Result<Json<omdb::Movie>, Debug<Error>> {
    let movie = omdb_helpers::find_by_title(title).await?;
    Ok(Json(movie))
}

#[get("/search/<title>")]
async fn search(title: String) -> Result<Json<Vec<omdb::SearchResultsMovie>>, Debug<Error>> {
    let movies = omdb_helpers::search(title).await?;
    Ok(Json(movies))
}

pub fn fuel(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/api/omdb", routes![find, find_by_title, search])
}
