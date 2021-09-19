use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, Build, Rocket};

use crate::api::endpoints::error_status;
use crate::omdb_helpers;

type Result<T, E = Status> = std::result::Result<T, E>;

#[get("/id/<id>")]
async fn find(id: String) -> Result<Json<omdb::Movie>> {
    let movie = omdb_helpers::find(id).await.map_err(error_status::omdb)?;
    Ok(Json(movie))
}

#[get("/title/<title>")]
async fn find_by_title(title: String) -> Result<Json<omdb::Movie>> {
    let movie = omdb_helpers::find_by_title(title)
        .await
        .map_err(error_status::omdb)?;
    Ok(Json(movie))
}

#[get("/search/<title>")]
async fn search(title: String) -> Result<Json<Vec<omdb::SearchResultsMovie>>> {
    let movies = omdb_helpers::search(title)
        .await
        .map_err(error_status::omdb)?;
    Ok(Json(movies))
}

pub fn fuel(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/api/omdb", routes![find, find_by_title, search])
}
