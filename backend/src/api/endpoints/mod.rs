use rocket::{Build, Rocket};
use rocket_cors::CorsOptions;

pub mod omdb;
pub mod watchlist;

pub fn fuel(rocket: Rocket<Build>) -> Rocket<Build> {
    let cors = CorsOptions::default().to_cors().unwrap();

    let rocket = omdb::fuel(rocket);
    let rocket = watchlist::fuel(rocket);
    rocket.attach(cors)
}
