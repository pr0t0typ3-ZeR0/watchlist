use anyhow::Error;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use crate::crud::movies;
use crate::db::DbConn;
use crate::models::movies::Movie;
use crate::omdb_helpers;
use crate::schemas::movies::MovieCreate;

#[get("/")]
async fn all(db: DbConn) -> Result<Json<Vec<Movie>>, Debug<Error>> {
    let movies = db.run(|c| movies::all(c)).await?;
    Ok(Json(movies))
}

#[delete("/")]
async fn clear(db: DbConn) -> Result<Json<Vec<Movie>>, Debug<Error>> {
    let movies = db.run(|c| movies::all(c)).await?;
    Ok(Json(movies))
}

#[post("/<id>")]
async fn add(db: DbConn, id: String) -> Result<Json<Movie>, Debug<Error>> {
    let movie = omdb_helpers::find(id).await?;
    let movie = db
        .run(|c| movies::add(c, MovieCreate::from_omdb_movie(movie)))
        .await?;
    Ok(Json(movie))
}

#[get("/<id>")]
async fn find(db: DbConn, id: String) -> Result<Json<Movie>, Debug<Error>> {
    let movie = db.run(|c| movies::find(c, id)).await?;
    Ok(Json(movie))
}

#[delete("/<id>")]
async fn delete(db: DbConn, id: String) -> Result<Json<Movie>, Debug<Error>> {
    let movie = db.run(|c| movies::delete(c, id)).await?;
    Ok(Json(movie))
}

#[post("/<title>")]
async fn add_by_title(db: DbConn, title: String) -> Result<Json<Movie>, Debug<Error>> {
    let movie = omdb_helpers::find_by_title(title).await?;
    let movie = db
        .run(|c| movies::add(c, MovieCreate::from_omdb_movie(movie)))
        .await?;
    Ok(Json(movie))
}

#[get("/<title>")]
async fn find_by_title(db: DbConn, title: String) -> Result<Json<Vec<Movie>>, Debug<Error>> {
    let movies = db.run(|c| movies::find_by_title(c, title)).await?;
    Ok(Json(movies))
}

#[delete("/<title>")]
async fn delete_by_title(db: DbConn, title: String) -> Result<Json<Vec<Movie>>, Debug<Error>> {
    let movies = db.run(|c| movies::delete_by_title(c, title)).await?;
    Ok(Json(movies))
}

pub fn fuel(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/api/watchlist", routes![all, clear])
        .mount("/api/watchlist/id", routes![add, find, delete])
        .mount(
            "/api/watchlist/title",
            routes![add_by_title, find_by_title, delete_by_title],
        )
}
