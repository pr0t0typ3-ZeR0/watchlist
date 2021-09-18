use anyhow::Result;

pub async fn find(id: String) -> Result<omdb::Movie> {
    let movie = omdb::imdb_id(id)
        .apikey(dotenv!("OMDB_APIKEY"))
        .get()
        .await?;
    Ok(movie)
}

pub async fn find_by_title(title: String) -> Result<omdb::Movie> {
    let movie = omdb::title(title)
        .apikey(dotenv!("OMDB_APIKEY"))
        .get()
        .await?;
    Ok(movie)
}

pub async fn search(title: String) -> Result<Vec<omdb::SearchResultsMovie>> {
    let movies = omdb::search(title)
        .apikey(dotenv!("OMDB_APIKEY"))
        .get()
        .await?;
    Ok(movies.results)
}
