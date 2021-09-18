use crate::diesel_schema::movies;

#[derive(Serialize, Deserialize)]
pub struct MovieCreate {
    pub id: String,
    pub title: String,
    pub genre: String,
    pub plot: String,
    pub poster: String,
    pub runtime: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Identifiable)]
#[table_name = "movies"]
pub struct MovieUpdate {
    pub id: String,
    pub title: Option<String>,
    pub genre: Option<String>,
    pub plot: Option<String>,
    pub poster: Option<String>,
    pub runtime: Option<String>,
}

impl MovieCreate {
    pub fn from_omdb_movie(movie: omdb::Movie) -> MovieCreate {
        MovieCreate {
            id: movie.imdb_id,
            title: movie.title,
            genre: movie.genre,
            plot: movie.plot,
            poster: movie.poster,
            runtime: movie.runtime,
        }
    }
}
