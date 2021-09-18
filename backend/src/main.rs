#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate dotenv_codegen;

pub mod api;
pub mod crud;
pub mod db;
pub mod diesel_schema;
pub mod models;
pub mod omdb_helpers;
pub mod schemas;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();

    let rocket = rocket::build().attach(db::DbConn::fairing());
    let rocket = api::endpoints::fuel(rocket);
    let rocket = api::catchers::fuel(rocket);
    rocket.ignite().await?.launch().await
}
