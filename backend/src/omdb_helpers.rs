pub async fn find(id: String) -> Result<omdb::Movie, omdb::Error> {
    omdb::imdb_id(id).apikey(dotenv!("OMDB_APIKEY")).get().await
}

pub async fn find_by_title(title: String) -> Result<omdb::Movie, omdb::Error> {
    omdb::title(title)
        .apikey(dotenv!("OMDB_APIKEY"))
        .get()
        .await
}

pub async fn search(title: String) -> Result<Vec<omdb::SearchResultsMovie>, omdb::Error> {
    let movies = omdb::search(title)
        .apikey(dotenv!("OMDB_APIKEY"))
        .get()
        .await?;
    Ok(movies.results)
}
