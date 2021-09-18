use rocket_sync_db_pools::{database, diesel};

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);
