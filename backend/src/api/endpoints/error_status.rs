use diesel::result::Error as DbError;
use omdb::Error as OmdbError;
use rocket::http::Status;

pub fn db(error: DbError) -> Status {
    match error {
        DbError::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

pub fn omdb(error: OmdbError) -> Status {
    match error {
        OmdbError::Api(api_err) => {
            if api_err.eq("Movie not found!") {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        }
        _ => Status::InternalServerError,
    }
}
