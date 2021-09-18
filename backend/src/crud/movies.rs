use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error::NotFound;

use crate::diesel_schema::movies;
use crate::models::movies::Movie;
use crate::schemas::movies::{MovieCreate, MovieUpdate};

pub fn add(db: &PgConnection, movie_in: MovieCreate) -> Result<Movie> {
    let mut new_movie = Movie {
        id: movie_in.id,
        title: movie_in.title,
        genre: movie_in.genre,
        plot: movie_in.plot,
        poster: movie_in.poster,
        runtime: movie_in.runtime,
    };
    new_movie = diesel::insert_into(movies::table)
        .values(&new_movie)
        .get_result(db)?;
    Ok(new_movie)
}

pub fn all(db: &PgConnection) -> Result<Vec<Movie>> {
    let movies = movies::table.get_results(db)?;
    Ok(movies)
}

pub fn clear(db: &PgConnection) -> Result<Vec<Movie>> {
    let movies = diesel::delete(movies::table).get_results(db)?;
    Ok(movies)
}

pub fn find(db: &PgConnection, id_in: String) -> Result<Movie> {
    let movie = movies::table.find(id_in).get_result(db)?;
    Ok(movie)
}

pub fn find_by_title(db: &PgConnection, title_in: String) -> Result<Vec<Movie>> {
    use crate::diesel_schema::movies::dsl::title;
    let movies = movies::table
        .filter(title.eq(title_in.as_str()))
        .get_results(db)?;
    if movies.is_empty() {
        Err(anyhow::Error::new(NotFound))
    } else {
        Ok(movies)
    }
}

pub fn update(db: &PgConnection, movie_in: &MovieUpdate) -> Result<Movie> {
    let updated_movie = diesel::update(movie_in).set(movie_in).get_result(db)?;
    Ok(updated_movie)
}

pub fn delete(db: &PgConnection, id_in: String) -> Result<Movie> {
    let deleted_movie = diesel::delete(movies::table.find(id_in)).get_result(db)?;
    Ok(deleted_movie)
}

pub fn delete_by_title(db: &PgConnection, title_in: String) -> Result<Vec<Movie>> {
    use crate::diesel_schema::movies::dsl::title;
    let deleted_movies =
        diesel::delete(movies::table.filter(title.eq(title_in))).get_results(db)?;
    if deleted_movies.is_empty() {
        Err(anyhow::Error::new(NotFound))
    } else {
        Ok(deleted_movies)
    }
}
