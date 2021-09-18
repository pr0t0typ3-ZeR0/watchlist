use crate::diesel_schema::movies;

#[derive(AsChangeset, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "movies"]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub genre: String,
    pub plot: String,
    pub poster: String,
    pub runtime: String,
}
